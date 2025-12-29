[English] | [简体中文](design_CN.md)

# SQLite WebUI Design Document

## 1. Project Overview

This project aims to build a Web-based SQLite database management tool, striving for ultimate simplicity and lightweight. Users can directly manage local SQLite database files, execute SQL queries, and view/edit data through a browser.

**Core Goals**:
- **Lightweight**: Single binary distribution (ideally), ready to use out of the box.
- **High Performance**: Capable of smoothly handling large data displays (virtual scrolling).
- **Simple & Easy to Use**: UI design based on the "Less is More" principle, focusing on core SQL operations.

## 2. Tech Stack Architecture

### 2.1 Backend (Rust)
- **Web Framework**: `Axum` - High performance, ergonomic asynchronous Web framework.
- **Runtime**: `Tokio` - Standard for Rust asynchronous ecosystem.
- **Database Interaction**: `SQLx` - Pure Rust asynchronous SQL driver, supporting connection pools.
- **Serialization**: `Serde` + `Serde JSON` - Handling frontend-backend data exchange.
- **Config/CLI**: `Clap` - Command-line argument parsing.

### 2.2 Frontend (Modern Web)
- **Framework**: `Svelte 5` - Next-generation reactive framework, no virtual DOM, Runes state management.
- **Build Tool**: `Vite` - Blazing fast development server and build tool.
- **UI Framework**: `TailwindCSS` + `DaisyUI` - Utility-first CSS framework and semantic component library.
- **Code Editor**: `CodeMirror 6` - Modern, modular code editor supporting SQL syntax highlighting and autocomplete.
- **Data Table**: `TanStack Table` (Svelte Adapter) - Headless table library for building high-performance data tables.
- **State Management**: Svelte 5 built-in Runes (`$state`, `$derived`).

## 3. System Architecture Design

### 3.1 Overall Architecture
Adopts a frontend-backend separation architecture, but in the production environment, frontend static resources will be embedded into the Rust binary and distributed via Axum's Static File service, achieving "single file deployment".

```mermaid
graph TD
    User[User Browser] <--> |HTTP/WebSocket| Backend[Rust Axum Server]
    Backend <--> |SQLx| SQLite[SQLite DB File]
    Backend --> |Serve| Static[Frontend Static Resources (Svelte Build Artifacts)]
```

### 3.2 Directory Structure Planning
```text
rust-sqlite-webui/
├── Cargo.toml          # Rust project configuration
├── src/                # Backend source code
│   ├── main.rs         # Entry point
│   ├── api/            # API route handling
│   │   ├── mod.rs
│   │   ├── query.rs    # SQL execution interface
│   │   └── db.rs       # Database metadata interface
│   ├── state.rs        # Global application state (connection pool, etc.)
│   └── utils.rs        # Utility functions
├── web/                # Frontend source code (Svelte Kit / Vite Project)
│   ├── package.json
│   ├── vite.config.ts
│   ├── tailwind.config.js
│   ├── src/
│   │   ├── lib/        # Components
│   │   │   ├── Editor.svelte
│   │   │   ├── DataTable.svelte
│   │   │   └── Sidebar.svelte
│   │   ├── App.svelte  # Root component
│   │   └── main.ts     # Entry point
│   └── public/
└── doc/                # Documentation
```

## 4. Core Functional Modules & API Design

### 4.1 Connection Management
Although SQLite is a file database, we need to manage the "currently open database".
- **API**:
  - `POST /api/connect`: Connect to the SQLite file at the specified path.
  - `GET /api/status`: Get current connection status.
  - `POST /api/disconnect`: Close connection.

### 4.2 Database Metadata
Used for displaying database structure in the sidebar.
- **API**:
  - `GET /api/tables`: Get all table names.
  - `GET /api/tables/:name/schema`: Get structure of a specific table (columns, types, primary keys).
  - `GET /api/views`: Get all views.

### 4.3 SQL Executor
Core function for executing user-input SQL.
- **API**:
  - `POST /api/query`: Execute SQL statement.
    - **Request**: `{ "sql": "SELECT * FROM users LIMIT 100" }`
    - **Response**: `{ "columns": ["id", "name"], "rows": [[1, "Alice"], [2, "Bob"]], "execution_time_ms": 12 }`
    - **Error Handling**: Return detailed SQL error information.

### 4.4 Data Explorer
Provides a simple table view to browse table data.
- **API**:
  - `GET /api/tables/:name/data?page=1&limit=50`: Get paginated table data.

## 5. UI/UX Design Draft

Interface layout adopts a classic **Three-Column Layout** (similar to VS Code or DBeaver):

*   **Header**: Top bar, containing Logo, current database path, supports dropdown for history connections, load button.
*   **Sidebar (Left)**: Database object browser.
    *   Collapsible tree structure: Tables, Views, Indexes.
    *   Clicking a table name can quickly preview data or generate a `SELECT` statement.
*   **Main Content (Right)**: Tabbed workspace.
    *   **SQL Editor Tab**: Upper part is code editor (CodeMirror).
    *   **Tool Bar**: Toolbar, including execute SQL, CRUD operations after selecting rows.
    *   **Table Data Tab**: Pure table browse mode, supporting simple sorting and filtering.

**Theme Style**:
- Defaults to following system dark/light mode.
- Uses DaisyUI's `light` and `dark` themes.
- Accent color uses Rust's official orange.
