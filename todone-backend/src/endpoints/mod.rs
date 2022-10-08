use axum::{Extension, Router};
use sqlx::PgPool;

mod login;

pub fn app(db: PgPool) -> Router {
    Router::new().merge(login::router()).layer(Extension(db))
}
