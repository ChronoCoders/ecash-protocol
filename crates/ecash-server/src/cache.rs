use crate::error::ApiResult;
use redis::{aio::ConnectionManager, AsyncCommands};

pub struct RedisCache {
    client: ConnectionManager,
}

impl RedisCache {
    pub async fn new(redis_url: &str) -> ApiResult<Self> {
        let client = redis::Client::open(redis_url)?;
        let manager = ConnectionManager::new(client).await?;

        Ok(Self { client: manager })
    }

    pub async fn check_and_mark_spent(
        &self,
        serial_hex: &str,
        ttl_seconds: i64,
    ) -> ApiResult<bool> {
        let key = format!("spent:{}", serial_hex);

        let was_set: bool = self.client.clone().set_nx(&key, "1").await?;

        if was_set {
            let _: () = self.client.clone().expire(&key, ttl_seconds).await?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn is_token_spent(&self, serial_hex: &str) -> ApiResult<bool> {
        let key = format!("spent:{}", serial_hex);
        let exists: bool = self.client.clone().exists(&key).await?;
        Ok(exists)
    }

    pub async fn health_check(&self) -> ApiResult<()> {
        use redis::cmd;
        let mut conn = self.client.clone();
        cmd("PING").query_async::<()>(&mut conn).await?;
        Ok(())
    }
}
