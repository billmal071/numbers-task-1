use config::ConfigError;
use dotenv::dotenv;
use serde::Deserialize;
use tracing::{info, instrument};
use tracing_subscriber::filter::EnvFilter;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
}

impl Config {
    #[instrument]
    pub fn from_env() -> Result<Config, ConfigError> {
        dotenv().ok();

        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::from_default_env())
            .init();

        info!("Loading configuration!");

        let settings = config::Config::builder()
            .add_source(config::Environment::default())
            .build()?;

        settings.try_deserialize::<Config>() 
    }
}
