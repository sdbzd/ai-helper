# AI Helper Skeleton

根据 Prompt 生成的架构与代码骨架，已拆分为 workspace 多 crate，包含：
- `crates/automation-engine`：驱动抽象、脚本模型、验证码接口
- `crates/script-manager`：录制/保存/版本管理骨架
- `crates/secure-vault`：凭证库与密钥提供者接口
- `crates/integration-ipc`：IPC 请求/响应和处理器占位
- `crates/app-shell`：示例入口（未来接入 Tauri）
- 设计文档：`docs/architecture.md`, `docs/module-guides.md`

## 目录
- `crates/*`：按模块拆分的库/可执行
- `docs/`：架构与模块设计摘要

## 开发提示
- 各模块以 trait + 占位实现形式提供，可按平台与需求填充具体逻辑
- 自动化驱动实现：实现 `AutomationDriver` 并注册到 `AutomationEngine`
- 密钥提供者实现：实现 `KeyProvider` 对接 Keystore
- 工作区构建：在仓库根目录运行 `cargo check`/`cargo test`

