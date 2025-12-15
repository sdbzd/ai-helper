//! Cross-platform UI automation engine skeleton.

mod types;
pub use types::*;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// High-level automation engine orchestrating platform-specific drivers.
#[derive(Debug, Default)]
pub struct AutomationEngine {
    drivers: Vec<Box<dyn AutomationDriver + Send + Sync>>,
    captcha: Box<dyn CaptchaHandler + Send + Sync>,
}

impl AutomationEngine {
    pub fn new(
        drivers: Vec<Box<dyn AutomationDriver + Send + Sync>>,
        captcha: Box<dyn CaptchaHandler + Send + Sync>,
    ) -> Self {
        Self { drivers, captcha }
    }

    /// Run a login script on the best-suited driver.
    pub async fn run(&self, script: LoginScript) -> anyhow::Result<LoginOutcome> {
        for driver in &self.drivers {
            if driver.supports(&script.target) {
                return driver
                    .execute(script.clone(), self.captcha.as_ref())
                    .await;
            }
        }
        Err(anyhow::anyhow!("no suitable driver found"))
    }
}

/// Driver interface for a platform (web / android / ios).
#[async_trait]
pub trait AutomationDriver {
    fn name(&self) -> &'static str;
    fn supports(&self, target: &TargetApp) -> bool;
    async fn execute(
        &self,
        script: LoginScript,
        captcha: &dyn CaptchaHandler,
    ) -> anyhow::Result<LoginOutcome>;
}

/// Captcha handler abstraction so drivers can delegate OCR/manual/third-party flows.
#[async_trait]
pub trait CaptchaHandler: Send + Sync {
    async fn solve(&self, challenge: CaptchaChallenge) -> anyhow::Result<CaptchaSolution>;
}

/// Minimal placeholder implementations to illustrate structure.
pub struct NoopCaptcha;

#[async_trait]
impl CaptchaHandler for NoopCaptcha {
    async fn solve(&self, challenge: CaptchaChallenge) -> anyhow::Result<CaptchaSolution> {
        Ok(CaptchaSolution {
            challenge,
            response: "manual-input-placeholder".into(),
            strategy: CaptchaStrategy::Manual,
        })
    }
}

