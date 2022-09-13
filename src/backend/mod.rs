use std::{fmt::Debug, error::Error};

use async_trait::async_trait;
use chrono::{Utc, DateTime};
use thiserror::Error;

pub mod redis;
pub mod memory;

#[derive(Error, Debug)]
pub enum BackendError {
    #[error("Shorten not found")]
    NotFound,
    #[error(transparent)]
    Internal(Box<dyn Error + Send + Sync>),
    #[error("Datetime overflow")]
    DateTimeOverflow,
}

#[async_trait]
pub trait Backend {
    async fn store<'a>(&self, url: &'a str) -> Result<u64, BackendError>;
    async fn retrive(&self, id: u64) -> Result<String, BackendError>;
    async fn stat(&self, id: u64, since: Option<DateTime<Utc>>) -> Result<u64, BackendError>;
    async fn update<'a>(&self, id: u64, url: &'a str) -> Result<(), BackendError>;
    async fn delete(&self, id: u64) -> Result<(), BackendError>;
}
