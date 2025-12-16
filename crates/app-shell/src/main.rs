//! Entry point placeholder for the multi-crate workspace.

use automation_engine::AutomationEngine;
use integration_ipc::IpcHandler;
use script_manager::ScriptManager;
use secure_vault::CredentialVault;

#[derive(Debug)]
pub struct AppContext {
    pub automation: AutomationEngine,
    pub scripts: ScriptManager,
    pub vault: CredentialVault,
}

impl AppContext {
    pub fn new() -> Self {
        Self {
            automation: AutomationEngine::with_defaults(),
            scripts: ScriptManager::default(),
            vault: CredentialVault::default(),
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let ctx = AppContext::new();
    let ipc = IpcHandler {
        vault: &ctx.vault,
        automation: &ctx.automation,
    };

    println!("app-shell initialized: {ctx:?}");
    // Example: demonstrate IPC call path with dummy script.
    let _ = ipc.handle(integration_ipc::IpcRequest::StoreCredential {
        id: "demo".into(),
        username: "user".into(),
        secret: "pass".into(),
    })?;

    Ok(())
}

