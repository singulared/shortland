use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::info;

use super::{Backend, BackendError};

#[derive(Default)]
pub struct InMemoryBackend {
    storage: RwLock<(u64, HashMap<u64, String>)>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        info!("Initialize InMemory backend");
        Self::default()
    }
}

#[derive(Error, Debug)]
pub enum MemoryBackendError {
    #[error("Record not found")]
    NotFound,
}

#[async_trait]
impl Backend for InMemoryBackend {
    async fn store<'a>(&self, url: &'a str) -> Result<u64, BackendError> {
        let mut storage = self.storage.write().await;
        storage.0 += 1;
        let id = storage.0;
        storage.1.insert(id, url.to_owned());
        Ok(storage.0)
    }

    async fn retrive(&self, id: u64) -> Result<String, BackendError> {
        let storage = self.storage.read().await;
        storage.1.get(&id).cloned().ok_or(BackendError::NotFound)
    }

    async fn stat(&self, _id: u64, _since: Option<DateTime<Utc>>) -> Result<u64, BackendError> {
        Ok(42)
    }

    async fn update<'a>(&self, _id: u64, _url: &'a str) -> Result<(), BackendError> {
        Ok(())
    }

    async fn delete(&self, _id: u64) -> Result<(), BackendError> {
        Ok(())
    }
}
