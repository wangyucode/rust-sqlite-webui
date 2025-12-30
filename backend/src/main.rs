use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

mod state;
mod handlers;

use state::AppState;
use handlers::{health_check, connect_db, execute_query, list_dbs, list_tables};

#[tokio::main]
async fn main() {
    // 初始化日志
    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "rust_sqlite_webui=debug,tower_http=debug".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();

    let state = AppState {
        db: Arc::new(RwLock::new(None)),
    };

    // 构建应用路由
    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/connect", post(connect_db))
        .route("/api/db-files", get(list_dbs))
        .route("/api/tables", get(list_tables))
        .route("/api/query", post(execute_query))
        .layer(CorsLayer::permissive()) // 开发阶段允许跨域
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // 绑定端口
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
