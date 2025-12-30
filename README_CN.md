[English](README.md) | [简体中文]

# SQLite WebUI

基于 Rust (Axum) 和 Solid.js 的轻量级 SQLite Web 管理工具。

## 技术栈

- **后端**: Rust, Axum, SQLx, Tokio
- **前端**: Solid.js, Vite, TailwindCSS, DaisyUI

## 快速开始

### 1. 启动前端开发服务器

需要安装 Node.js (v18+)。

```bash
cd frontend
pnpm install
pnpm start
```

前端服务默认运行在 `http://localhost:5173`。

### 2. 启动后端 API 服务

需要安装 Rust (cargo)。

```bash
cd backend
cargo run
```

后端服务运行在 `http://localhost:3000`。

### 3. 构建发布

**构建前端**:
```bash
cd frontend
pnpm build
```
构建产物位于 `frontend/dist`。

**构建后端**:
```bash
cd backend
cargo build --release
```

后端发布二进制文件位于 `backend/target/release`。

## 文档

详细设计文档请参考 [doc/design_CN.md](doc/design_CN.md)。
