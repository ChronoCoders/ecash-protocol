use num_bigint::{BigInt, BigUint};
use num_traits::{One, Signed, Zero};
use rand::thread_rng;
use rsa::traits::{PrivateKeyParts, PublicKeyParts};
use rsa::{RsaPrivateKey, RsaPublicKey};
use sha2::{Digest, Sha256};

use crate::error::{EcashError, Result};

pub struct BlindSigner {
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl BlindSigner {
    pub fn new(bits: usize) -> Result<Self> {
        let mut rng = thread_rng();
        let private_key =
            RsaPrivateKey::new(&mut rng, bits).map_err(|_| EcashError::CryptoError)?;
        let public_key = private_key.to_public_key();

        Ok(Self {
            private_key,
            public_key,
        })
    }

    pub fn from_keys(private_key: RsaPrivateKey) -> Self {
        let public_key = private_key.to_public_key();
        Self {
            private_key,
            public_key,
        }
    }

    pub fn public_key(&self) -> &RsaPublicKey {
        &self.public_key
    }

    pub fn sign_blinded(&self, blinded_message: &BigUint) -> Result<BigUint> {
        let n = self.private_key.n();
        let n_big = BigUint::from_bytes_be(&n.to_bytes_be());
        let d = self.private_key.d();
        let d_big = BigUint::from_bytes_be(&d.to_bytes_be());

        Ok(blinded_message.modpow(&d_big, &n_big))
    }
}

pub struct BlindUser {
    public_key: RsaPublicKey,
}

impl BlindUser {
    pub fn new(public_key: RsaPublicKey) -> Self {
        Self { public_key }
    }

    pub fn blind_message(&self, message: &[u8]) -> Result<(BigUint, BigUint)> {
        let n = self.public_key.n();
        let n_big = BigUint::from_bytes_be(&n.to_bytes_be());
        let e = self.public_key.e();
        let e_big = BigUint::from_bytes_be(&e.to_bytes_be());

        let mut hasher = Sha256::new();
        hasher.update(message);
        let m = BigUint::from_bytes_be(&hasher.finalize());

        let r = loop {
            let bytes: Vec<u8> = (0..n.to_bytes_be().len())
                .map(|_| rand::random::<u8>())
                .collect();
            let candidate = BigUint::from_bytes_be(&bytes) % &n_big;
            if candidate > BigUint::one() && Self::gcd(&candidate, &n_big) == BigUint::one() {
                break candidate;
            }
        };

        let r_e = r.modpow(&e_big, &n_big);
        let blinded = (&m * &r_e) % &n_big;

        Ok((blinded, r))
    }

    pub fn unblind_signature(
        &self,
        blind_signature: &BigUint,
        blinding_factor: &BigUint,
    ) -> Result<BigUint> {
        let n = self.public_key.n();
        let n_big = BigUint::from_bytes_be(&n.to_bytes_be());

        let r_inv = Self::mod_inverse(blinding_factor, &n_big).ok_or(EcashError::BlindingFailed)?;

        Ok((blind_signature * r_inv) % &n_big)
    }

    pub fn verify_signature(&self, message: &[u8], signature: &BigUint) -> bool {
        let n = self.public_key.n();
        let n_big = BigUint::from_bytes_be(&n.to_bytes_be());
        let e = self.public_key.e();
        let e_big = BigUint::from_bytes_be(&e.to_bytes_be());

        let mut hasher = Sha256::new();
        hasher.update(message);
        let m = BigUint::from_bytes_be(&hasher.finalize());

        let verified = signature.modpow(&e_big, &n_big);
        verified == m
    }

    fn gcd(a: &BigUint, b: &BigUint) -> BigUint {
        let mut a = a.clone();
        let mut b = b.clone();
        while b != BigUint::zero() {
            let temp = b.clone();
            b = &a % &b;
            a = temp;
        }
        a
    }

    fn mod_inverse(a: &BigUint, n: &BigUint) -> Option<BigUint> {
        let a_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &a.to_bytes_be());
        let n_int = BigInt::from_bytes_be(num_bigint::Sign::Plus, &n.to_bytes_be());

        let (g, x, _) = Self::extended_gcd(&a_int, &n_int);

        if g != BigInt::one() {
            return None;
        }

        let result = if x.is_negative() {
            (x + &n_int) % &n_int
        } else {
            x % &n_int
        };

        Some(BigUint::from_bytes_be(&result.to_bytes_be().1))
    }

    fn extended_gcd(a: &BigInt, b: &BigInt) -> (BigInt, BigInt, BigInt) {
        if b.is_zero() {
            return (a.clone(), BigInt::one(), BigInt::zero());
        }

        let (g, x1, y1) = Self::extended_gcd(b, &(a % b));
        let x = y1.clone();
        let y = x1 - (a / b) * y1;

        (g, x, y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blind_signature_flow() {
        let signer = BlindSigner::new(2048).unwrap();
        let user = BlindUser::new(signer.public_key().clone());

        let message = b"test message";

        let (blinded, blinding_factor) = user.blind_message(message).unwrap();
        let blind_sig = signer.sign_blinded(&blinded).unwrap();
        let signature = user
            .unblind_signature(&blind_sig, &blinding_factor)
            .unwrap();

        assert!(user.verify_signature(message, &signature));
    }
}
