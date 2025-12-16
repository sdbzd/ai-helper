//! Tauri + IPC integration skeleton.

use automation_engine::{LoginOutcome, LoginScript};
use secure_vault::CredentialVault;

/// IPC message formats (simplified).
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum IpcRequest {
    RunScript(LoginScript),
    StoreCredential { id: String, username: String, secret: String },
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum IpcResponse {
    Ack,
    ScriptResult(LoginOutcome),
    Error(String),
}

/// Host-side handler placeholder. In real Tauri this would be wired into commands.
pub struct IpcHandler<'a> {
    pub vault: &'a CredentialVault,
    pub automation: &'a automation_engine::AutomationEngine,
}

impl<'a> IpcHandler<'a> {
    pub fn handle(&self, req: IpcRequest) -> anyhow::Result<IpcResponse> {
        match req {
            IpcRequest::RunScript(script) => {
                // In real code, spawn async task; simplified here.
                let outcome = futures::executor::block_on(self.automation.run(script))?;
                Ok(IpcResponse::ScriptResult(outcome))
            }
            IpcRequest::StoreCredential {
                id,
                username,
                secret,
            } => {
                self.vault.store(secure_vault::CredentialEntry {
                    id,
                    username,
                    secret,
                    token: None,
                    metadata: None,
                })?;
                Ok(IpcResponse::Ack)
            }
        }
    }
}

