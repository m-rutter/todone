use clap::Parser;
use once_cell::sync::Lazy;

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    dotenv::dotenv().ok();
    Config::parse()
});

#[derive(Parser, Debug)]
pub struct Config {
    #[arg(env = "DATABASE_URL")]
    pub database_url: String,
    #[arg(env = "JWT_SECRET")]
    pub jwt_secret: String,
}
