use axum::{response::IntoResponse, Json};

pub async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "version": "0.1.0" }))
}
