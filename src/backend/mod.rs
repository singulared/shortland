use std::fmt::Debug;

use async_trait::async_trait;
use chrono::{Utc, DateTime};

pub mod redis;
pub mod memory;

#[async_trait]
pub trait Backend 
where
    Self::Error: Debug,
{
    type Error;
    async fn store<'a>(&self, url: &'a str) -> Result<u64, Self::Error>;
    async fn retrive(&self, id: u64) -> Result<String, Self::Error>;
    async fn stat(&self, id: u64, since: Option<DateTime<Utc>>) -> Result<u64, Self::Error>;
    async fn update<'a>(&self, id: u64, url: &'a str) -> Result<(), Self::Error>;
    async fn delete(&self, id: u64) -> Result<(), Self::Error>;
}
