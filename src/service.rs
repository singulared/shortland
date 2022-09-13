use std::sync::Arc;

use anyhow::Context;
use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::{DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{
    backend::{memory::InMemoryBackend, redis::RedisBackend, Backend},
    errors::ServiceError,
    handlers::{
        create_shorten, delete_shorten, expand_shorten, get_stat_by_shorten, update_shorten,
    },
    settings::{self, Config},
    shortener::{HashIds, Shortner},
    AppState,
};

type BoxedBackend = dyn Backend + Send + Sync;

pub struct State<S>
where
    S: Shortner,
{
    pub shortner: S,
    pub backend: Box<BoxedBackend>,
    pub config: Config,
}

impl<S> State<S>
where
    S: Shortner,
{
    pub fn builder() -> StateBuilder<S> {
        StateBuilder {
            shortner: None,
            backend: None,
            config: None,
        }
    }
}

#[derive(Default)]
pub struct StateBuilder<S>
where
    S: Shortner,
{
    pub config: Option<Config>,
    pub shortner: Option<S>,
    pub backend: Option<Box<BoxedBackend>>,
}

impl<S> StateBuilder<S>
where
    S: Shortner,
{
    pub fn shortner<NS: Shortner>(self, shortener: NS) -> StateBuilder<NS> {
        StateBuilder {
            backend: self.backend,
            shortner: Some(shortener),
            config: self.config,
        }
    }

    pub fn config(self, config: Config) -> StateBuilder<S> {
        Self {
            config: Some(config),
            ..self
        }
    }

    pub fn backend(self, backend: Box<BoxedBackend>) -> StateBuilder<S> {
        Self {
            backend: Some(backend),
            ..self
        }
    }

    pub fn build(self) -> Result<State<S>, ServiceError> {
        Ok(State {
            shortner: self
                .shortner
                .ok_or(ServiceError::State("Uninitialized shortner"))?,
            backend: self
                .backend
                .ok_or(ServiceError::State("Uninitialized backend"))?,
            config: self
                .config
                .ok_or(ServiceError::State("Uninitialized config"))?,
        })
    }
}

pub async fn application(config: &Config) -> anyhow::Result<Router<Arc<AppState>>> {
    let shortner = HashIds::new(None).context("Unable to initialize shortner")?;
    let backend: Box<BoxedBackend> = match &config.backend {
        settings::Backend::Redis(backend_config) => Box::new(
            RedisBackend::new(backend_config.connection.as_str())
                .await
                .context("Unable initialize redis backend")?,
        ),
        settings::Backend::InMemory => Box::new(InMemoryBackend::new()),
    };

    let state = AppState::builder()
        .shortner(shortner)
        .config(config.clone())
        .backend(backend)
        .build()
        .context("Unable to initialize application state")?;

    let app = Router::with_state(Arc::new(state))
        .route("/urls", post(create_shorten))
        .route(
            "/urls/:shorten",
            get(expand_shorten)
                .put(update_shorten)
                .delete(delete_shorten),
        )
        .route("/urls/:shorten/stats", get(get_stat_by_shorten))
        .layer(ServiceBuilder::new().layer(
            TraceLayer::new_for_http().on_response(DefaultOnResponse::new().level(Level::INFO)),
        ));
    Ok(app)
}
