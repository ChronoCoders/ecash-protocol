use thiserror::Error;

#[derive(Error, Debug)]
pub enum EcashError {
    #[error("Cryptographic operation failed")]
    CryptoError,

    #[error("Invalid signature")]
    InvalidSignature,

    #[error("Token expired")]
    TokenExpired,

    #[error("Invalid denomination")]
    InvalidDenomination,

    #[error("Serialization error")]
    SerializationError,

    #[error("Invalid key")]
    InvalidKey,

    #[error("Blinding failed")]
    BlindingFailed,
}

pub type Result<T> = std::result::Result<T, EcashError>;
