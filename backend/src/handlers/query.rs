use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Column, Row, TypeInfo};
use crate::state::AppState;

#[derive(Deserialize)]
pub struct QueryRequest {
    pub sql: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub columns: Vec<String>,
    pub rows: Vec<Vec<Value>>,
    pub error: Option<String>,
}

pub async fn execute_query(
    State(state): State<AppState>,
    Json(payload): Json<QueryRequest>,
) -> impl IntoResponse {
    tracing::info!("Executing query: {}", payload.sql);
    
    let db = state.db.read().await;
    let pool = match &*db {
        Some(pool) => pool,
        None => {
            return Json(QueryResponse {
                columns: vec![],
                rows: vec![],
                error: Some("Database not connected".to_string()),
            });
        }
    };

    let result = sqlx::query(&payload.sql).fetch_all(pool).await;

    match result {
        Ok(rows) => {
            if rows.is_empty() {
                return Json(QueryResponse {
                    columns: vec![],
                    rows: vec![],
                    error: None,
                });
            }

            let columns: Vec<String> = rows[0].columns().iter().map(|c| c.name().to_string()).collect();
            
            let mut data = Vec::new();
            for row in rows {
                let mut row_data = Vec::new();
                for (i, _) in row.columns().iter().enumerate() {
                     let col = row.column(i);
                     let type_info = col.type_info();
                     let type_name = type_info.name();
                     
                     let val: Value = if type_name == "NULL" {
                         Value::Null
                     } else if type_name == "INTEGER" || type_name == "INT" || type_name == "BIGINT" || type_name == "int8" {
                         match row.try_get::<i64, _>(i) {
                             Ok(v) => Value::Number(v.into()),
                             Err(_) => Value::Null,
                         }
                     } else if type_name == "REAL" || type_name == "FLOAT" || type_name == "DOUBLE" {
                         match row.try_get::<f64, _>(i) {
                             Ok(v) => {
                                 if let Some(n) = serde_json::Number::from_f64(v) {
                                    Value::Number(n)
                                 } else {
                                    Value::Null 
                                 }
                             },
                             Err(_) => Value::Null,
                         }
                     } else if type_name == "BOOLEAN" || type_name == "BOOL" {
                          match row.try_get::<bool, _>(i) {
                             Ok(v) => Value::Bool(v),
                             Err(_) => Value::Null,
                         }
                     } else {
                         match row.try_get::<String, _>(i) {
                             Ok(v) => Value::String(v),
                             Err(_) => {
                                 // Fallback: try to convert whatever it is to string if possible, or use a placeholder
                                 Value::String(format!("[{}]", type_name))
                             }
                         }
                     };
                     row_data.push(val);
                }
                data.push(row_data);
            }

            Json(QueryResponse {
                columns,
                rows: data,
                error: None,
            })
        }
        Err(e) => {
             Json(QueryResponse {
                columns: vec![],
                rows: vec![],
                error: Some(e.to_string()),
            })
        }
    }
}
