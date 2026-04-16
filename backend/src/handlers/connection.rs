use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode};
use crate::state::AppState;
use std::path::Path;

#[derive(Deserialize)]
pub struct ConnectRequest {
    pub path: String,
    #[serde(default)]
    pub create: bool,
}

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
    match std::fs::read_dir(db_dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_file() {
                        if let Some(name) = entry.file_name().to_str() {
                            // Filter out -wal and -shm files
                            if !name.ends_with("-wal") && !name.ends_with("-shm") {
                                files.push(name.to_string());
                            }
                        }
                    }
                }
            }
            Json(files).into_response()
        }
        Err(e) => {
            tracing::error!("Failed to read db directory: {}", e);
             (StatusCode::INTERNAL_SERVER_ERROR, "Failed to read db directory").into_response()
        }
    }
}

pub async fn connect_db(
    State(state): State<AppState>,
    Json(payload): Json<ConnectRequest>,
) -> impl IntoResponse {
    // Security check: prevent directory traversal
    if payload.path.contains('/') || payload.path.contains('\\') || payload.path.contains("..") {
         return (StatusCode::BAD_REQUEST, "Invalid filename. Only filenames in ./db/ are allowed.").into_response();
    }

    let db_dir = Path::new("db");
    if !db_dir.exists() {
         if let Err(e) = std::fs::create_dir(db_dir) {
             tracing::error!("Failed to create db directory: {}", e);
             return (StatusCode::INTERNAL_SERVER_ERROR, "Failed to create db directory").into_response();
        }
    }

    let full_path = db_dir.join(&payload.path);

    if !full_path.exists() && !payload.create {
        return (StatusCode::NOT_FOUND, "Database file not found").into_response();
    }
    
    let options = SqliteConnectOptions::new()
        .filename(&full_path)
        .create_if_missing(payload.create)
        .journal_mode(SqliteJournalMode::Wal);
    
    match SqlitePool::connect_with(options).await {
        Ok(pool) => {
            let mut db = state.db.write().await;
            *db = Some(pool);
            tracing::info!("Connected to database: {:?}", full_path);
            (StatusCode::OK, "Connected").into_response()
        }
        Err(e) => {
            tracing::error!("Failed to connect: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to connect: {}", e)).into_response()
        }
    }
}

pub async fn list_tables(State(state): State<AppState>) -> impl IntoResponse {
    let db = state.db.read().await;

    if let Some(pool) = db.as_ref() {
        let query = "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%' ORDER BY name";
        match sqlx::query_scalar::<_, String>(query)
            .fetch_all(pool)
            .await 
        {
            Ok(tables) => Json(tables).into_response(),
            Err(e) => {
                tracing::error!("Failed to fetch tables: {}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Failed to fetch tables").into_response()
            }
        }
    } else {
        (StatusCode::BAD_REQUEST, "No database connected").into_response()
    }
}
