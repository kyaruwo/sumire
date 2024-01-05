use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

pub fn routes() -> Router {
    Router::new()
        .nest_service("/assets", ServeDir::new("app/assets"))
        .nest_service("/js", ServeDir::new("app/js"))
        .nest_service("/", html())
}

fn html() -> Router {
    Router::new()
        .route_service("/", ServeFile::new("app/html/landing.html"))
        .nest_service("/auth", ServeFile::new("app/html/auth.html"))
        .nest_service("/main", ServeFile::new("app/html/main.html"))
}
