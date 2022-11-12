use std::net::SocketAddr;

use sqlx::postgres::PgPoolOptions;

use todone_backend::{config::CONFIG, endpoints};

use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    env_logger::init();

    // Setting up logging
    {
        let subscriber = FmtSubscriber::builder()
            .with_env_filter(EnvFilter::from_default_env())
            .finish();
        tracing::subscriber::set_global_default(subscriber)
            .expect("setting default subscriber failed");
    }

    let db_pool = PgPoolOptions::new().connect(&CONFIG.database_url).await?;

    let app = endpoints::app(db_pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
