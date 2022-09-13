use std::collections::HashMap;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use tokio::sync::RwLock;
use thiserror::Error;

use super::Backend;


pub struct InMemoryBackend {
    storage: RwLock<(u64, HashMap<u64, String>)>,
}

impl Default for InMemoryBackend {
    fn default() -> Self {
        Self {
            storage: RwLock::new((0, HashMap::new())),
        }
    }
}

#[derive(Error, Debug)]
pub enum MemoryBackendError {
    #[error("Record not found")]
    NotFound
}

#[async_trait]
impl Backend for InMemoryBackend {
    type Error = MemoryBackendError;

    async fn store<'a>(&self, url: &'a str) -> Result<u64, Self::Error> {
        let mut storage = self.storage.write().await;
        storage.0 += 1;
        let id = storage.0;
        storage.1.insert(id, url.to_owned());
        Ok(storage.0)
    }

    async fn retrive(&self, id: u64) -> Result<String, Self::Error> {
        let storage = self.storage.read().await;
        storage.1.get(&id).cloned().ok_or(MemoryBackendError::NotFound)
    }

    async fn stat(&self, _id: u64, _since: Option<DateTime<Utc>>) -> Result<u64, Self::Error> {
        Ok(42)
    }

    async fn update<'a>(&self, _id: u64, _url: &'a str) -> Result<(), Self::Error> {
        Ok(())
    }

    async fn delete(&self, _id: u64) -> Result<(), Self::Error> {
        Ok(())
    }
}

