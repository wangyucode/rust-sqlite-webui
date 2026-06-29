# Task: SQLite WebUI 支持子目录数据库挂载 + WAL 模式完整访问

## Background

Docker-compose 中 sqlite webui 服务的挂载方式：
```yaml
volumes:
  - ./rust/data/db:/app/db/rust      # Rust 后端数据库（含sqlite.db, access_log.db）
  - ./qingjin/data/db/:/app/db/qingjin  # 青衿AI数据库
```

当前 `list_dbs()` 只列出 `/app/db/` 根目录下的文件，不递归子目录。
导致挂载的子目录中的数据库不会出现在列表中。

## Issues Found

### Issue 1: list_dbs() 不递归子目录
- **位置**: `backend/src/handlers/connection.rs::list_dbs()`
- **问题**: `std::fs::read_dir(db_dir)` 只列出根目录，不递归
- **影响**: 无法发现 `./db/rust/sqlite.db`、`./db/qingjin/qingjin.db` 等子目录数据库

### Issue 2: connect_db() 路径安全检查过于严格
- **位置**: `backend/src/handlers/connection.rs::connect_db()`
- **问题**: `payload.path.contains('/')` 拒绝所有含 `/` 的路径
- **影响**: 无法通过 API 连接子目录中的数据库（如 `rust/sqlite.db`）

### Issue 3: WAL 模式文件完整性检查
- SQLite WAL 模式需要三个文件：`.db`, `.db-wal`, `.db-shm`
- 当前代码没有验证这些文件的完整性
- 建议添加可选的 WAL 文件完整性检查

## Changes Required

### File 1: `backend/src/handlers/connection.rs`

#### Change A: list_dbs() — 递归扫描子目录

```rust
pub async fn list_dbs() -> impl IntoResponse {
    let db_dir = Path::new("db");
    if !db_dir.exists() {
        if let Err(e) = std::fs::create_dir(db_dir) {
             tracing::error!("Failed to create db directory: {}", e);
             return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create db directory").into_response();
        }
        return Json(Vec::<String>::new()).into_response();
    }

    let mut files = Vec::new();
    
    // 递归扫描所有子目录中的 .db 文件
    fn scan_db_files(dir: &Path, result: &mut Vec<String>) {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(name) = entry.file_name().to_str() {
                            // 排除 WAL/SHM 文件
                            if !name.ends_with("-wal") && !name.ends_with("-shm") 
                               && name.ends_with(".db") {
                                // 记录相对于 db/ 目录的相对路径
                                if let Ok(rel) = path.strip_prefix(dir.parent().unwrap_or(dir)) {
                                    result.push(rel.to_string_lossy().to_string());
                                }
                            }
                        }
                    } else if file_type.is_dir() {
                        // 递归子目录
                        scan_db_files(&path, result);
                    }
                }
            }
        }
    }
    
    scan_db_files(db_dir, &mut files);
    Json(files).into_response()
}
```

#### Change B: connect_db() — 放宽路径检查

```rust
// 原代码：
if payload.path.contains('/') || payload.path.contains('\\') || payload.path.contains("..") {
     return (StatusCode::BAD_REQUEST, "Invalid filename. Only filenames in ./db/ are allowed.").into_response();
}

// 改为（允许子目录，但防止路径穿越）：
if payload.path.contains("..") {
     return (StatusCode::BAD_REQUEST, "Invalid path: directory traversal not allowed").into_response();
}
```

### File 2: `backend/src/handlers/connection.rs` — 新增 WAL 完整性检查（可选增强）

在 `connect_db()` 中连接成功后，验证 WAL 文件是否存在：

```rust
// 连接后验证 WAL 模式是否正常工作
fn verify_wal_mode(pool: &SqlitePool) -> Result<(), String> {
    // 检查 journal_mode 是否为 WAL
    let mode: String = sqlx::query_scalar("PRAGMA journal_mode")
        .fetch_one(pool)
        .await
        .map_err(|e| format!("Failed to check journal_mode: {}", e))?;
    
    if mode != "wal" {
        return Err(format!("Expected WAL mode, got: {}", mode));
    }
    
    Ok(())
}
```

## Verification Steps

1. 启动 sqlite webui 服务
2. 调用 `GET /api/dbs` — 应该列出所有子目录中的 .db 文件
3. 通过 API 连接子目录数据库（如 `rust/sqlite.db`）— 应成功
4. 验证 WAL 模式正常工作

## Docker Compose 挂载分析

### Rust Backend (`./rust/data/`)
- `/app/data/db/sqlite.db` — 主数据库，WAL 模式
- `/app/data/db/access_log.db` — Caddy 日志库，WAL 模式
- WAL 文件：`.db-wal`, `.db-shm`

### Qingjin AI (`./qingjin/data/`)  
- `/app/data/db/qingjin.db` — 青衿AI数据库，WAL 模式
- WAL 文件：`.db-wal`, `.db-shm`

### SQLite WebUI (`./sqlite/data/`)
挂载方式（目录级别）：
```yaml
- ./rust/data/db:/app/db/rust      # → /app/db/rust/sqlite.db + .wal/.shm
- ./qingjin/data/db/:/app/db/qingjin  # → /app/db/qingjin/qingjin.db + .wal/.shm
```

### Dashboard (`./dashboard/`)
```yaml
- ./dashboard/meta.db:/app/data/meta.db      # 本地 meta.db
- ./rust/db/sqlite.db:/app/data/db/sqlite.db:ro  # Rust DB 只读挂载
```

## PR Requirements

1. 修改 `list_dbs()` 支持递归子目录扫描
2. 放宽 `connect_db()` 路径检查，允许子目录路径
3. 添加 WAL 模式完整性验证（可选）
4. 更新 API 文档说明子目录数据库访问方式
5. PR title: `feat(sqlite-webui): support recursive subdirectory database mounting for WAL mode`
