use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sqlx::{Column, Row, TypeInfo};
use sqlx::sqlite::SqliteRow;
use crate::state::AppState;
use reqwest::Client;
use std::env;

#[derive(Deserialize)]
pub struct QueryRequest {
    pub sql: String,
}

#[derive(Serialize)]
pub struct QueryResponse {
    pub columns: Vec<String>,
    pub column_types: Vec<String>,
    pub rows: Vec<Vec<Value>>,
    pub affected_rows: Option<u64>,
    pub execution_time: f64,
    pub error: Option<String>,
}

#[derive(Deserialize)]
pub struct AICorrectRequest {
    pub sql: String,
}

#[derive(Serialize)]
pub struct AICorrectResponse {
    pub sql: String,
    pub error: Option<String>,
}

fn serialize_row(row: &SqliteRow) -> Vec<Value> {
    let mut row_data = Vec::new();
    for (i, _) in row.columns().iter().enumerate() {
        let col = row.column(i);
        let type_info = col.type_info();
        let type_name = type_info.name();
        
        let val: Value = if type_name == "NULL" {
            if let Ok(v) = row.try_get::<String, _>(i) {
                Value::String(v)
            } else if let Ok(v) = row.try_get::<i64, _>(i) {
                Value::Number(v.into())
            } else {
                Value::Null
            }
        } else if type_name == "INTEGER" || type_name == "INT" || type_name == "BIGINT" || type_name == "int8" {
            match row.try_get::<i64, _>(i) {
                Ok(v) => Value::Number(v.into()),
                Err(_) => {
                    match row.try_get::<String, _>(i) {
                        Ok(v) => Value::String(v),
                        Err(_) => Value::Null,
                    }
                },
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
                Err(_) => {
                    match row.try_get::<String, _>(i) {
                        Ok(v) => Value::String(v),
                        Err(_) => Value::Null,
                    }
                },
            }
        } else if type_name == "BOOLEAN" || type_name == "BOOL" {
            match row.try_get::<bool, _>(i) {
                Ok(v) => Value::Bool(v),
                Err(_) => {
                    match row.try_get::<String, _>(i) {
                        Ok(v) => Value::String(v),
                        Err(_) => Value::Null,
                    }
                },
            }
        } else if type_name == "BLOB" {
            match row.try_get::<Vec<u8>, _>(i) {
                Ok(v) => {
                    let hex: String = v.iter().map(|b| format!("{:02X}", b)).collect();
                    Value::String(format!("x'{}'", hex))
                },
                Err(_) => Value::Null,
            }
        } else {
            match row.try_get::<String, _>(i) {
                Ok(v) => Value::String(v),
                Err(_) => {
                    Value::String(format!("[{}]", type_name))
                }
            }
        };
        row_data.push(val);
    }
    row_data
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
                column_types: vec![],
                rows: vec![],
                affected_rows: None,
                execution_time: 0.0,
                error: Some("Database not connected".to_string()),
            });
        }
    };

    let start = std::time::Instant::now();
    let sql_upper = payload.sql.trim().to_uppercase();
    let return_rows = sql_upper.starts_with("SELECT") 
        || sql_upper.starts_with("PRAGMA") 
        || sql_upper.starts_with("EXPLAIN") 
        || sql_upper.starts_with("WITH")
        || sql_upper.contains("RETURNING");

    if return_rows {
        let result = sqlx::query(&payload.sql).fetch_all(pool).await;
        let execution_time = start.elapsed().as_secs_f64() * 1000.0;

        match result {
            Ok(rows) => {
                if rows.is_empty() {
                    return Json(QueryResponse {
                        columns: vec![],
                        column_types: vec![],
                        rows: vec![],
                        affected_rows: None,
                        execution_time,
                        error: None,
                    });
                }

                let columns: Vec<String> = rows[0].columns().iter().map(|c| c.name().to_string()).collect();
                let column_types: Vec<String> = rows[0].columns().iter().map(|c| c.type_info().name().to_string()).collect();
                
                let data: Vec<Vec<Value>> = rows.iter().map(serialize_row).collect();

                Json(QueryResponse {
                    columns,
                    column_types,
                    rows: data,
                    affected_rows: None,
                    execution_time,
                    error: None,
                })
            }
            Err(e) => {
                Json(QueryResponse {
                    columns: vec![],
                    column_types: vec![],
                    rows: vec![],
                    affected_rows: None,
                    execution_time,
                    error: Some(e.to_string()),
                })
            }
        }
    } else {
        let result = sqlx::query(&payload.sql).execute(pool).await;
        let execution_time = start.elapsed().as_secs_f64() * 1000.0;

        match result {
            Ok(result) => {
                Json(QueryResponse {
                    columns: vec![],
                    column_types: vec![],
                    rows: vec![],
                    affected_rows: Some(result.rows_affected()),
                    execution_time,
                    error: None,
                })
            }
            Err(e) => {
                Json(QueryResponse {
                    columns: vec![],
                    column_types: vec![],
                    rows: vec![],
                    affected_rows: None,
                    execution_time,
                    error: Some(e.to_string()),
                })
            }
        }
    }
}

pub async fn ai_correct_sql(
    Json(payload): Json<AICorrectRequest>,
) -> impl IntoResponse {
    let api_key = env::var("OPENAI_API_KEY").unwrap_or_default();
    let api_base = env::var("OPENAI_API_BASE").unwrap_or_else(|_| "https://api.deepseek.com".to_string());
    let model = env::var("OPENAI_MODEL_NAME").unwrap_or_else(|_| "deepseek-chat".to_string());

    if api_key.is_empty() {
        return Json(AICorrectResponse {
            sql: payload.sql,
            error: Some("OPENAI_API_KEY is not set in environment".to_string()),
        });
    }

    let client = Client::new();
    let response = client
        .post(format!("{}/chat/completions", api_base))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": model,
            "messages": [
                {
                    "role": "system",
                    "content": "You are a professional SQL expert. Your task is to correct the provided SQLite SQL query. Return ONLY the corrected SQL query without any explanation or markdown formatting."
                },
                {
                    "role": "user",
                    "content": format!("Correct this SQLite query: {}", payload.sql)
                }
            ],
            "temperature": 0.0
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            let body: Value = resp.json().await.unwrap_or(json!({}));
            if let Some(content) = body["choices"][0]["message"]["content"].as_str() {
                Json(AICorrectResponse {
                    sql: content.trim().to_string(),
                    error: None,
                })
            } else {
                Json(AICorrectResponse {
                    sql: payload.sql,
                    error: Some("Invalid response from AI API".to_string()),
                })
            }
        }
        Err(e) => Json(AICorrectResponse {
            sql: payload.sql,
            error: Some(e.to_string()),
        }),
    }
}
