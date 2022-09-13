use crate::{backend::Backend, errors::ServiceError, settings::Config, shortener::Shortner};

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
