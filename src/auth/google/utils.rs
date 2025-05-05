use std::env;
use urlencoding::encode;
use axum::{
    extract::Query,
    response::{IntoResponse, Response},
    http::StatusCode,
    Json,
};
use std::collections::HashMap;
use reqwest::Client;
use serde::{Deserialize, Serialize};

pub async fn exchange_code_for_token(code: &str) -> Result<GoogleTokenResponse, String> {
    let client_id = env::var("GCP_CLIENT_ID").map_err(|_| "Missing GCP_CLIENT_ID")?;
    let client_secret = env::var("GCP_CLIENT_SECRET").map_err(|_| "Missing GCP_CLIENT_SECRET")?;
    let redirect_uri = env::var("GCP_REDIRECT_URI").map_err(|_| "Missing GCP_REDIRECT_URI")?;

    let params = [
        ("code", code),
        ("client_id", &client_id),
        ("client_secret", &client_secret),
        ("redirect_uri", &redirect_uri),
        ("grant_type", "authorization_code"),
    ];

    let client = Client::new();
    let res = client
        .post("https://oauth2.googleapis.com/token")
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("HTTP request failed: {}", e))?;

    if !res.status().is_success() {
        let text = res.text().await.unwrap_or_default();
        return Err(format!("Token exchange failed: {}", text));
    }

    let token_response = res
        .json::<GoogleTokenResponse>()
        .await
        .map_err(|e| format!("Failed to parse response: {}", e))?;

    Ok(token_response)
}
