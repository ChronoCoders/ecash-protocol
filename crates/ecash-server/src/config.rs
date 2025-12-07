use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub institution: InstitutionConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InstitutionConfig {
    pub institution_id: String,
    pub key_id: String,
    pub token_expiry_days: i64,
    pub denominations: Vec<u64>,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let denominations: Vec<u64> = env::var("DENOMINATIONS")?
            .split(',')
            .filter_map(|s| s.trim().parse().ok())
            .collect();

        Ok(Self {
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".to_string())
                    .parse()?,
            },
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")?,
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "10".to_string())
                    .parse()?,
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL")?,
            },
            institution: InstitutionConfig {
                institution_id: env::var("INSTITUTION_ID")?,
                key_id: env::var("KEY_ID")?,
                token_expiry_days: env::var("TOKEN_EXPIRY_DAYS")
                    .unwrap_or_else(|_| "90".to_string())
                    .parse()?,
                denominations,
            },
        })
    }
}
