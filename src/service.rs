use std::sync::Arc;

use anyhow::Context;
use axum::{
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::trace::{TraceLayer, DefaultOnResponse};
use tracing::Level;

use crate::{
    backend::{redis::RedisBackend, Backend},
    errors::ServiceError,
    handlers::{create_shorten, expand_shorten, update_shorten, delete_shorten, get_stat_by_shorten},
    settings::{self, Config},
    shortener::{HashIds, Shortner},
    AppState,
};

pub struct State<S, B>
where
    S: Shortner,
    B: Backend,
{
    pub shortner: S,
    pub backend: B,
    pub config: Config,
}

impl<S, B> State<S, B>
where
    S: Shortner,
    B: Backend,
{
    pub fn builder() -> StateBuilder<S, B> {
        StateBuilder {
            shortner: None,
            backend: None,
            config: None,
        }
    }
}

#[derive(Default)]
pub struct StateBuilder<S, B>
where
    S: Shortner,
    B: Backend,
{
    pub config: Option<Config>,
    pub backend: Option<B>,
    pub shortner: Option<S>,
}

impl<S, B> StateBuilder<S, B>
where
    S: Shortner,
    B: Backend,
{
    pub fn backend<NB: Backend>(self, backend: NB) -> StateBuilder<S, NB> {
        StateBuilder {
            backend: Some(backend),
            shortner: self.shortner,
            config: self.config,
        }
    }

    pub fn shortner<NS: Shortner>(self, shortener: NS) -> StateBuilder<NS, B> {
        StateBuilder {
            backend: self.backend,
            shortner: Some(shortener),
            config: self.config,
        }
    }

    pub fn config(self, config: Config) -> StateBuilder<S, B> {
        Self {
            config: Some(config),
            ..self
        }
    }

    pub fn build(self) -> Result<State<S, B>, ServiceError> {
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
    let backend = match &config.backend {
        settings::Backend::Redis(backend_config) => {
            RedisBackend::new(backend_config.connection.as_str()).await?
        }
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
