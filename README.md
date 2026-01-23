# r-web-api-scaffold


一个基于 Rust 和 Actix-Web 构建的高性能 Web API 服务，提供统一的错误处理、响应格式和日志追踪能力。

## ✨ 特性

- 🚀 **高性能** - 基于 Actix-Web 框架，提供卓越的并发性能
- 🔒 **类型安全** - 利用 Rust 的类型系统确保代码安全性
- 📊 **统一响应** - 标准化的 API 响应格式（`MzResp`）
- ⚠️ **错误处理** - 完善的错误处理机制，自动转换为统一响应
- 📝 **日志追踪** - 集成 tracing 框架，支持结构化日志
- 🔧 **环境配置** - 通过环境变量灵活配置服务参数
- 🛡️ **JSON 验证** - 自动处理 JSON 解析错误并返回友好提示

## 🛠️ 技术栈

- **Web 框架**: [actix-web](https://actix.rs/) 4.12.1
- **异步运行时**: [tokio](https://tokio.rs/) (actix-web 内置)
- **错误处理**: [anyhow](https://docs.rs/anyhow/) + [thiserror](https://docs.rs/thiserror/)
- **序列化**: [serde](https://serde.rs/) + [serde_json](https://docs.rs/serde_json/)
- **日志**: [tracing](https://docs.rs/tracing/) + [tracing-subscriber](https://docs.rs/tracing-subscriber/)
- **时间处理**: [chrono](https://docs.rs/chrono/)
- **环境变量**: [dotenv](https://docs.rs/dotenv/)

## 📁 项目结构

```
simple-web-api/
├── src/
│   ├── main.rs              # 应用入口，服务器配置
│   ├── config/
│   │   ├── mod.rs           # 模块导出
│   │   └── errors.rs        # 错误定义和统一响应格式
│   └── handlers/
│       ├── mod.rs           # 处理器模块导出
│       └── api_handler.rs   # API 端点处理
├── Cargo.toml               # 项目依赖配置
├── Cargo.lock               # 依赖版本锁定
└── .env                     # 环境变量配置（不提交到版本控制）
```

## 🚀 快速开始

### 前置要求

- Rust 2024 edition 或更高版本
- Cargo（随 Rust 安装）

### 安装

```bash
# 克隆项目
git clone <repository-url>
cd simple-web-api

# 安装依赖（首次运行时）
cargo build
```

### 配置

创建 `.env` 文件并配置环境变量：

```env
HOST=127.0.0.1
PORT=8080
```

### 运行

```bash
# 开发模式运行
cargo run

# 或使用自定义环境变量
HOST=0.0.0.0 PORT=3000 cargo run
```

服务启动后，访问 `http://127.0.0.1:8080` 即可使用。

## 📡 API 端点

### 健康检查

```http
GET /health
```

**响应示例**：

```json
{
  "code": 0,
  "message": "success",
  "data": "healthy",
  "timestamp": "2025-01-23T10:30:00+00:00"
}
```

### 404 处理

所有未匹配的路由将返回统一的 404 响应：

```json
{
  "code": 500,
  "message": "Not found!!!",
  "data": null,
  "timestamp": "2025-01-23T10:30:00+00:00"
}
```

### JSON 错误处理

当请求体 JSON 格式错误时，自动返回友好提示：

```json
{
  "code": 500,
  "message": "JSON error: ...",
  "data": null,
  "timestamp": "2025-01-23T10:30:00+00:00"
}
```

## 🏗️ 核心设计

### 统一响应格式

所有 API 响应都遵循 `MzResp<T>` 结构：

```rust
pub struct MzResp<T> {
    pub code: u16,        // 状态码（0 表示成功）
    pub message: String,  // 消息描述
    pub data: T,          // 响应数据
}
```

**使用示例**：

```rust
// 成功响应
let resp = MzResp::<&str>::ok("healthy");

// 错误响应
let resp = MzResp::<()>::ng(500, "Internal error", ());
```

### 错误处理机制

使用 `thiserror` 定义自定义错误类型，自动转换为统一响应：

```rust
pub enum MzError {
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

// 实现 ResponseError trait，自动转换错误
impl ResponseError for MzError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Ok().json(MzResp::<&str>::ng(self.code(), &self.to_string(), ""))
    }
}
```

### 统一返回类型

```rust
pub type MzResult<T> = Result<MzResp<T>, MzError>;
```

**处理器示例**：

```rust
pub async fn health_check() -> MzResult<&'static str> {
    info!("Health check");
    let out = MzResp::<&str>::ok("healthy");
    Ok(out)
}

pub async fn not_found() -> MzResult<()> {
    Err(anyhow!("Not found!!!").into())
}
```

## 🔧 开发指南

### 添加新的 API 端点

1. 在 `src/handlers/api_handler.rs` 中定义处理器函数：

```rust
pub async fn my_handler() -> MzResult<String> {
    let data = "my data".to_string();
    Ok(MzResp::ok(data))
}
```

2. 在 `src/main.rs` 中注册路由：

```rust
.route("/my-endpoint", web::get().to(my_handler))
```

### 添加新的错误类型

在 `src/config/errors.rs` 中扩展 `MzError` 枚举：

```rust
#[derive(Error, Debug)]
pub enum MzError {
    #[error("Custom error: {0}")]
    Custom(String),

    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
}

impl MzError {
    pub fn code(&self) -> u16 {
        match self {
            MzError::Custom(_) => 400,
            MzError::Anyhow(_) => 500,
        }
    }
}
```

### 日志使用

```rust
use tracing::{info, warn, error};

info!("信息日志");
warn!("警告日志");
error!("错误日志");
```

## 🧪 测试

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_name

# 显示测试输出
cargo test -- --nocapture
```

## 📝 环境变量

| 变量 | 默认值 | 说明 |
|------|--------|------|
| `HOST` | `127.0.0.1` | 服务器监听地址 |
| `PORT` | `8080` | 服务器监听端口 |

## 🚢 生产部署

```bash
# 优化编译
cargo build --release

# 运行发布版本
./target/release/simple-web-api
```

建议在生产环境中使用反向代理（如 Nginx）和服务管理工具（如 systemd）。

## 📄 许可证

本项目采用 MIT 许可证。详见 LICENSE 文件。

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📧 联系方式

如有问题或建议，请提交 Issue。

---

**Built with ❤️ using Rust and Actix-Web**
