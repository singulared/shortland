use config::{builder::AsyncState, ConfigBuilder, Environment, File};
use serde::{Deserialize, Serialize};
use tracing_subscriber::filter::LevelFilter;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Http {
    pub host: String,
    pub port: u16,
}

impl Default for Http {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_owned(),
            port: 3000,
        }
    }
}

#[derive(Default, Deserialize, Serialize, Debug, Clone)]
pub enum LoggingLevel {
    Trace,
    Debug,
    Info,
    #[default]
    Warning,
    Error,
}

impl From<&LoggingLevel> for LevelFilter {
    fn from(level: &LoggingLevel) -> Self {
        match level {
            LoggingLevel::Trace => LevelFilter::TRACE,
            LoggingLevel::Debug => LevelFilter::DEBUG,
            LoggingLevel::Info => LevelFilter::INFO,
            LoggingLevel::Warning => LevelFilter::WARN,
            LoggingLevel::Error => LevelFilter::ERROR,
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Logging {
    pub level: LoggingLevel,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RedisBackend {
    pub connection: String
}

impl Default for RedisBackend {
    fn default() -> Self {
        Self {
            connection: "redis://localhost:6379/0".to_owned(),
        }
    }
}

#[derive(Serialize, Default, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum Backend {
    Redis(RedisBackend),
    #[default]
    InMemory,
}

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Config {
    pub http: Http,
    pub logging: Logging,
    pub backend: Backend,
}

impl Config {
    pub async fn load() -> Result<Self, anyhow::Error> {
        let config = ConfigBuilder::<AsyncState>::default()
            .add_source(config::Config::try_from(&Config::default())?)
            .add_source(File::with_name("/etc/shortland").required(false))
            .add_source(File::with_name("/url/local/etc/shortland").required(false))
            .add_source(Environment::with_prefix("SL").separator("__"))
            .build()
            .await?;
        Ok(config.try_deserialize()?)
    }
}
