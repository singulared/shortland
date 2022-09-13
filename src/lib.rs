use backend::redis::RedisBackend;
use service::State;
use shortener::HashIds;

pub mod backend;
pub mod errors;
pub mod handlers;
pub mod service;
pub mod settings;
pub mod shortener;

pub type AppState = State<HashIds, RedisBackend>;
