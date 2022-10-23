use axum::{Extension, Router};
use sqlx::PgPool;

mod login;
mod todos;

pub fn app(db: PgPool) -> Router {
    Router::new()
        .merge(login::router())
        .merge(todos::router())
        .layer(Extension(db))
}
