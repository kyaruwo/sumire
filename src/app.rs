use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn routes() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("app/assets"))
        .nest_service("/html", ServeDir::new("app/html"))
        .nest_service("/js", ServeDir::new("app/js"))
        .route_service("/", ServeFile::new("app/index.html"))
}
