use cot::cli::CliMetadata;
use cot::config::ProjectConfig;
use cot::http::StatusCode;
use cot::json::Json;
use cot::project::RegisterAppsContext;
use cot::response::{IntoResponse, Response};
use cot::router::method::post;
use cot::router::{Route, Router};
use cot::{App, AppBuilder, Project};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
struct QuoteRequest {
    quantity: u16,
}
#[derive(Serialize)]
struct Quote {
    total_cents: u64,
}
#[derive(Serialize)]
struct QuoteError {
    code: &'static str,
    message: &'static str,
}

async fn quote(Json(input): Json<QuoteRequest>) -> cot::Result<Response> {
    if !(1..=20).contains(&input.quantity) {
        let mut response = Json(QuoteError {
            code: "invalid_quantity",
            message: "Choose between 1 and 20 items.",
        })
        .into_response()?;
        *response.status_mut() = StatusCode::UNPROCESSABLE_ENTITY;
        return Ok(response);
    }
    Json(Quote {
        total_cents: 450 * u64::from(input.quantity),
    })
    .into_response()
}

struct QuoteApp;
impl App for QuoteApp {
    fn name(&self) -> &'static str {
        "quotes"
    }
    fn router(&self) -> Router {
        Router::with_urls([Route::with_handler("/quotes", post(quote))])
    }
}
struct QuoteProject;
impl Project for QuoteProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }
    fn config(&self, _: &str) -> cot::Result<ProjectConfig> {
        Ok(ProjectConfig::dev_default())
    }
    fn register_apps(&self, apps: &mut AppBuilder, _: &RegisterAppsContext) {
        apps.register_with_views(QuoteApp, "");
    }
}
#[cot::main]
fn main() -> impl Project {
    QuoteProject
}

#[cfg(test)]
mod tests {
    use super::*;
    use cot::test::{Client, TestRequestBuilder};
    #[cot::test]
    async fn quote_contract() -> cot::Result<()> {
        let mut client = Client::new(QuoteProject).await;
        let response = client
            .request(
                TestRequestBuilder::post("/quotes")
                    .json(&QuoteRequest { quantity: 3 })
                    .build(),
            )
            .await?;
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.headers()["content-type"], "application/json");
        assert_eq!(
            response.into_body().into_bytes().await?.as_ref(),
            br#"{"total_cents":1350}"#
        );
        let response = client
            .request(
                TestRequestBuilder::post("/quotes")
                    .json(&QuoteRequest { quantity: 0 })
                    .build(),
            )
            .await?;
        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        let body = response.into_body().into_bytes().await?;
        assert!(String::from_utf8_lossy(&body).contains("invalid_quantity"));
        assert_eq!(
            client.get("/quotes").await?.status(),
            StatusCode::METHOD_NOT_ALLOWED
        );
        Ok(())
    }
}
