use crate::api::{ApiClient, RedeemRequest, WithdrawRequest};
use crate::error::{ClientError, Result};
use crate::storage::{StoredToken, WalletStorage};
use chrono::{DateTime, Utc};
use ecash_core::{Token, Wallet as CoreWallet};
use rsa::RsaPublicKey;

pub struct Wallet {
    api: ApiClient,
    storage: WalletStorage,
    core_wallet: Option<CoreWallet>,
    institution_id: String,
}

impl Wallet {
    pub fn new(server_url: String, db_path: String) -> Result<Self> {
        let api = ApiClient::new(server_url);
        let storage = WalletStorage::new(&db_path)?;
        
        Ok(Self {
            api,
            storage,
            core_wallet: None,
            institution_id: String::new(),
        })
    }

    pub async fn initialize(&mut self) -> Result<()> {
        let key_response = self.api.get_public_key().await?;
        
        // Parse decimal strings to BigUint
        let n = rsa::BigUint::parse_bytes(key_response.public_key_n.as_bytes(), 10)
            .ok_or_else(|| ClientError::InvalidResponse("Invalid public key N".to_string()))?;
        
        let e = rsa::BigUint::parse_bytes(key_response.public_key_e.as_bytes(), 10)
            .ok_or_else(|| ClientError::InvalidResponse("Invalid public key E".to_string()))?;
        
        let public_key = RsaPublicKey::new(n, e)
            .map_err(|e| ClientError::InvalidResponse(format!("Invalid public key: {}", e)))?;
        
        self.core_wallet = Some(CoreWallet::new(
            public_key,
            key_response.institution_id.clone(),
            "USD".to_string(),
        ));
        
        self.institution_id = key_response.institution_id;
        
        Ok(())
    }

    pub async fn withdraw(&self, amount: u64, denomination: u64) -> Result<Vec<Token>> {
        let core_wallet = self.core_wallet.as_ref()
            .ok_or_else(|| ClientError::InvalidResponse("Wallet not initialized".to_string()))?;
        
        let tokens_to_prepare = core_wallet.prepare_withdrawal(amount, denomination)
            .map_err(ClientError::Core)?;
        
        let (blinded_tokens, metadata): (Vec<_>, Vec<_>) = tokens_to_prepare.into_iter().unzip();
        
        let request = WithdrawRequest {
            amount,
            denomination,
            blinded_tokens: blinded_tokens.clone(),
        };
        
        let response = self.api.withdraw(request).await?;
        
        let expires_at = DateTime::parse_from_rfc3339(&response.expires_at)
            .map_err(|e| ClientError::InvalidResponse(format!("Invalid expires_at: {}", e)))?
            .with_timezone(&Utc);
        
        let tokens = core_wallet.finalize_withdrawal(
            response.blind_signatures,
            metadata,
            expires_at,
        ).map_err(ClientError::Core)?;
        
        for token in &tokens {
            self.storage.store_token(token.clone())?;
        }
        
        self.storage.log_transaction(
            "withdraw",
            amount,
            tokens.len(),
            Some(response.transaction_id),
        )?;
        
        Ok(tokens)
    }

    pub async fn spend(&self, amount: u64) -> Result<String> {
        let available = self.storage.get_available_tokens()?;
        
        if available.is_empty() {
            return Err(ClientError::NoTokensAvailable);
        }
        
        let mut selected_tokens = Vec::new();
        let mut total = 0u64;
        let mut token_ids = Vec::new();
        
        for stored_token in available {
            if total >= amount {
                break;
            }
            total += stored_token.token.denomination;
            token_ids.push(stored_token.id);
            selected_tokens.push(stored_token.token);
        }
        
        if total < amount {
            return Err(ClientError::InsufficientBalance { 
                required: amount, 
                available: total 
            });
        }
        
        let request = RedeemRequest {
            tokens: selected_tokens,
            merchant_id: Some("self".to_string()),
        };
        
        let response = self.api.redeem(request).await?;
        
        self.storage.mark_tokens_spent(&token_ids)?;
        
        self.storage.log_transaction(
            "spend",
            amount,
            token_ids.len(),
            Some(response.transaction_id.clone()),
        )?;
        
        Ok(response.transaction_id)
    }

    pub fn get_balance(&self) -> Result<u64> {
        self.storage.get_balance()
    }

    pub fn get_available_tokens(&self) -> Result<Vec<StoredToken>> {
        self.storage.get_available_tokens()
    }

    pub async fn health_check(&self) -> Result<bool> {
        self.api.health_check().await
    }
}
