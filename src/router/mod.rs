mod favorite_languages;

use axum::{routing::get, Router};
use favorite_languages::favorite_languages;

pub async fn create_router(router: Router) -> Router {
    router.route("/", get(favorite_languages))
}
