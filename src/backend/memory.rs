use std::{
    borrow::BorrowMut,
    collections::{BTreeMap, BTreeSet, HashMap},
};

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use thiserror::Error;
use tokio::sync::RwLock;
use tracing::info;

use super::{Backend, BackendError};

#[derive(Default)]
pub struct InMemoryBackend {
    storage: RwLock<(u64, HashMap<u64, String>)>,
    stat: RwLock<HashMap<u64, BTreeMap<i64, u64>>>,
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
        dbg!(&storage);
        let ts = Utc::now().timestamp();
        self.stat
            .write()
            .await
            .entry(id)
            .and_modify(|stat| {
                stat.entry(ts)
                    .and_modify(|counter| *counter += 1)
                    .or_insert(1);
            })
            .or_insert_with(|| {
                let mut map = BTreeMap::new();
                map.insert(ts, 1);
                map
            });
        storage.1.get(&id).cloned().ok_or(BackendError::NotFound)
    }

    async fn stat(&self, id: u64, _since: Option<DateTime<Utc>>) -> Result<u64, BackendError> {
        self.stat
            .read()
            .await
            .get(&id)
            .map(BTreeMap::len)
            .map(u64::try_from)
            .map(Result::unwrap)
            .ok_or(BackendError::NotFound)
    }

    async fn update<'a>(&self, id: u64, url: &'a str) -> Result<(), BackendError> {
        self.storage
            .write()
            .await
            .1
            .get_mut(&id)
            .map(|old| *old = url.to_owned())
            .ok_or(BackendError::NotFound)
    }

    async fn delete(&self, id: u64) -> Result<(), BackendError> {
        self.storage
            .write()
            .await
            .1
            .remove(&id);
        self.stat
            .write()
            .await
            .remove(&id);
        Ok(())
    }
}
