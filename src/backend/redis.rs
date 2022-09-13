use async_trait::async_trait;
use chrono::{DateTime, Duration, Utc};
use redis::{aio::ConnectionManager, Client, IntoConnectionInfo, Script};
use thiserror::Error;
use uuid::Uuid;

use super::Backend;

static STORE_SCRIPT: &str = r"
local id = redis.call('INCR', 'LID');
redis.call('SET', id, ARGV[1]);
return id;";

static RETRIVE_SCRIPT: &str = r"
local url = redis.call('GET', ARGV[1]);
local key = 'stat:'..ARGV[1]..':'..ARGV[2];
if url then
    redis.call('ZADD', key, ARGV[3], ARGV[4]);
    redis.call('EXPIRE', key, 172800, 'NX');
end
return url;
";

static RETRIVE_STAT: &str = r"
local today_stat_key = 'stat:'..ARGV[1]..':'..ARGV[2];
local yesterday_stat_key = 'stat:'..ARGV[1]..':'..ARGV[3];
local today_stat = redis.call('ZCOUNT', today_stat_key, ARGV[4], ARGV[5]);
local yesterday_stat = redis.call('ZCOUNT', yesterday_stat_key, ARGV[4], ARGV[5]);
return today_stat + yesterday_stat;
";

static KEY_DATE_FORMAT: &str = "%Y%m%d";

static DEFAULT_STAT_PERIOD_IN_HOURS: i64 = 24;

pub struct RedisBackend {
    client: ConnectionManager,
}

impl RedisBackend {
    pub async fn new<T: IntoConnectionInfo>(connection_info: T) -> Result<Self, RedisBackendError> {
        let connection = Client::open(connection_info)?;
        Ok(Self {
            client: connection.get_tokio_connection_manager().await?,
        })
    }
}

#[derive(Error, Debug)]
pub enum RedisBackendError {
    #[error(transparent)]
    Client(#[from] redis::RedisError),
    #[error("Shorten not found")]
    NotFound,
}

#[async_trait]
impl Backend for RedisBackend {
    type Error = RedisBackendError;

    async fn store<'a>(&self, url: &'a str) -> Result<u64, Self::Error> {
        let mut con = self.client.clone();
        let script = Script::new(STORE_SCRIPT);
        let result = script.arg(url).invoke_async(&mut con).await?;
        Ok(result)
    }

    async fn retrive(&self, id: u64) -> Result<String, Self::Error> {
        let mut con = self.client.clone();
        let uuid = Uuid::new_v4();
        let now = Utc::now();
        let date = now.date().format(KEY_DATE_FORMAT).to_string();
        let ts = now.timestamp();
        let member = format!("{}:{}", ts, uuid);
        let script = Script::new(RETRIVE_SCRIPT);
        let result = script
            .arg(id)
            .arg(date)
            .arg(ts)
            .arg(member)
            .invoke_async(&mut con)
            .await?;
        Ok(result)
    }

    async fn stat(&self, id: u64, since: Option<DateTime<Utc>>) -> Result<u64, Self::Error> {
        let mut con = self.client.clone();
        let now = Utc::now();
        let today = now.date();
        let yesterday = today.pred();
        let since = since.unwrap_or_else(|| {
            now.checked_sub_signed(Duration::hours(DEFAULT_STAT_PERIOD_IN_HOURS))
                .unwrap()
        });
        let stat = Script::new(RETRIVE_STAT)
            .arg(id)
            .arg(today.format(KEY_DATE_FORMAT).to_string())
            .arg(yesterday.format(KEY_DATE_FORMAT).to_string())
            .arg(since.timestamp())
            .arg(now.timestamp())
            .invoke_async(&mut con)
            .await?;
        Ok(stat)
    }

    async fn update<'a>(&self, id: u64, url: &'a str) -> Result<(), Self::Error> {
        let mut con = self.client.clone();
        let res: Option<()> = redis::cmd("SET")
            .arg(id)
            .arg(url)
            .arg("XX")
            .query_async(&mut con)
            .await
            .map_err(RedisBackendError::from)?;
        res.ok_or(RedisBackendError::NotFound)
    }

    async fn delete(&self, id: u64) -> Result<(), Self::Error> {
        let mut con = self.client.clone();
        let today = Utc::now().date();
        let yesterday = today.pred();
        let res: u64 = redis::cmd("DEL")
            .arg(id)
            .arg(format!("stat:{}:{}", id, today.format(KEY_DATE_FORMAT)))
            .arg(format!("stat:{}:{}", id, yesterday.format(KEY_DATE_FORMAT)))
            .query_async(&mut con)
            .await
            .map_err(RedisBackendError::from)?;
        if res == 0 {
            Err(RedisBackendError::NotFound)
        } else {
            Ok(())
        }
    }
}
