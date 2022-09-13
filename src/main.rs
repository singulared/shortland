use std::{net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use axum::{
    routing::{get, post},
    Router, Server,
};
use shortland::{
    backend::redis::RedisBackend,
    handlers::{create_shorten, expand_shorten, get_stat_by_shorten, update_shorten, delete_shorten},
    settings::{Config, LoggingLevel, Backend},
    shortener::HashIds, AppState,
};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::Level;
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
    dbg!(&config);
    initialize_logging(&config.logging.level);

    let shortner = HashIds::new(None).context("Unable to initialize shortner")?;
    let backend = match &config.backend {
        Backend::Redis(backend_config) => RedisBackend::new(backend_config.connection.as_str()).await?,
    };

    let state = AppState::builder()
        .shortner(shortner)
        .config(config.clone())
        .backend(backend)
        .build();

    let app = Router::with_state(Arc::new(state))
        .route("/urls", post(create_shorten))
        .route("/urls/:shorten", get(expand_shorten).put(update_shorten).delete(delete_shorten))
        .route("/urls/:shorten/stats", get(get_stat_by_shorten))
        .layer(ServiceBuilder::new().layer(
            TraceLayer::new_for_http().on_response(DefaultOnResponse::new().level(Level::INFO)),
        ));

    let address = SocketAddr::new(
        config
            .http
            .host
            .parse()
            .context("Unable to parse server ip")?,
        config.http.port,
    );
    Server::try_bind(&address)
        .context("Unable to bind a server")?
        .serve(app.into_make_service())
        .await
        .context("Unable to start a server")?;
    Ok(())
}
