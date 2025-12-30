[English] | [简体中文](README_CN.md)

# SQLite WebUI

A lightweight SQLite Web administration tool based on Rust (Axum) and Solid.js.

## Tech Stack

- **Backend**: Rust, Axum, SQLx, Tokio
- **Frontend**: Solid.js, Vite, TailwindCSS, DaisyUI

## Quick Start

### 1. Start Frontend Development Server

Node.js (v18+) is required.

```bash
cd frontend
pnpm install
pnpm start
```

The frontend service runs at `http://localhost:5173` by default.

### 2. Start Backend API Service

Rust (cargo) is required.

```bash
cd backend
cargo run
```

The backend service runs at `http://localhost:3000`.

### 3. Build for Release

**Build Frontend**:
```bash
cd frontend
pnpm build
```
The build artifacts are located in `frontend/dist`.

**Build Backend**:
```bash
cd backend
cargo build --release
```

The release binary is located in `backend/target/release`.

## Documentation

For detailed design documentation, please refer to [doc/design.md](doc/design.md).
