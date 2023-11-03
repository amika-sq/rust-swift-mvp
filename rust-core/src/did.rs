use crate::jwk::Jwk;
use crate::key_manager::{KeyAlgorithm, KeyManager};

use did_method_key::DIDKey;
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

pub enum DidMethod {
    Key,
}

pub struct Did {
    pub uri: String,
    pub key_manager: Arc<KeyManager>,
}

/// Creates a new did:key.
///
/// This function generates a new decentralized identifier (did:key) and stores the corresponding
/// private key in the provided `KeyManager`.
///
/// # Parameters
/// * `key_manager`: Reference to the `KeyManager` where the private key will be stored.
///
/// # Returns
/// Returns the generated did:key as a `String`.
///
/// # Examples
/// ```
/// // Example usage:
/// // let key_manager = KeyManager::new();
/// // let did_key = create_did_key(&key_manager);
/// ```
pub fn create_did(key_manager: Arc<KeyManager>, did_method: DidMethod) -> Did {
    let uri: String;

    match did_method {
        DidMethod::Key => {
            let jwk = key_manager.generate_private_key(KeyAlgorithm::Ed25519);
            uri = DIDKey.generate(&Source::Key(&jwk)).unwrap();
        }
    }

    Did { uri, key_manager }
}

// pub fn create_did_key(jwk: &Jwk) -> String {
//     DIDKey
//         .generate(&Source::Key(&jwk.get_jwk()))
//         .expect("Somehow didn't get a did:key")
// }
//
// struct DidKey {
//     key_manager: Arc<KeyManager>,
// }
