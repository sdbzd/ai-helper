//! Library entry exposing high-level contexts and traits for the system.

pub mod automation;
pub mod integration;
pub mod script_manager;
pub mod secure_storage;

/// Shared application context; in a full Tauri app this would carry
/// handles to IPC, secure storage, and automation engine instances.
#[derive(Debug, Default)]
pub struct AppContext {
    pub automation: automation::AutomationEngine,
    pub scripts: script_manager::ScriptManager,
    pub vault: secure_storage::CredentialVault,
}

impl AppContext {
    pub fn new() -> Self {
        Self::default()
    }
}

