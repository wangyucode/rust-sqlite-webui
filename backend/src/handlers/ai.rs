use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use reqwest::Client;
use sqlx::Row;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct CorrectRequest {
    pub sql: String,
    pub table: String,
}

#[derive(Serialize)]
pub struct CorrectResponse {
    pub sql: Option<String>,
    pub error: Option<String>,
}

pub async fn correct_sql(
    State(state): State<AppState>,
    Json(payload): Json<CorrectRequest>,
) -> impl IntoResponse {
    let api_key = std::env::var("OPENAI_API_KEY");
    let base_url = std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-5.4-mini".to_string());

    if api_key.is_err() {
        return Json(CorrectResponse {
            sql: None,
            error: Some("OPENAI_API_KEY not configured".to_string()),
        });
    }

    let db = state.db.read().await;
    let schema = if let Some(pool) = db.as_ref() {
        let columns: Vec<(String, String, bool, bool)> = sqlx::query_as::<_, (String, String, bool, bool)>(
            "SELECT name, type, notnull = 1, pk = 1 FROM pragma_table_info(?)"
        )
        .bind(&payload.table)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

        let foreign_keys: Vec<(String, String, String)> = sqlx::query_as::<_, (String, String, String)>(
            "SELECT `from`, `table`, `to` FROM pragma_foreign_key_list(?)"
        )
        .bind(&payload.table)
        .fetch_all(pool)
        .await
        .unwrap_or_default();

        let mut schema_str = String::new();
        for (name, col_type, not_null, pk) in &columns {
            schema_str.push_str(&format!("  {} {}",name, col_type));
            if *pk { schema_str.push_str(" PRIMARY KEY"); }
            if *not_null { schema_str.push_str(" NOT NULL"); }
            schema_str.push('\n');
        }
        for (from, to_table, to_col) in &foreign_keys {
            schema_str.push_str(&format!("  {} REFERENCES {}({})\n", from, to_table, to_col));
        }
        // try to fetch one sample row from the table
        let sample_row = sqlx::query(&format!("SELECT * FROM {} LIMIT 1", payload.table))
            .fetch_optional(pool)
            .await
            .ok()
            .flatten();

        let mut sample_str = String::new();
        if let Some(row) = sample_row {
            let mut parts: Vec<String> = Vec::new();
            for (name, _col_type, _not_null, _pk) in &columns {
                // try several common types and fall back to NULL
                let val = row.try_get::<String, &str>(name)
                    .map(|s| s)
                    .or_else(|_| row.try_get::<i64, &str>(name).map(|v| v.to_string()))
                    .or_else(|_| row.try_get::<f64, &str>(name).map(|v| v.to_string()))
                    .unwrap_or_else(|_| "NULL".to_string());
                parts.push(format!("{}={}", name, val));
            }
            sample_str = parts.join(", ");
        }

        // append sample row info to schema string for prompt clarity
        if !sample_str.is_empty() {
            schema_str.push_str(&format!("\nSample row: {}\n", sample_str));
        }
        schema_str
    } else {
        String::new()
    };
    drop(db);

    let client = Client::new();
    let prompt = format!(
        "Correct/Translate/Complete the User Input to SQL using SQLite dialect.\n\
        Table: {}\n\
        Schema:\n{}\n\
        User Input: {}\n\n\
        If user intent is ambiguous, return a simple SELECT * FROM <table> LIMIT 10.",
        payload.table, schema, payload.sql
    );
    tracing::debug!("AI API prompt: {}", prompt);

    let response = client.post(format!("{}/chat/completions", base_url))
        .bearer_auth(api_key.unwrap())
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "system", "content": "You are a SQL expert. Always reply a single raw SQL string without any format."},
                {"role": "user", "content": prompt}
            ],
            "reasoning_effort": "low"
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let body: Value = resp.json().await.unwrap_or(json!({}));
                let content = body["choices"][0]["message"]["content"].as_str().unwrap_or("");
                tracing::info!("AI API response: {}", content);
                
                Json(CorrectResponse {
                    sql: Some(content.to_string()),
                    error: None,
                })
            } else {
                Json(CorrectResponse {
                    sql: None,
                    error: Some(format!("AI API returned error: {}", resp.status())),
                })
            }
        }
        Err(e) => {
            Json(CorrectResponse {
                sql: None,
                error: Some(e.to_string()),
            })
        }
    }
}
