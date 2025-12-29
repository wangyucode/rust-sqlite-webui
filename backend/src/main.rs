use axum::{
    routing::{get, post},
    Router,
    Json,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_sqlite_webui=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // 构建应用路由
    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/query", post(execute_query))
        .layer(CorsLayer::permissive()) // 开发阶段允许跨域
        .layer(TraceLayer::new_for_http());

    // 绑定端口
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health_check() -> impl IntoResponse {
    Json(serde_json::json!({ "status": "ok", "version": "0.1.0" }))
}

#[derive(Deserialize)]
struct QueryRequest {
    sql: String,
}

#[derive(Serialize)]
struct QueryResponse {
    result: String, // 暂时用字符串占位
}

async fn execute_query(Json(payload): Json<QueryRequest>) -> impl IntoResponse {
    tracing::info!("Executing query: {}", payload.sql);
    // TODO: 实现真正的 SQL 执行逻辑
    Json(QueryResponse {
        result: format!("Executed: {}", payload.sql),
    })
}
