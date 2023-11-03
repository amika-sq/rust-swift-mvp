use crate::KeyStore;

use ssi_jwk::JWK;
use std::sync::Arc;

pub struct KeyManager {
    key_store: Arc<dyn KeyStore>,
}

pub enum KeyAlgorithm {
    Secp256k1,
    Ed25519,
}

impl KeyManager {
    pub fn new(key_store: Arc<dyn KeyStore>) -> Self {
        Self { key_store }
    }

    pub fn generate_private_key(&self, key_algorithm: KeyAlgorithm) -> JWK {
        let jwk: JWK;
        match key_algorithm {
            KeyAlgorithm::Secp256k1 => {
                jwk = JWK::generate_secp256k1().unwrap();
            }
            KeyAlgorithm::Ed25519 => {
                jwk = JWK::generate_ed25519().unwrap();
            }
        }

        let alias = Self::get_alias(&jwk);
        let _ = self
            .key_store
            .set(alias.clone(), crate::jwk::Jwk(jwk.clone()));

        jwk
    }

    // fn get_public_key(&self, alias: &str) -> Option<JWK> {
    //     let jwk = self
    //         .key_store
    //         .get(alias.to_string())
    //         .unwrap()
    //         .unwrap_or_else(None);
    // }

    fn get_alias(key: &JWK) -> String {
        key.thumbprint().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_alias_consistency() {
        let private_jwk = JWK::generate_ed25519().unwrap();
        let public_jwk = private_jwk.to_public();

        let private_alias = KeyManager::get_alias(&private_jwk);
        let public_alias = KeyManager::get_alias(&public_jwk);

        assert_eq!(private_alias, public_alias)
    }
}
