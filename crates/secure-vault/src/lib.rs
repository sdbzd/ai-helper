//! Secure storage and key management skeleton crate.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use thiserror::Error;

pub struct CredentialVault {
    provider: Arc<dyn KeyProvider + Send + Sync>,
    store: Arc<Mutex<HashMap<String, CredentialBlob>>>,
}

impl fmt::Debug for CredentialVault {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let entry_count = self.store.lock().map(|map| map.len()).unwrap_or(0);
        f.debug_struct("CredentialVault")
            .field("provider", &self.provider.label())
            .field("entries", &entry_count)
            .finish()
    }
}

impl Default for CredentialVault {
    fn default() -> Self {
        Self::new(Arc::new(NoopKeyProvider))
    }
}

impl CredentialVault {
    pub fn new(provider: Arc<dyn KeyProvider + Send + Sync>) -> Self {
        Self {
            provider,
            store: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Persist an entry using the derived key. Encryption is mocked for now
    /// but the flow matches a real implementation (derive key -> seal bytes).
    pub fn store(&self, entry: CredentialEntry) -> Result<(), VaultError> {
        let key = self.provider.derive_key(&entry.id)?;
        let sealed = CredentialBlob {
            ciphertext: serde_json::to_vec(&entry)
                .map_err(|e| VaultError::Internal(e.to_string()))?,
            key_alias: key.alias,
        };

        let mut guard = self
            .store
            .lock()
            .map_err(|_| VaultError::Internal("vault lock poisoned".into()))?;
        guard.insert(entry.id.clone(), sealed);
        Ok(())
    }

    /// Fetch and "decrypt" an entry; errors if missing or corrupted.
    pub fn fetch(&self, id: &str) -> Result<CredentialEntry, VaultError> {
        let guard = self
            .store
            .lock()
            .map_err(|_| VaultError::Internal("vault lock poisoned".into()))?;
        let blob = guard
            .get(id)
            .ok_or_else(|| VaultError::NotFound(id.to_string()))?;

        serde_json::from_slice::<CredentialEntry>(&blob.ciphertext)
            .map_err(|e| VaultError::Internal(e.to_string()))
    }
}

/// Key provider abstraction to wrap Android Keystore / platform key stores.
pub trait KeyProvider {
    fn derive_key(&self, label: &str) -> Result<KeyHandle, VaultError>;

    fn label(&self) -> &'static str {
        "key-provider"
    }
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

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct CredentialBlob {
    ciphertext: Vec<u8>,
    key_alias: String,
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

    fn label(&self) -> &'static str {
        "noop-key-provider"
    }
}

