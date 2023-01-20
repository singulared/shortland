use async_trait::async_trait;
use harsh::{BuildError, Harsh, HarshBuilder};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ShortnerError {
    #[error("Shortner initialization error: {0}")]
    Initialization(#[from] BuildError),
    #[error("Shorten Decode error")]
    Decode(#[from] harsh::Error),
}

#[async_trait]
pub trait Shortner {
    async fn decode<'a>(&self, url: &'a str) -> Result<u64, ShortnerError>;
    async fn encode(&self, id: u64) -> Result<String, ShortnerError>;
}

pub struct HashIds {
    convertor: Harsh,
}

impl HashIds {
    pub fn new(_salt: Option<String>) -> Result<Self, ShortnerError> {
        let convertor = HarshBuilder::new().build()?;
        Ok(Self { convertor })
    }
}

#[async_trait]
impl Shortner for HashIds {
    async fn decode<'a>(&self, url: &'a str) -> Result<u64, ShortnerError> {
        let id = *self
            .convertor
            .decode(url)?
            .first()
            .ok_or(ShortnerError::Decode(harsh::Error::Hex))?;
        Ok(id)
    }

    async fn encode(&self, id: u64) -> Result<String, ShortnerError> {
        Ok(self.convertor.encode(&[id]))
    }
}
