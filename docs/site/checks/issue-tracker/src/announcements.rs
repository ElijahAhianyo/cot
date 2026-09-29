use cot::App;
use cot::router::method::get;
use cot::router::{Route, Router};

pub struct AnnouncementsApp {
    pub message: &'static str,
}

impl App for AnnouncementsApp {
    fn name(&self) -> &'static str {
        "announcements"
    }
    fn router(&self) -> Router {
        let message = self.message;
        Router::with_urls([Route::with_handler(
            "/",
            get(move || async move { message }),
        )])
    }
}
