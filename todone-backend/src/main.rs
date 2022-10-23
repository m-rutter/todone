use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;

use todone_backend::{config::CONFIG, endpoints};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init();

    let db_pool = PgPoolOptions::new().connect(&CONFIG.database_url).await?;

    let app = endpoints::app(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
