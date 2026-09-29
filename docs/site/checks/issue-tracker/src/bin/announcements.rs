#[path = "../announcements.rs"]
mod announcements;
use announcements::AnnouncementsApp;
use cot::cli::CliMetadata;
use cot::config::ProjectConfig;
use cot::project::RegisterAppsContext;
use cot::{AppBuilder, Project};

struct HostProject {
    prefix: &'static str,
    message: &'static str,
}
impl Project for HostProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }
    fn config(&self, _: &str) -> cot::Result<ProjectConfig> {
        Ok(ProjectConfig::dev_default())
    }
    fn register_apps(&self, apps: &mut AppBuilder, _: &RegisterAppsContext) {
        apps.register_with_views(
            AnnouncementsApp {
                message: self.message,
            },
            self.prefix,
        );
    }
}
#[cot::main]
fn main() -> impl Project {
    HostProject {
        prefix: "/support",
        message: "Support closes at 16:00 UTC on Friday.",
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use cot::test::Client;
    #[cot::test]
    async fn two_hosts_choose_their_own_mount_and_message() -> cot::Result<()> {
        for (prefix, message) in [
            ("/support", "Support opens at 09:00 UTC."),
            ("/news", "A new guide is available."),
        ] {
            let mut client = Client::new(HostProject { prefix, message }).await;
            let response = client.get(&format!("{prefix}/")).await?;
            assert_eq!(response.status(), cot::http::StatusCode::OK);
            assert_eq!(
                response.into_body().into_bytes().await?.as_ref(),
                message.as_bytes()
            );
            assert_eq!(
                client.get("/").await?.status(),
                cot::http::StatusCode::NOT_FOUND
            );
        }
        Ok(())
    }
}
