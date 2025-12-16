use serde::{Deserialize, Serialize};

/// Metadata about a target app (web/native).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TargetApp {
    pub kind: TargetAppKind,
    pub name: String,
    pub version: Option<String>,
    pub endpoint: Option<String>, // URL or package id
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TargetAppKind {
    Web,
    Android,
}

/// Core login script model.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginScript {
    pub meta: ScriptMeta,
    pub target: TargetApp,
    pub steps: Vec<Step>,
    pub validations: Vec<Validation>,
    pub error_handlers: Vec<ErrorHandler>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScriptMeta {
    pub id: String,
    pub version: String,
    pub author: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Step {
    Click(Selector),
    Input { selector: Selector, value: ValueRef },
    WaitFor(Selector),
    SleepMs(u64),
    Conditional {
        condition: Condition,
        on_true: Vec<Step>,
        on_false: Vec<Step>,
    },
    Loop { times: u32, body: Vec<Step> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Selector {
    Css(String),
    XPath(String),
    AccessibilityId(String),
    Image(String),
    Coordinates { x: i32, y: i32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Condition {
    Exists(Selector),
    TextEquals { selector: Selector, expected: String },
    And(Vec<Condition>),
    Or(Vec<Condition>),
    Not(Box<Condition>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Validation {
    pub description: String,
    pub condition: Condition,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorHandler {
    pub name: String,
    pub on_error: Vec<Step>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueRef {
    Literal(String),
    FromVault(String), // key in secure storage
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginOutcome {
    pub success: bool,
    pub session_token: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaChallenge {
    pub kind: CaptchaKind,
    pub payload: Vec<u8>, // image bytes or other data
    pub metadata: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaptchaKind {
    Image,
    Slider,
    OtpPush,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaptchaStrategy {
    Ocr,
    ThirdParty,
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaptchaSolution {
    pub challenge: CaptchaChallenge,
    pub response: String,
    pub strategy: CaptchaStrategy,
}

