mod navigation;

use std::time::Duration;

use cot::cli::CliMetadata;
use cot::config::{ProjectConfig, StaticFilesConfig, StaticFilesPathRewriteMode};
use cot::error::handler::DynErrorPageHandler;
use cot::project::{MiddlewareContext, RegisterAppsContext, RootHandler, RootHandlerBuilder};
use cot::static_files::{StaticFile, StaticFilesMiddleware};
use cot::{App, AppBuilder, Project};
use cot_site::{CotSiteApp, cot_site_handle_error};

// Keep documentation-specific assets with the documentation, not the renderer.
struct ShowcaseAssets;

impl App for ShowcaseAssets {
    fn name(&self) -> &str {
        "showcase_assets"
    }

    fn static_files(&self) -> Vec<StaticFile> {
        cot::static_files!(
            "static/images/community/blog-20260929.jpg",
            "static/images/community/chombogen-20260929.jpg",
            "static/images/community/cot-20260929.jpg",
        )
    }
}

struct CotSiteProject;

impl Project for CotSiteProject {
    fn cli_metadata(&self) -> CliMetadata {
        cot::cli::metadata!()
    }

    fn config(&self, _config_name: &str) -> cot::Result<ProjectConfig> {
        // we don't need to load any config
        Ok(ProjectConfig::builder()
            .static_files(
                StaticFilesConfig::builder()
                    .url("/")
                    .rewrite(StaticFilesPathRewriteMode::QueryParam)
                    .cache_timeout(Duration::from_secs(365 * 24 * 60 * 60))
                    .build(),
            )
            .build())
    }

    fn register_apps(&self, modules: &mut AppBuilder, _app_context: &RegisterAppsContext) {
        modules.register_with_views(CotSiteApp::new(navigation::documentation()), "");
        modules.register(ShowcaseAssets);
    }

    fn middlewares(&self, handler: RootHandlerBuilder, context: &MiddlewareContext) -> RootHandler {
        let handler = handler.middleware(StaticFilesMiddleware::from_context(context));
        #[cfg(debug_assertions)]
        let handler = handler.middleware(cot::middleware::LiveReloadMiddleware::new());
        handler.build()
    }

    fn error_handler(&self) -> DynErrorPageHandler {
        DynErrorPageHandler::new(cot_site_handle_error)
    }
}

#[cot::main]
fn main() -> impl Project {
    CotSiteProject
}
