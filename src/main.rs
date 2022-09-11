use std::{net::SocketAddr, sync::Arc};

use anyhow::{Context, Result};
use axum::{
    routing::{get, post},
    Router, Server,
};
use shortland::{
    backend::{RedisBackend, InMemoryBackend},
    handlers::{create_shorten, expand_shorten},
    settings::{Config, LoggingLevel},
    shortener::HashIds,
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

    let shortner = HashIds::new(None);
    let _backend = RedisBackend;
    use shortland::AppState;

    let backend2 = InMemoryBackend::default();

    let state = AppState::builder()
        .shortner(shortner)
        .config(config.clone())
        .backend(backend2)
        .build();

    let app = Router::with_state(Arc::new(state))
        .route(
            "/",
            get(|| async {
                tracing::warn!("Hello woorld");
                "Hello world"
            }),
        )
        .route("/urls", post(create_shorten))
        .route("/urls/:shorten", get(expand_shorten))
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
