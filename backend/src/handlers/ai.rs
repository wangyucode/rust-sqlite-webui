use axum::{
    extract::State,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::state::AppState;
use reqwest::Client;
use serde_json::json;

#[derive(Deserialize)]
pub struct CorrectSqlRequest {
    pub sql: String,
}

#[derive(Serialize)]
pub struct CorrectSqlResponse {
    pub corrected_sql: Option<String>,
    pub error: Option<String>,
}

pub async fn correct_sql(
    State(_state): State<AppState>,
    Json(payload): Json<CorrectSqlRequest>,
) -> impl IntoResponse {
    let api_key = std::env::var("AI_API_KEY").unwrap_or_default();
    let base_url = std::env::var("AI_BASE_URL").unwrap_or_else(|_| "https://api.minimax.chat/v1".to_string());
    let model = std::env::var("AI_MODEL").unwrap_or_else(|_| "minimax-m2.5".to_string());

    if api_key.is_empty() {
        return Json(CorrectSqlResponse {
            corrected_sql: None,
            error: Some("AI_API_KEY is not set".to_string()),
        });
    }

    let client = Client::new();
    let prompt = format!(
        "You are a SQLite expert. I will give you a SQL statement that might have syntax errors or issues. \
        Please correct it so it runs perfectly on SQLite. \
        Only return the corrected SQL code, no explanations or markdown blocks.\n\n\
        SQL: {}",
        payload.sql
    );

    let response = client.post(format!("{}/chat/completions", base_url))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.3
        }))
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let body: serde_json::Value = res.json().await.unwrap_or_default();
                let corrected_sql = body["choices"][0]["message"]["content"]
                    .as_str()
                    .map(|s| s.trim().trim_matches('`').to_string());
                
                Json(CorrectSqlResponse {
                    corrected_sql,
                    error: None,
                })
            } else {
                let err_msg = res.text().await.unwrap_or_else(|_| "Unknown error".to_string());
                Json(CorrectSqlResponse {
                    corrected_sql: None,
                    error: Some(format!("AI API error: {}", err_msg)),
                })
            }
        },
        Err(e) => Json(CorrectSqlResponse {
            corrected_sql: None,
            error: Some(format!("Request failed: {}", e)),
        }),
    }
}
