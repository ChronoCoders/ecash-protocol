use ecash_core::{BlindSignature, BlindedToken, Token};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawRequest {
    pub amount: u64,
    pub denomination: u64,
    pub blinded_tokens: Vec<BlindedToken>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WithdrawResponse {
    pub blind_signatures: Vec<BlindSignature>,
    pub key_id: String,
    pub expires_at: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedeemRequest {
    pub tokens: Vec<Token>,
    pub merchant_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RedeemResponse {
    pub accepted_count: usize,
    pub total_amount: u64,
    pub transaction_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicKeyResponse {
    pub key_id: String,
    pub institution_id: String,
    pub public_key_n: String,
    pub public_key_e: String,
    pub denominations: Vec<u64>,
    pub expires_at: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
    pub redis: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct VerifyRequest {
    pub token: Token,
}

#[derive(Debug, Clone, Serialize)]
pub struct VerifyResponse {
    pub valid: bool,
    pub expired: bool,
    pub spent: bool,
    pub message: String,
}
