use crate::{shortener::Shortner, backend::Backend, settings::Config};

pub struct State<S> 
where
    S: Shortner,
{
    pub shortner: S,
    pub backend: BoxedBackend,
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
    pub backend: Option<BoxedBackend>,
    pub shortner: Option<S>
}

type BoxedBackend = Box<dyn Backend + Sync + Send>;

impl<S> StateBuilder<S>
where
    S: Shortner,
{
    pub fn backend(self, backend: BoxedBackend) -> StateBuilder<S> {
        StateBuilder { 
            backend: Some(backend), 
            ..self
        }
    }

    pub fn shortner<NS: Shortner>(self, shortener: NS) -> StateBuilder<NS> {
        StateBuilder { 
            shortner: Some(shortener),
            backend: self.backend, 
            config: self.config,
        }
    }

    pub fn config(self, config: Config) -> StateBuilder<S> {
        Self {
            config: Some(config),
            ..self
        }
    }

    pub fn build(self) -> State<S> {
        State {
            shortner: self.shortner.unwrap(),
            backend: self.backend.unwrap(),
            config: self.config.unwrap(),
        }
    }
}
