use crate::cache::RedisCache;
use crate::config::Config;
use crate::db::Database;
use crate::error::ApiResult;
use ecash_core::Institution;
use rsa::{RsaPrivateKey, RsaPublicKey};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub institution: Arc<Institution>,
    pub public_key: Arc<RsaPublicKey>,
    pub db: Arc<Database>,
    pub cache: Arc<RedisCache>,
    pub config: Arc<Config>,
}

impl AppState {
    pub async fn new(
        institution: Institution,
        public_key: RsaPublicKey,
        db: Database,
        cache: RedisCache,
        config: Config,
    ) -> Self {
        Self {
            institution: Arc::new(institution),
            public_key: Arc::new(public_key),
            db: Arc::new(db),
            cache: Arc::new(cache),
            config: Arc::new(config),
        }
    }

    pub fn institution_id(&self) -> &str {
        &self.config.institution.institution_id
    }

    pub fn key_id(&self) -> &str {
        &self.config.institution.key_id
    }

    pub fn denominations(&self) -> &[u64] {
        &self.config.institution.denominations
    }

    pub fn is_valid_denomination(&self, denomination: u64) -> bool {
        self.denominations().contains(&denomination)
    }
}

pub fn generate_or_load_keys() -> ApiResult<(RsaPrivateKey, RsaPublicKey)> {
    tracing::info!("Generating new RSA key pair (3072-bit)...");

    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 3072)
        .map_err(|e| crate::error::ApiError::Internal(format!("Key generation failed: {}", e)))?;

    let public_key = private_key.to_public_key();

    tracing::info!("RSA key pair generated successfully");
    Ok((private_key, public_key))
}
