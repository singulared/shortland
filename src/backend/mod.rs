use std::error::Error;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;

pub mod memory;
pub mod redis;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Shorten not found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn Error>),
}

#[async_trait]
pub trait Backend {
    async fn store<'a>(&self, url: &'a str) -> Result<u64, BackendError>;
    async fn retrive(&self, id: u64) -> Result<String, BackendError>;
    async fn stat(&self, id: u64, since: Option<DateTime<Utc>>) -> Result<u64, BackendError>;
    async fn update<'a>(&self, id: u64, url: &'a str) -> Result<(), BackendError>;
    async fn delete(&self, id: u64) -> Result<(), BackendError>;
}

#[async_trait]
impl<B> Backend for Box<B> 
where 
    B: Backend + Sync + ?Sized,
{
    async fn store<'a>(&self, url: &'a str) -> Result<u64, BackendError> {
        self.store(url).await
    }
    async fn retrive(&self, id: u64) -> Result<String, BackendError> { 
        self.retrive(id).await 
    }
    async fn stat(&self, id: u64, since: Option<DateTime<Utc>>) -> Result<u64, BackendError> { 
        self.stat(id, since).await 
    }
    async fn update<'a>(&self, id: u64, url: &'a str) -> Result<(), BackendError> {
        self.update(id, url).await
    }
    async fn delete(&self, id: u64) -> Result<(), BackendError> {
        self.delete(id).await
    }
}
