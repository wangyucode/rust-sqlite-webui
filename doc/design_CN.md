[English](design.md) | [简体中文]

# SQLite WebUI 设计方案

## 1. 项目概述

本项目旨在构建一个基于 Web 的 SQLite 数据库管理工具，追求极致的简洁与轻量。用户可以通过浏览器直接管理本地的 SQLite 数据库文件，执行 SQL 查询，查看和编辑数据。

**核心目标**:
- **轻量级**: 单二进制文件分发（理想情况下），启动即用。
- **高性能**: 能够流畅处理大量数据的展示（虚拟滚动）。
- **简洁易用**: UI 设计基于 "Less is More" 原则，专注于核心 SQL 操作。

## 2. 技术栈架构 (方案 A)

### 2.1 后端 (Rust)
- **Web 框架**: `Axum` - 高性能、人体工学极佳的异步 Web 框架。
- **运行时**: `Tokio` - Rust 异步生态的标准。
- **数据库交互**: `SQLx` - 纯 Rust 实现的异步 SQL 驱动，支持连接池。
- **序列化**: `Serde` + `Serde JSON` - 处理前后端数据交换。
- **配置/CLI**: `Clap` - 命令行参数解析。

### 2.2 前端 (Modern Web)
- **框架**: `Svelte 5` - 下一代响应式框架，无虚拟 DOM，Runes 状态管理。
- **构建工具**: `Vite` - 极速开发服务器和构建工具。
- **UI 框架**: `TailwindCSS` + `DaisyUI` - 实用优先的 CSS 框架及语义化组件库。
- **代码编辑器**: `CodeMirror 6` - 现代、模块化的代码编辑器，支持 SQL 语法高亮和自动补全。
- **数据表格**: `TanStack Table` (Svelte Adapter) - 无头表格库，用于构建高性能数据表格。
- **状态管理**: Svelte 5 内置 Runes (`$state`, `$derived`)。

## 3. 系统架构设计

### 3.1 整体架构
采用前后端分离架构，但在生产环境中，前端静态资源将嵌入到 Rust 二进制文件中，通过 Axum 提供的 Static File 服务进行分发，实现“单文件部署”。

```mermaid
graph TD
    User[用户浏览器] <--> |HTTP/WebSocket| Backend[Rust Axum Server]
    Backend <--> |SQLx| SQLite[SQLite DB File]
    Backend --> |Serve| Static[前端静态资源 (Svelte构建产物)]
```

### 3.2 目录结构规划
```text
rust-sqlite-webui/
├── Cargo.toml          # Rust 项目配置
├── src/                # 后端源码
│   ├── main.rs         # 入口
│   ├── api/            # API 路由处理
│   │   ├── mod.rs
│   │   ├── query.rs    # SQL 执行接口
│   │   └── db.rs       # 数据库元数据接口
│   ├── state.rs        # 全局应用状态 (连接池等)
│   └── utils.rs        # 工具函数
├── web/                # 前端源码 (Svelte Kit / Vite Project)
│   ├── package.json
│   ├── vite.config.ts
│   ├── tailwind.config.js
│   ├── src/
│   │   ├── lib/        # 组件
│   │   │   ├── Editor.svelte
│   │   │   ├── DataTable.svelte
│   │   │   └── Sidebar.svelte
│   │   ├── App.svelte  # 根组件
│   │   └── main.ts     # 入口
│   └── public/
└── doc/                # 文档
```

## 4. 核心功能模块与 API 设计

### 4.1 连接管理
虽然 SQLite 是文件数据库，但我们需要管理“当前打开的数据库”。
- **API**:
  - `POST /api/connect`: 连接到指定路径的 SQLite 文件。
  - `GET /api/status`: 获取当前连接状态。
  - `POST /api/disconnect`: 关闭连接。

### 4.2 数据库元数据
用于侧边栏展示数据库结构。
- **API**:
  - `GET /api/tables`: 获取所有表名。
  - `GET /api/tables/:name/schema`: 获取指定表的结构（列、类型、主键）。
  - `GET /api/views`: 获取所有视图。

### 4.3 SQL 执行器
核心功能，用于执行用户输入的 SQL。
- **API**:
  - `POST /api/query`: 执行 SQL 语句。
    - **Request**: `{ "sql": "SELECT * FROM users LIMIT 100" }`
    - **Response**: `{ "columns": ["id", "name"], "rows": [[1, "Alice"], [2, "Bob"]], "execution_time_ms": 12 }`
    - **Error Handling**: 返回详细的 SQL 错误信息。

### 4.4 数据浏览 (Data Explorer)
提供简单的表格视图查看表数据。
- **API**:
  - `GET /api/tables/:name/data?page=1&limit=50`: 分页获取表数据。

## 5. UI/UX 设计草图

界面布局采用经典的 **三栏式布局** (类似 VS Code 或 DBeaver)：

*   **Header**: 顶部栏，包含 Logo、当前数据库路径，支持下拉框选择历史连接、加载按钮。
*   **Sidebar (左侧)**: 数据库对象浏览器。
    *   折叠式树状结构: Tables, Views, Indexes.
    *   点击表名可快速预览数据或生成 `SELECT` 语句。
*   **Main Content (右侧)**: 选项卡式工作区。
    *   **SQL Editor Tab**: 上半部分为代码编辑器 (CodeMirror)。
    *   **Tool Bar**: 工具栏，包括执行SQL，选中行之后的增删改查操作。
    *   **Table Data Tab**: 纯表格浏览模式，支持简单的排序和过滤。

**主题风格**:
- 默认跟随系统深色/浅色模式。
- 使用 DaisyUI 的 `light` 和 `dark` 主题。
- 强调色使用 Rust 的官方橙色。