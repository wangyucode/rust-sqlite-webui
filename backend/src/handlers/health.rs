use axum::{response::IntoResponse, Json};

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "version": option_env!("APP_VERSION").unwrap_or(env!("CARGO_PKG_VERSION")) }))
}
