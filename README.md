# AI Helper Skeleton

根据 `REAME.md` 提示生成的架构与代码骨架，包含：
- Rust 库 + 占位入口 (`src/`)
- 模块抽象：自动化引擎、脚本管理、安全存储、集成 IPC
- 设计文档：`docs/architecture.md`, `docs/module-guides.md`

## 目录
- `src/main.rs`：示例入口（未来接入 Tauri）
- `src/lib.rs`：公开上下文与模块
- `src/automation/`：驱动抽象、脚本模型、验证码接口
- `src/script_manager/`：录制/保存/版本管理骨架
- `src/secure_storage/`：凭证库与密钥提供者接口
- `src/integration/`：IPC 请求/响应和处理器占位
- `docs/`：架构与模块设计摘要

## 开发提示
- 各模块以 trait + 占位实现形式提供，可按平台与需求填充具体逻辑
- feature flags (`tauri`, `android`, `ios`) 预留，用于按平台构建
- 自动化驱动实现：实现 `AutomationDriver` 并注册到 `AutomationEngine`
- 密钥提供者实现：实现 `KeyProvider` 对接 Keystore/Keychain

