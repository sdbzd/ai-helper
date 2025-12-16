//! Script recording, editing, and versioning skeleton crate.

use automation_engine::{LoginScript, Selector, Step, TargetApp};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// In-memory script manager that supports recording, saving, loading and
/// keeping multiple versions per script id. This can later be swapped for a
/// persistent backend (SQLite/kv).
#[derive(Debug, Default)]
pub struct ScriptManager {
    store: Arc<Mutex<HashMap<String, Vec<LoginScript>>>>,
}

impl ScriptManager {
    pub fn record(&self, target: TargetApp) -> RecordedSession {
        RecordedSession {
            target,
            steps: Vec::new(),
            selectors: Vec::new(),
        }
    }

    /// Save a script, keeping versions under the same id ordered by insertion.
    pub fn save(&self, script: LoginScript) -> anyhow::Result<()> {
        let mut guard = self
            .store
            .lock()
            .map_err(|_| anyhow::anyhow!("script store poisoned"))?;
        guard.entry(script.meta.id.clone()).or_default().push(script);
        Ok(())
    }

    /// Load a specific version or the latest version of the script.
    pub fn load(&self, id: &str, version: Option<&str>) -> anyhow::Result<LoginScript> {
        let guard = self
            .store
            .lock()
            .map_err(|_| anyhow::anyhow!("script store poisoned"))?;
        let versions = guard
            .get(id)
            .ok_or_else(|| anyhow::anyhow!("script {id} not found"))?;

        if let Some(ver) = version {
            versions
                .iter()
                .find(|s| s.meta.version == ver)
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("script {id} version {ver} not found"))
        } else {
            versions
                .last()
                .cloned()
                .ok_or_else(|| anyhow::anyhow!("script {id} has no versions"))
        }
    }

    /// List available versions for a script id.
    pub fn list_versions(&self, id: &str) -> Vec<ScriptVersion> {
        let guard = self.store.lock().ok();
        guard
            .and_then(|map| map.get(id).cloned())
            .unwrap_or_default()
            .into_iter()
            .map(|s| ScriptVersion {
                id: s.meta.id.clone(),
                version: s.meta.version.clone(),
                changelog: None,
                created_at: s.meta.created_at.clone().unwrap_or_default(),
                author: s.meta.author.clone(),
            })
            .collect()
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
        meta: automation_engine::ScriptMeta {
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

