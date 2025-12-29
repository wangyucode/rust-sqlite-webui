use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct AppState {
    pub db: Arc<RwLock<Option<SqlitePool>>>,
}
