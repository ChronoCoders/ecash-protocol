use chrono::{DateTime, Duration, Utc};
use num_bigint::BigUint;
use rand::Rng;
use rsa::RsaPrivateKey;

use crate::crypto::{BlindSigner, BlindUser};
use crate::error::{EcashError, Result};
use crate::token::{BlindSignature, BlindedToken, Token, TokenMetadata};

pub struct Institution {
    signer: BlindSigner,
    institution_id: String,
    key_id: String,
    denominations: Vec<u64>,
    default_expiry: Duration,
}

impl Institution {
    pub fn new(
        private_key: RsaPrivateKey,
        institution_id: String,
        key_id: String,
        denominations: Vec<u64>,
        default_expiry_days: i64,
    ) -> Self {
        Self {
            signer: BlindSigner::from_keys(private_key),
            institution_id,
            key_id,
            denominations,
            default_expiry: Duration::days(default_expiry_days),
        }
    }

    pub fn institution_id(&self) -> &str {
        &self.institution_id
    }

    pub fn public_key(&self) -> &rsa::RsaPublicKey {
        self.signer.public_key()
    }

    pub fn validate_denomination(&self, denomination: u64) -> Result<()> {
        if self.denominations.contains(&denomination) {
            Ok(())
        } else {
            Err(EcashError::InvalidDenomination)
        }
    }

    pub fn sign_blinded_token(&self, blinded: &BlindedToken) -> Result<BlindSignature> {
        self.validate_denomination(blinded.denomination)?;

        let blinded_msg = BigUint::from_bytes_be(&blinded.blinded_message);
        let signature = self.signer.sign_blinded(&blinded_msg)?;

        Ok(BlindSignature {
            signature: signature.to_bytes_be(),
            key_id: self.key_id.clone(),
        })
    }

    pub fn verify_token(&self, token: &Token) -> Result<bool> {
        if token.is_expired() {
            return Ok(false);
        }

        self.validate_denomination(token.denomination)?;

        let user = BlindUser::new(self.signer.public_key().clone());
        let message = Self::construct_message(
            &token.serial_number,
            token.denomination,
            &token.currency,
            &token.issued_at,
        );

        let signature = BigUint::from_bytes_be(&token.signature);
        Ok(user.verify_signature(&message, &signature))
    }

    pub fn expiry_time(&self) -> DateTime<Utc> {
        Utc::now() + self.default_expiry
    }

    fn construct_message(
        serial: &[u8],
        denomination: u64,
        currency: &str,
        issued_at: &DateTime<Utc>,
    ) -> Vec<u8> {
        let mut message = Vec::new();
        message.extend_from_slice(serial);
        message.extend_from_slice(&denomination.to_be_bytes());
        message.extend_from_slice(currency.as_bytes());
        message.extend_from_slice(&issued_at.timestamp().to_be_bytes());
        message
    }
}

pub struct Wallet {
    user: BlindUser,
    institution_id: String,
    currency: String,
}

impl Wallet {
    pub fn new(public_key: rsa::RsaPublicKey, institution_id: String, currency: String) -> Self {
        Self {
            user: BlindUser::new(public_key),
            institution_id,
            currency,
        }
    }

    pub fn prepare_withdrawal(
        &self,
        amount: u64,
        denomination: u64,
    ) -> Result<Vec<(BlindedToken, TokenMetadata)>> {
        let count = amount.div_ceil(denomination);
        let mut tokens = Vec::new();

        for _ in 0..count {
            let serial = Self::generate_serial();
            let message = Self::construct_message_for_serial(&serial, denomination, &self.currency);

            let (blinded, blinding_factor) = self.user.blind_message(&message)?;

            tokens.push((
                BlindedToken {
                    blinded_message: blinded.to_bytes_be(),
                    denomination,
                    currency: self.currency.clone(),
                },
                TokenMetadata {
                    serial_number: serial,
                    blinding_factor: blinding_factor.to_bytes_be(),
                    denomination,
                    currency: self.currency.clone(),
                },
            ));
        }

        Ok(tokens)
    }

    pub fn finalize_withdrawal(
        &self,
        blind_signatures: Vec<BlindSignature>,
        metadata: Vec<TokenMetadata>,
        expires_at: DateTime<Utc>,
    ) -> Result<Vec<Token>> {
        if blind_signatures.len() != metadata.len() {
            return Err(EcashError::CryptoError);
        }

        let mut tokens = Vec::new();

        for (blind_sig, meta) in blind_signatures.into_iter().zip(metadata) {
            let blind_signature = BigUint::from_bytes_be(&blind_sig.signature);
            let blinding_factor = BigUint::from_bytes_be(&meta.blinding_factor);

            let signature = self
                .user
                .unblind_signature(&blind_signature, &blinding_factor)?;

            let message = Self::construct_message_for_serial(
                &meta.serial_number,
                meta.denomination,
                &meta.currency,
            );

            if !self.user.verify_signature(&message, &signature) {
                return Err(EcashError::InvalidSignature);
            }

            tokens.push(Token::new(
                meta.serial_number,
                meta.denomination,
                meta.currency,
                signature.to_bytes_be(),
                expires_at,
                self.institution_id.clone(),
                blind_sig.key_id,
            ));
        }

        Ok(tokens)
    }

    fn generate_serial() -> Vec<u8> {
        let mut rng = rand::thread_rng();
        let mut serial = vec![0u8; 32];
        rng.fill(&mut serial[..]);
        serial
    }

    fn construct_message_for_serial(serial: &[u8], denomination: u64, currency: &str) -> Vec<u8> {
        let mut message = Vec::new();
        message.extend_from_slice(serial);
        message.extend_from_slice(&denomination.to_be_bytes());
        message.extend_from_slice(currency.as_bytes());
        message.extend_from_slice(&Utc::now().timestamp().to_be_bytes());
        message
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::thread_rng;
    use rsa::RsaPrivateKey;

    #[test]
    fn test_full_withdrawal_flow() {
        let mut rng = thread_rng();
        let private_key = RsaPrivateKey::new(&mut rng, 2048).unwrap();
        let public_key = private_key.to_public_key();

        let institution = Institution::new(
            private_key,
            "inst_test".to_string(),
            "key_001".to_string(),
            vec![10, 50, 100],
            90,
        );

        let wallet = Wallet::new(public_key, "inst_test".to_string(), "USD".to_string());

        let tokens_to_prepare = wallet.prepare_withdrawal(100, 50).unwrap();
        let (blinded_tokens, metadata): (Vec<_>, Vec<_>) = tokens_to_prepare.into_iter().unzip();

        let blind_signatures: Vec<_> = blinded_tokens
            .iter()
            .map(|bt| institution.sign_blinded_token(bt).unwrap())
            .collect();

        let tokens = wallet
            .finalize_withdrawal(blind_signatures, metadata, institution.expiry_time())
            .unwrap();

        for token in &tokens {
            assert!(institution.verify_token(token).unwrap());
        }
    }
}
