use axum::{Extension, Router};
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

pub mod login;
pub mod todos;

pub fn app(db: PgPool) -> Router {
    Router::new()
        .merge(login::router())
        .merge(todos::router())
        .layer(Extension(db))
        .layer(TraceLayer::new_for_http())
}
