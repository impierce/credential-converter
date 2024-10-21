pub mod root;
pub mod translate_file;

use axum::{extract::DefaultBodyLimit, routing::get, routing::post, Router};
//use save_file::save_file;
use root::root;
use translate_file::translate_file;

pub fn create_router() -> Router {
    Router::new()
        .route(
            "/translate_file",
            post(translate_file).route_layer(DefaultBodyLimit::max(135476000)),
        )
        .route("/", get(root))
    // .layer(tower_http::trace::TraceLayer::new_for_http())
}
