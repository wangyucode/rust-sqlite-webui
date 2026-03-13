use axum::{
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use reqwest::Client;

#[derive(Deserialize)]
pub struct CorrectRequest {
    pub sql: String,
    pub error: String,
}

#[derive(Serialize)]
pub struct CorrectResponse {
    pub corrected_sql: Option<String>,
    pub explanation: Option<String>,
    pub error: Option<String>,
}

pub async fn correct_sql(
    Json(payload): Json<CorrectRequest>,
) -> impl IntoResponse {
    let api_key = std::env::var("OPENAI_API_KEY");
    let base_url = std::env::var("OPENAI_BASE_URL").unwrap_or_else(|_| "https://api.openai.com/v1".to_string());
    let model = std::env::var("OPENAI_MODEL").unwrap_or_else(|_| "gpt-3.5-turbo".to_string());

    if api_key.is_err() {
        return Json(CorrectResponse {
            corrected_sql: None,
            explanation: None,
            error: Some("OPENAI_API_KEY not configured".to_string()),
        });
    }

    let client = Client::new();
    let prompt = format!(
        "The following SQLite query failed:\n\nquery: {}\n\nerror: {}\n\nPlease correct the SQL query. Provide only the corrected SQL and a brief explanation. Return a JSON object with keys 'corrected_sql' and 'explanation'.",
        payload.sql, payload.error
    );

    let response = client.post(format!("{}/chat/completions", base_url))
        .bearer_auth(api_key.unwrap())
        .json(&json!({
            "model": model,
            "messages": [
                {"role": "system", "content": "You are a helpful SQL expert assistant. Always return JSON."},
                {"role": "user", "content": prompt}
            ],
            "response_format": { "type": "json_object" }
        }))
        .send()
        .await;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                let body: Value = resp.json().await.unwrap_or(json!({}));
                let content = body["choices"][0]["message"]["content"].as_str().unwrap_or("{}");
                let parsed: Value = serde_json::from_str(content).unwrap_or(json!({}));
                
                Json(CorrectResponse {
                    corrected_sql: parsed["corrected_sql"].as_str().map(|s| s.to_string()),
                    explanation: parsed["explanation"].as_str().map(|s| s.to_string()),
                    error: None,
                })
            } else {
                Json(CorrectResponse {
                    corrected_sql: None,
                    explanation: None,
                    error: Some(format!("AI API returned error: {}", resp.status())),
                })
            }
        }
        Err(e) => {
            Json(CorrectResponse {
                corrected_sql: None,
                explanation: None,
                error: Some(e.to_string()),
            })
        }
    }
}
