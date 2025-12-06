use crate::error::ApiResult;
use crate::models::{TokenRecord, TransactionRecord};
use sqlx::PgPool;

pub struct TransactionLog<'a> {
    pub transaction_type: &'a str,
    pub amount: u64,
    pub denomination: u64,
    pub token_count: usize,
    pub institution_id: &'a str,
    pub key_id: &'a str,
    pub status: &'a str,
    pub error_message: Option<String>,
}

pub struct Database {
    pub pool: PgPool,
}

impl Database {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn check_token_spent(&self, serial_hex: &str) -> ApiResult<bool> {
        let result = sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM tokens WHERE serial_hex = $1)",
        )
        .bind(serial_hex)
        .fetch_one(&self.pool)
        .await?;

        Ok(result)
    }

    pub async fn mark_token_spent(
        &self,
        serial_number: Vec<u8>,
        serial_hex: String,
        denomination: u64,
        currency: &str,
        merchant_id: Option<String>,
    ) -> ApiResult<TokenRecord> {
        let record = sqlx::query_as::<_, TokenRecord>(
            r#"
            INSERT INTO tokens (serial_number, serial_hex, denomination, currency, merchant_id)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING id, serial_number, serial_hex, denomination, currency, status, 
                      redeemed_at, merchant_id, created_at
            "#,
        )
        .bind(serial_number)
        .bind(serial_hex)
        .bind(denomination as i64)
        .bind(currency)
        .bind(merchant_id)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }

    pub async fn log_transaction(&self, log: TransactionLog<'_>) -> ApiResult<TransactionRecord> {
        let record = sqlx::query_as::<_, TransactionRecord>(
            r#"
            INSERT INTO transactions 
                (transaction_type, amount, denomination, token_count, institution_id, key_id, status, error_message)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, transaction_type, amount, denomination, token_count, 
                      institution_id, key_id, status, error_message, request_data, created_at
            "#
        )
        .bind(log.transaction_type)
        .bind(log.amount as i64)
        .bind(log.denomination as i64)
        .bind(log.token_count as i32)
        .bind(log.institution_id)
        .bind(log.key_id)
        .bind(log.status)
        .bind(log.error_message)
        .fetch_one(&self.pool)
        .await?;

        Ok(record)
    }
}
