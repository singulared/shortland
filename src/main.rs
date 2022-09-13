use std::net::SocketAddr;

use anyhow::{Context, Result};
use axum::Server;
use shortland::{
    service::application,
    settings::{Config, LoggingLevel},
};
use tracing::info;
use tracing_subscriber::{filter::LevelFilter, layer::SubscriberExt, util::SubscriberInitExt};

fn initialize_logging(level: &LoggingLevel) {
    tracing_subscriber::registry()
        .with(LevelFilter::from(level))
        .with(tracing_subscriber::fmt::layer())
        .init();
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::load().await.context("Configuration load error")?;
    initialize_logging(&config.logging.level);
    info!("Startup application");

    let app = application(&config).await?;

    let address = SocketAddr::new(
        config
            .http
            .host
            .parse()
            .context("Unable to parse server ip")?,
        config.http.port,
    );
    info!(
        "Run application on {}:{}",
        config.http.host, config.http.port
    );
    Server::try_bind(&address)
        .context("Unable to bind a server")?
        .serve(app.into_make_service())
        .await
        .context("Unable to start a server")?;
    Ok(())
}
