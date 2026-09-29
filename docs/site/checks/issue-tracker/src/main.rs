mod migrations;
mod models;

use cot::cli::CliMetadata;
use cot::config::{DatabaseConfig, ProjectConfig};
use cot::db::migrations::SyncDynMigration;
use cot::db::{Auto, Database, Model, query};
use cot::error::NotFound;
use cot::form::{Form, FormResult};
use cot::html::Html;
use cot::project::RegisterAppsContext;
use cot::request::Request;
use cot::request::extractors::{Path, RequestForm};
use cot::response::{IntoResponse, Redirect, Response};
use cot::router::method::get;
use cot::router::{Route, Router};
use cot::{App, AppBuilder, Project, Template};
use models::Issue;

#[derive(Debug, Template)]
#[template(
    source = "<!doctype html><html lang=\"en\"><title>Issues</title><h1>Issues</h1><a href=\"/issues/new\">Report an issue</a><ul>{% for issue in issues %}<li><a href=\"/issues/{{ issue.id }}\">{{ issue.title }}</a></li>{% endfor %}</ul></html>",
    ext = "html"
)]
struct IssueList {
    issues: Vec<Issue>,
}

#[derive(Debug, Template)]
#[template(
    source = "<!doctype html><html lang=\"en\"><title>{{ issue.title }}</title><h1>{{ issue.title }}</h1><p>{{ issue.description }}</p><a href=\"/issues/\">All issues</a></html>",
    ext = "html"
)]
struct IssueDetail {
    issue: Issue,
}

#[derive(Debug, Form)]
struct IssueForm {
    #[form(opts(min_length = 1, max_length = 120))]
    title: String,
    #[form(opts(min_length = 1, max_length = 2000))]
    description: String,
}

#[derive(Debug, Template)]
#[template(
    source = "<!doctype html><html lang=\"en\"><title>Report an issue</title><h1>Report an issue</h1><form method=\"post\" action=\"/issues/new\">{{ form }}<button type=\"submit\">Save issue</button></form></html>",
    ext = "html"
)]
struct IssueFormPage {
    form: <IssueForm as Form>::Context,
}

async fn home() -> Redirect {
    Redirect::new("/issues/")
}

async fn list_issues(db: Database) -> cot::Result<Html> {
    let issues = Issue::objects()
        .order_by([<Issue as Model>::Fields::id])
        .all(&db)
        .await?;
    Ok(Html::new(IssueList { issues }.render()?))
}

async fn show_issue(Path(id): Path<i64>, db: Database) -> cot::Result<Html> {
    let issue = query!(Issue, $id == id)
        .get(&db)
        .await?
        .ok_or_else(NotFound::new)?;
    Ok(Html::new(IssueDetail { issue }.render()?))
}

async fn new_issue(mut request: Request) -> cot::Result<Html> {
    let form = IssueForm::build_context(&mut request).await?;
    Ok(Html::new(IssueFormPage { form }.render()?))
}

async fn create_issue(
    db: Database,
    RequestForm(result): RequestForm<IssueForm>,
) -> cot::Result<Response> {
    match result {
        FormResult::Ok(form) => {
            let mut issue = Issue {
                id: Auto::auto(),
                title: form.title,
                description: form.description,
            };
            issue.insert(&db).await?;
            Redirect::new(format!("/issues/{}", issue.id)).into_response()
        }
        FormResult::ValidationError(form) => {
            Html::new(IssueFormPage { form }.render()?).into_response()
        }
    }
}

struct IssuesApp;

impl App for IssuesApp {
    fn name(&self) -> &'static str {
        "issue_tracker"
    }
    fn migrations(&self) -> Vec<Box<SyncDynMigration>> {
        cot::db::migrations::wrap_migrations(migrations::MIGRATIONS)
    }
    fn router(&self) -> Router {
        Router::with_urls([
            Route::with_handler("/", get(home)),
            Route::with_handler("/issues/", get(list_issues)),
            Route::with_handler("/issues/new", get(new_issue).post(create_issue)),
            Route::with_handler("/issues/{id}", get(show_issue)),
        ])
    }
}

struct IssueProject {
    database_url: String,
}

impl Project for IssueProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }
    fn config(&self, _name: &str) -> cot::Result<ProjectConfig> {
        let mut config = ProjectConfig::dev_default();
        config.database = DatabaseConfig::builder()
            .url(self.database_url.clone())
            .build();
        Ok(config)
    }
    fn register_apps(&self, apps: &mut AppBuilder, _: &RegisterAppsContext) {
        apps.register_with_views(IssuesApp, "");
    }
}

#[cot::main]
fn main() -> impl Project {
    IssueProject {
        database_url: "sqlite://issue-tracker.sqlite3?mode=rwc".into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cot::http::{StatusCode, header};
    use cot::test::{Client, TestRequestBuilder};

    async fn test_client() -> cot::Result<(tempfile::TempDir, Client)> {
        let directory = tempfile::tempdir().expect("create isolated test directory");
        let url = format!(
            "sqlite://{}?mode=rwc",
            directory.path().join("test.sqlite3").display()
        );
        let db = Database::new(url.clone()).await?;
        cot::db::migrations::MigrationEngine::new(IssuesApp.migrations())?
            .run(&db)
            .await?;
        db.close().await?;
        let client = Client::new(IssueProject { database_url: url }).await;
        Ok((directory, client))
    }

    #[cot::test]
    async fn create_read_and_reject_invalid_input() -> cot::Result<()> {
        let (_directory, mut client) = test_client().await?;
        let response = client
            .request(
                TestRequestBuilder::post("/issues/new")
                    .form_data(&[
                        ("title", "The sign-in button is hard to find"),
                        ("description", "It disappears below the fold on a phone."),
                    ])
                    .build(),
            )
            .await?;
        assert_eq!(response.status(), StatusCode::SEE_OTHER);
        let location = response.headers()[header::LOCATION]
            .to_str()
            .unwrap()
            .to_owned();
        let detail = client.get(&location).await?;
        assert_eq!(detail.status(), StatusCode::OK);
        let body = detail.into_body().into_bytes().await?;
        assert!(String::from_utf8_lossy(&body).contains("below the fold"));
        let invalid = client
            .request(
                TestRequestBuilder::post("/issues/new")
                    .form_data(&[("title", ""), ("description", "Keep this description")])
                    .build(),
            )
            .await?;
        assert_eq!(invalid.status(), StatusCode::OK);
        let body = invalid.into_body().into_bytes().await?;
        assert!(String::from_utf8_lossy(&body).contains("Keep this description"));
        let list = client
            .get("/issues/")
            .await?
            .into_body()
            .into_bytes()
            .await?;
        assert!(!String::from_utf8_lossy(&list).contains("Keep this description"));
        Ok(())
    }

    #[cot::test]
    async fn routing_errors_remain_distinct() -> cot::Result<()> {
        let (_directory, mut client) = test_client().await?;
        assert_eq!(
            client.get("/issues/999999").await?.status(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            client.get("/issues/not-a-number").await?.status(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(client.get("/issues/new").await?.status(), StatusCode::OK);
        assert_eq!(
            client
                .request(TestRequestBuilder::post("/issues/999999").build())
                .await?
                .status(),
            StatusCode::METHOD_NOT_ALLOWED
        );
        Ok(())
    }
}
