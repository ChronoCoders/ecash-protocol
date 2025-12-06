use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Token {
    pub serial_number: Vec<u8>,
    pub denomination: u64,
    pub currency: String,
    pub signature: Vec<u8>,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub institution_id: String,
    pub key_id: String,
}

impl Token {
    pub fn new(
        serial_number: Vec<u8>,
        denomination: u64,
        currency: String,
        signature: Vec<u8>,
        expires_at: DateTime<Utc>,
        institution_id: String,
        key_id: String,
    ) -> Self {
        Self {
            serial_number,
            denomination,
            currency,
            signature,
            issued_at: Utc::now(),
            expires_at,
            institution_id,
            key_id,
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }

    pub fn serial_hex(&self) -> String {
        hex::encode(&self.serial_number)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlindedToken {
    pub blinded_message: Vec<u8>,
    pub denomination: u64,
    pub currency: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlindSignature {
    pub signature: Vec<u8>,
    pub key_id: String,
}

#[derive(Debug, Clone)]
pub struct TokenMetadata {
    pub serial_number: Vec<u8>,
    pub blinding_factor: Vec<u8>,
    pub denomination: u64,
    pub currency: String,
}
