//! Cross-platform (web + Android) UI automation engine skeleton.
//! Provides abstractions for drivers, captcha handling, and login script model.

mod types;
pub use types::*;

use async_trait::async_trait;
use std::fmt;
use std::sync::Arc;

/// High-level automation engine orchestrating platform-specific drivers.
pub struct AutomationEngine {
    drivers: Vec<Arc<dyn AutomationDriver + Send + Sync>>,
    captcha: Arc<dyn CaptchaHandler + Send + Sync>,
}

impl fmt::Debug for AutomationEngine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let driver_names: Vec<&'static str> = self.drivers.iter().map(|d| d.name()).collect();
        f.debug_struct("AutomationEngine")
            .field("drivers", &driver_names)
            .field("captcha", &self.captcha.label())
            .finish()
    }
}

impl Default for AutomationEngine {
    fn default() -> Self {
        Self::with_defaults()
    }
}

impl AutomationEngine {
    pub fn new(
        drivers: Vec<Arc<dyn AutomationDriver + Send + Sync>>,
        captcha: Arc<dyn CaptchaHandler + Send + Sync>,
    ) -> Self {
        Self { drivers, captcha }
    }

    /// Convenience initializer with built-in stubs for Web/Android and a
    /// no-op captcha handler. This keeps the engine usable out-of-the-box.
    pub fn with_defaults() -> Self {
        Self {
            drivers: vec![Arc::new(WebDriverStub), Arc::new(AndroidDriverStub)],
            captcha: Arc::new(NoopCaptcha),
        }
    }

    /// Register an additional driver at runtime (useful for tests or plugins).
    pub fn register_driver(&mut self, driver: Arc<dyn AutomationDriver + Send + Sync>) {
        self.drivers.push(driver);
    }

    /// Run a login script on the best-suited driver.
    pub async fn run(&self, script: LoginScript) -> anyhow::Result<LoginOutcome> {
        for driver in &self.drivers {
            if driver.supports(&script.target) {
                return driver
                    .execute(script.clone(), self.captcha.as_ref())
                    .await
                    .map_err(|err| anyhow::anyhow!("driver {} failed: {err}", driver.name()));
            }
        }
        Err(anyhow::anyhow!(
            "no suitable driver found for {:?}",
            script.target.kind
        ))
    }
}

/// Driver interface for a platform (web / android).
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
    fn label(&self) -> &'static str {
        "captcha-handler"
    }
    async fn solve(&self, challenge: CaptchaChallenge) -> anyhow::Result<CaptchaSolution>;
}

/// Minimal placeholder implementations to illustrate structure.
pub struct NoopCaptcha;

#[async_trait]
impl CaptchaHandler for NoopCaptcha {
    fn label(&self) -> &'static str {
        "noop-captcha"
    }

    async fn solve(&self, challenge: CaptchaChallenge) -> anyhow::Result<CaptchaSolution> {
        Ok(CaptchaSolution {
            challenge,
            response: "manual-input-placeholder".into(),
            strategy: CaptchaStrategy::Manual,
        })
    }
}

/// Very lightweight stub for web automation. In production this would wrap
/// Playwright or a similar driver.
pub struct WebDriverStub;

#[async_trait]
impl AutomationDriver for WebDriverStub {
    fn name(&self) -> &'static str {
        "web-stub"
    }

    fn supports(&self, target: &TargetApp) -> bool {
        matches!(target.kind, TargetAppKind::Web)
    }

    async fn execute(
        &self,
        script: LoginScript,
        captcha: &dyn CaptchaHandler,
    ) -> anyhow::Result<LoginOutcome> {
        simulate_steps("web", &script, captcha).await
    }
}

/// Stub for Android automation (e.g., accessibility service).
pub struct AndroidDriverStub;

#[async_trait]
impl AutomationDriver for AndroidDriverStub {
    fn name(&self) -> &'static str {
        "android-stub"
    }

    fn supports(&self, target: &TargetApp) -> bool {
        matches!(target.kind, TargetAppKind::Android)
    }

    async fn execute(
        &self,
        script: LoginScript,
        captcha: &dyn CaptchaHandler,
    ) -> anyhow::Result<LoginOutcome> {
        simulate_steps("android", &script, captcha).await
    }
}

/// Simulate step execution and demonstrate captcha delegation without touching
/// real UI stacks.
async fn simulate_steps(
    platform: &str,
    script: &LoginScript,
    captcha: &dyn CaptchaHandler,
) -> anyhow::Result<LoginOutcome> {
    // Capture that we would iterate over steps and handle captcha challenges.
    for step in &script.steps {
        match step {
            Step::WaitFor(selector) => {
                tracing::info!(platform, ?selector, "wait for selector");
            }
            Step::Click(selector) => {
                tracing::info!(platform, ?selector, "click");
            }
            Step::Input { selector, value } => {
                tracing::info!(platform, ?selector, ?value, "input");
            }
            Step::SleepMs(ms) => {
                tracing::info!(platform, duration_ms = *ms, "sleep");
            }
            Step::Conditional { on_true, on_false, .. } => {
                tracing::info!(platform, "conditional branch (simulated=true)");
                for branch_step in on_true {
                    tracing::debug!(platform, ?branch_step, "conditional true branch step");
                }
                for branch_step in on_false {
                    tracing::debug!(platform, ?branch_step, "conditional false branch step");
                }
            }
            Step::Loop { times, body } => {
                tracing::info!(platform, iterations = *times, "loop body (not executed)");
                for loop_step in body {
                    tracing::debug!(platform, ?loop_step, "loop step");
                }
            }
        }
    }

    // Demonstrate captcha solving once to exercise the interface.
    let _ = captcha
        .solve(CaptchaChallenge {
            kind: CaptchaKind::Image,
            payload: vec![],
            metadata: Some(format!("{}-placeholder", platform)),
        })
        .await?;

    Ok(LoginOutcome {
        success: true,
        session_token: Some(format!("{}-session-token", platform)),
        error: None,
    })
}

