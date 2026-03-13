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
    pub error: String,
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
    let api_base = std::env::var("AI_API_BASE").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let model = std::env::var("AI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

    if api_key.is_empty() {
        return Json(CorrectSqlResponse {
            corrected_sql: None,
            error: Some("AI_API_KEY not configured".to_string()),
        });
    }

    let client = Client::new();
    let prompt = format!(
        "The following SQLite query failed with an error:\n\nQuery: {}\nError: {}\n\nPlease provide only the corrected SQL query that fixes this error. Do not include any explanation or markdown formatting.",
        payload.sql, payload.error
    );

    let response = client.post(format!("{}/chat/completions", api_base))
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "system", "content": "You are a SQLite expert. You only output corrected SQL queries without any markdown blocks or explanations."},
                {"role": "user", "content": prompt}
            ],
            "temperature": 0.0
        }))
        .send()
        .await;

    match response {
        Ok(res) => {
            let res_json: serde_json::Value = res.json().await.unwrap_or_default();
            let corrected = res_json["choices"][0]["message"]["content"]
                .as_str()
                .map(|s| s.trim().trim_matches('`').trim_start_matches("sql").trim().to_string());

            Json(CorrectSqlResponse {
                corrected_sql: corrected,
                error: None,
            })
        },
        Err(e) => Json(CorrectSqlResponse {
            corrected_sql: None,
            error: Some(e.to_string()),
        }),
    }
}
