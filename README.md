[English] | [简体中文](README_CN.md)

# SQLite WebUI

A lightweight SQLite Web administration tool based on Rust (Axum) and Svelte 5.

## Tech Stack

- **Backend**: Rust, Axum, SQLx, Tokio
- **Frontend**: Svelte 5, Vite, TailwindCSS, DaisyUI

## Quick Start

### 1. Start Frontend Development Server

Node.js (v18+) is required.

```bash
cd web
npm install
npm start
```

The frontend service runs at `http://localhost:5173` by default.

### 2. Start Backend API Service

Rust (cargo) is required.

```bash
# In the project root directory
cargo run
```

The backend service runs at `http://localhost:3000`.

### 3. Build for Release

**Build Frontend**:
```bash
cd web
npm run build
```
The build artifacts are located in `web/dist`.

**Build Backend**:
```bash
cargo build --release
```

## Documentation

For detailed design documentation, please refer to [doc/design.md](doc/design.md).
