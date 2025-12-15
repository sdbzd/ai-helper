# 模块级设计摘要

## UI 自动化引擎
- 架构：统一编排层 + 驱动适配器(Web/Android/iOS) + 资源管理 + 验证码管线
- 抽象接口：`AutomationDriver` (supports/execute)、`CaptchaHandler`
- 插件机制：驱动通过注册表装配；策略通过配置文件选择
- 性能/资源：分级超时、元素查找退避、截图/录屏按需、隔离进程减少内存泄漏

## 脚本管理器
- 数据结构：YAML/JSON，含 meta/version/steps/validations/error_handlers
- 录制原理：hook DOM/无障碍事件，生成多套选择器 + 语义标签
- 测试框架：沙箱执行 + 断言校验 + 重放日志
- 版本控制：脚本 ID + 语义化版本；变更历史、回滚、共享导入导出

## 安全存储
- 密钥管理：平台 HSM 生成主密钥，派生数据密钥；支持轮换与吊销
- 数据结构：按凭证 ID 存储 username/secret/token/metadata（AES-GCM 加密）
- 生物识别：解封装密钥前触发；失败锁定冷却
- 备份恢复：密钥封装 + 云/本地加密备份；恢复时二次验证

## 集成与跨平台
- Tauri：主进程 UI；侧载进程运行自动化；IPC 仅传递最小必要数据
- 平台适配：Web 使用 Playwright；Android 用 Accessibility；iOS 用 XCUITest
- 回退策略：按目标类型优先级选择驱动，失败自动切换

