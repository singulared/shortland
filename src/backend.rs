use std::collections::HashMap;

use async_trait::async_trait;
use tokio::sync::RwLock;

#[async_trait]
pub trait Backend {
    async fn store<'a>(&self, url: &'a str) -> Result<u64, ()>;
    async fn retrive(&self, id: u64) -> Result<String, ()>;
}

pub struct RedisBackend;

#[async_trait]
impl Backend for RedisBackend {
    async fn store<'a>(&self, _url: &'a str) -> Result<u64, ()> {
        Ok(42)
    }

    async fn retrive(&self, _id: u64) -> Result<String, ()> {
        Ok("test".to_owned())
    }
}

pub struct InMemoryBackend {
    storage: RwLock<(u64, HashMap<u64, String>)>,
}

// impl InMemoryBackend {
    // pub fn new() -> Self {
        // Self {
            // storage: RwLock::new((0, HashMap::new())),
        // }
    // }
// }

impl Default for InMemoryBackend {
    fn default() -> Self {
        Self {
            storage: RwLock::new((0, HashMap::new())),
        }
    }
}

#[async_trait]
impl Backend for InMemoryBackend {
    async fn store<'a>(&self, url: &'a str) -> Result<u64, ()> {
        let mut storage = self.storage.write().await;
        storage.0 += 1;
        let id = storage.0;
        storage.1.insert(id, url.to_owned());
        Ok(storage.0)
    }

    async fn retrive(&self, id: u64) -> Result<String, ()> {
        let storage = self.storage.read().await;
        storage.1.get(&id).cloned().ok_or(())
    }
}
