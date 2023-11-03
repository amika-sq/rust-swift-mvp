use crate::key_manager::{KeyAlgorithm, KeyManager};

use did_method_key::DIDKey;
use ssi_dids::{DIDMethod, Source};
use std::sync::Arc;

#[derive(uniffi::Enum)]
pub enum DidMethod {
    Key,
}

#[derive(uniffi::Record)]
pub struct Did {
    pub uri: String,
    pub key_manager: Arc<KeyManager>,
}

#[uniffi::export]
pub fn create_did(
    key_manager: Arc<KeyManager>,
    did_method: DidMethod,
    key_algorithm: KeyAlgorithm,
) -> Did {
    let uri: String;

    match did_method {
        DidMethod::Key => {
            let jwk = key_manager.generate_private_key(key_algorithm);
            uri = DIDKey.generate(&Source::Key(&jwk)).unwrap();
        }
    }

    Did { uri, key_manager }
}
