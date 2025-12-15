//! Script recording, editing, and versioning skeleton.

use crate::automation::{LoginScript, Selector, Step, TargetApp, ValueRef};
use serde::{Deserialize, Serialize};

#[derive(Debug, Default)]
pub struct ScriptManager;

impl ScriptManager {
    pub fn record(&self, target: TargetApp) -> RecordedSession {
        RecordedSession {
            target,
            steps: Vec::new(),
            selectors: Vec::new(),
        }
    }

    pub fn save(&self, script: LoginScript) -> anyhow::Result<()> {
        // Placeholder for persistence (e.g., SQLite + versioning)
        println!("saving script {}@{}", script.meta.id, script.meta.version);
        Ok(())
    }

    pub fn load(&self, id: &str, version: Option<&str>) -> anyhow::Result<LoginScript> {
        Err(anyhow::anyhow!("script {id} not found (placeholder)"))
    }
}

/// Captured artifacts during recording.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedSession {
    pub target: TargetApp,
    pub steps: Vec<Step>,
    pub selectors: Vec<Selector>,
}

/// Version control metadata for scripts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptVersion {
    pub id: String,
    pub version: String,
    pub changelog: Option<String>,
    pub created_at: String,
    pub author: Option<String>,
}

/// Simple helper to build scripts programmatically.
pub fn build_login_script(meta: (&str, &str), target: TargetApp, steps: Vec<Step>) -> LoginScript {
    LoginScript {
        meta: crate::automation::ScriptMeta {
            id: meta.0.to_string(),
            version: meta.1.to_string(),
            author: None,
            created_at: None,
            updated_at: None,
        },
        target,
        steps,
        validations: vec![],
        error_handlers: vec![],
    }
}

