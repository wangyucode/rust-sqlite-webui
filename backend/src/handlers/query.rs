use axum::{response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct QueryRequest {
    pub sql: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub result: String, // 暂时用字符串占位
}

pub async fn execute_query(Json(payload): Json<QueryRequest>) -> impl IntoResponse {
    tracing::info!("Executing query: {}", payload.sql);
    // TODO: 实现真正的 SQL 执行逻辑
    Json(QueryResponse {
        result: format!("Executed: {}", payload.sql),
    })
}
