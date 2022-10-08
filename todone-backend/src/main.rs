use std::net::SocketAddr;

use clap::Parser;
use sqlx::postgres::PgPoolOptions;

use todone_backend::{config::Config, endpoints};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let config: Config = Config::parse();

    let db_pool = PgPoolOptions::new().connect(&config.database_url).await?;

    let app = endpoints::app(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
