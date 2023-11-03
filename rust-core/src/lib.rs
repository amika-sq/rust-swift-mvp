uniffi::include_scaffolding!("rust-core");

mod did;
mod jwk;
mod key_manager;
mod key_store;

use crate::did::{create_did, Did, DidMethod};
use crate::jwk::Jwk;
use crate::key_manager::{KeyAlgorithm, KeyManager};
use crate::key_store::{KeyStore, KeyStoreError};

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_generate_random_num() {
    //     let min = 5;
    //     let max = 10;
    //     let result = generate_random_num(min, max);
    //     assert!(result >= min && result <= max);
    // }
}
