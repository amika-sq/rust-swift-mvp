use crate::jwk::Jwk;
use std::fmt::Debug;

#[derive(Debug, thiserror::Error)]
pub enum KeyStoreError {
    #[error("I have no clue what happened")]
    Unknown,
}

pub trait KeyStore: Send + Sync + Debug {
    fn get(&self, key: String) -> Result<Option<Jwk>, KeyStoreError>;
    fn set(&self, key: String, value: Jwk) -> Result<(), KeyStoreError>;
}
