mod favorite_languages;

use axum::{routing::get, Router};
use favorite_languages::favorite_languages;
use tower_http::trace::TraceLayer;

pub async fn create_router(router: Router) -> Router {
    router
        .route("/", get(favorite_languages))
        .layer(TraceLayer::new_for_http())
}
