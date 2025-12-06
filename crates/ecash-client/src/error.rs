use thiserror::Error;

#[derive(Debug, Error)]
pub enum ClientError {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("eCash core error: {0}")]
    Core(#[from] ecash_core::EcashError),
    
    #[error("Storage error: {0}")]
    Storage(#[from] rusqlite::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Insufficient balance: required {required}, available {available}")]
    InsufficientBalance { required: u64, available: u64 },
    
    #[error("No tokens available")]
    NoTokensAvailable,
    
    #[error("Invalid server response: {0}")]
    InvalidResponse(String),
    
    #[error("API error: {0}")]
    ApiError(String),
    
    #[error("Invalid denomination: {0}")]
    InvalidDenomination(u64),
    
    #[error("QR code error: {0}")]
    QrCode(String),
}

pub type Result<T> = std::result::Result<T, ClientError>;
