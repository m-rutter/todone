use axum::Router;
use sqlx::PgPool;
use tower_http::trace::TraceLayer;

pub mod login;
pub mod todos;

pub fn app(pool: PgPool) -> Router {
    Router::new()
        .merge(login::router())
        .merge(todos::router())
        .layer(TraceLayer::new_for_http())
        .with_state(pool)
}
