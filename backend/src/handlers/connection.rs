use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use sqlx::SqlitePool;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ConnectRequest {
    pub path: String,
}

pub async fn connect_db(
    State(state): State<AppState>,
    Json(payload): Json<ConnectRequest>,
) -> impl IntoResponse {
    let path = std::path::Path::new(&payload.path);
    if !path.exists() {
        return (StatusCode::NOT_FOUND, "Database file not found").into_response();
    }
    
    let conn_str = format!("sqlite://{}", payload.path);
    
    match SqlitePool::connect(&conn_str).await {
        Ok(pool) => {
            let mut db = state.db.write().await;
            *db = Some(pool);
            tracing::info!("Connected to database: {}", payload.path);
            (StatusCode::OK, "Connected").into_response()
        }
        Err(e) => {
            tracing::error!("Failed to connect: {}", e);
            (StatusCode::INTERNAL_SERVER_ERROR, format!("Failed to connect: {}", e)).into_response()
        }
    }
}
