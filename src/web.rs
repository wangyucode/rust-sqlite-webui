use axum::{
    extract::{Path, State},
    response::{Html, IntoResponse},
    Form,
};
use serde::Deserialize;
use std::sync::{Arc, RwLock};
use crate::db;
use crate::templates;

#[derive(Default)]
pub struct AppState {
    pub db_path: Option<String>,
    pub tables: Vec<String>,
}

pub type SharedState = Arc<RwLock<AppState>>;

#[derive(Deserialize)]
pub struct LoadDbForm {
    pub db_path: String,
}

#[derive(Deserialize)]
pub struct QueryForm {
    pub sql: String,
}

pub async fn index(State(state): State<SharedState>) -> impl IntoResponse {
    let state = state.read().unwrap();
    Html(templates::index_page(state.db_path.clone(), state.tables.clone()).into_string())
}

pub async fn load_db(
    State(state): State<SharedState>,
    Form(form): Form<LoadDbForm>,
) -> impl IntoResponse {
    let db_path = form.db_path.trim();
    if db_path.is_empty() {
        return Html(templates::error_message("Database path cannot be empty").into_string());
    }

    match db::get_tables(db_path) {
        Ok(tables) => {
            let mut state = state.write().unwrap();
            state.db_path = Some(db_path.to_string());
            state.tables = tables.clone();
            Html(templates::main_content(tables, None, None).into_string())
        }
        Err(e) => Html(templates::error_message(&format!("Failed to load database: {}", e)).into_string()),
    }
}

pub async fn get_table(
    State(state): State<SharedState>,
    Path(table_name): Path<String>,
) -> impl IntoResponse {
    let state = state.read().unwrap();
    let db_path = match &state.db_path {
        Some(path) => path,
        None => return Html(templates::error_message("No database loaded").into_string()),
    };

    let sql = format!("SELECT * FROM {} LIMIT 100", table_name);
    match db::execute_query(db_path, &sql) {
        Ok(result) => Html(templates::content_area(Some(table_name), Some(result)).into_string()),
        Err(e) => Html(templates::error_message(&format!("Failed to query table: {}", e)).into_string()),
    }
}

pub async fn execute_query(
    State(state): State<SharedState>,
    Form(form): Form<QueryForm>,
) -> impl IntoResponse {
    let state = state.read().unwrap();
    let db_path = match &state.db_path {
        Some(path) => path,
        None => return Html(templates::error_message("No database loaded").into_string()),
    };

    let sql = form.sql.trim();
    let is_query = sql.to_uppercase().starts_with("SELECT") || sql.to_uppercase().starts_with("PRAGMA");

    if is_query {
        match db::execute_query(db_path, sql) {
            Ok(result) => Html(templates::results_table(result).into_string()),
            Err(e) => Html(templates::error_message(&format!("Query error: {}", e)).into_string()),
        }
    } else {
        match db::execute_update(db_path, sql) {
            Ok(affected) => Html(templates::success_message(&format!("Success! {} rows affected.", affected)).into_string()),
            Err(e) => Html(templates::error_message(&format!("Execution error: {}", e)).into_string()),
        }
    }
}
