use crate::error::{ClientError, Result};
use ecash_core::{BlindedToken, BlindSignature, Token};
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct PublicKeyResponse {
    pub key_id: String,
    pub institution_id: String,
    pub public_key_n: String,
    pub public_key_e: String,
    pub denominations: Vec<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct WithdrawRequest {
    pub amount: u64,
    pub denomination: u64,
    pub blinded_tokens: Vec<BlindedToken>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WithdrawResponse {
    pub blind_signatures: Vec<BlindSignature>,
    pub key_id: String,
    pub expires_at: String,
    pub transaction_id: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RedeemRequest {
    pub tokens: Vec<Token>,
    pub merchant_id: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedeemResponse {
    pub accepted_count: usize,
    pub total_amount: u64,
    pub transaction_id: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorResponse {
    pub error: String,
}

pub struct ApiClient {
    client: Client,
    base_url: String,
}

impl ApiClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    pub async fn get_public_key(&self) -> Result<PublicKeyResponse> {
        let url = format!("{}/api/v1/keys", self.base_url);
        let response = self.client.get(&url).send().await?;
        
        if !response.status().is_success() {
            let error: ApiErrorResponse = response.json().await?;
            return Err(ClientError::ApiError(error.error));
        }
        
        Ok(response.json().await?)
    }

    pub async fn withdraw(&self, request: WithdrawRequest) -> Result<WithdrawResponse> {
        let url = format!("{}/api/v1/withdraw", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;
        
        if !response.status().is_success() {
            let error: ApiErrorResponse = response.json().await?;
            return Err(ClientError::ApiError(error.error));
        }
        
        Ok(response.json().await?)
    }

    pub async fn redeem(&self, request: RedeemRequest) -> Result<RedeemResponse> {
        let url = format!("{}/api/v1/redeem", self.base_url);
        let response = self.client.post(&url).json(&request).send().await?;
        
        if !response.status().is_success() {
            let error: ApiErrorResponse = response.json().await?;
            return Err(ClientError::ApiError(error.error));
        }
        
        Ok(response.json().await?)
    }

    pub async fn health_check(&self) -> Result<bool> {
        let url = format!("{}/health", self.base_url);
        let response = self.client.get(&url).send().await?;
        Ok(response.status().is_success())
    }
}
