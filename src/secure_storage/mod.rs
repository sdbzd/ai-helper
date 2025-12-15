//! Secure storage and key management skeleton.

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Default)]
pub struct CredentialVault {
    provider: Box<dyn KeyProvider + Send + Sync>,
}

impl CredentialVault {
    pub fn new(provider: Box<dyn KeyProvider + Send + Sync>) -> Self {
        Self { provider }
    }

    pub fn store(&self, entry: CredentialEntry) -> Result<(), VaultError> {
        let key = self.provider.derive_key(&entry.id)?;
        // Placeholder: encrypt + persist using platform keystore
        println!("storing credential {} using key {}", entry.id, key.alias);
        Ok(())
    }

    pub fn fetch(&self, id: &str) -> Result<CredentialEntry, VaultError> {
        Err(VaultError::NotFound(id.to_string()))
    }
}

/// Key provider abstraction to wrap Android Keystore / iOS Keychain.
pub trait KeyProvider {
    fn derive_key(&self, label: &str) -> Result<KeyHandle, VaultError>;
}

/// Placeholder key handle.
#[derive(Debug, Clone)]
pub struct KeyHandle {
    pub alias: String,
}

/// Credential entry persisted in encrypted form.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CredentialEntry {
    pub id: String,
    pub username: String,
    pub secret: String,
    pub token: Option<String>,
    pub metadata: Option<String>,
}

#[derive(Debug, Error)]
pub enum VaultError {
    #[error("credential not found: {0}")]
    NotFound(String),
    #[error("key error: {0}")]
    Key(String),
    #[error("internal error: {0}")]
    Internal(String),
}

/// No-op provider useful for tests and scaffolding.
pub struct NoopKeyProvider;

impl KeyProvider for NoopKeyProvider {
    fn derive_key(&self, label: &str) -> Result<KeyHandle, VaultError> {
        Ok(KeyHandle {
            alias: format!("noop-{label}"),
        })
    }
}

