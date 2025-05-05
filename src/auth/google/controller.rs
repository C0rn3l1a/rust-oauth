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

pub async fn redirect(Query(params): Query<HashMap<String, String>>) -> Response {
    match params.get("code") {
        Some(code) => match exchange_code_for_token(code).await {
            Ok(tokens) => Json(tokens).into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Token exchange error: {}", e),
                )
                .into_response(),
        },
        None => (
            StatusCode::BAD_REQUEST,
            "Missing 'code' parameter".to_string(),
            )
            .into_response(),
    }
}

pub async fn login() -> String {
    // TODO: handle env vars not existing
    let client_id = env::var("GCP_CLIENT_ID").unwrap_or(String::from("NONE"));
    let redirect_uri = env::var("GCP_REDIRECT_URI").unwrap_or(String::from("NONE"));

    let encoded_redirect_uri = encode(&redirect_uri);

    let scope = [
        "https://www.googleapis.com/auth/userinfo.email",
        "https://www.googleapis.com/auth/userinfo.profile",
    ].join(" ");

    let encoded_scope = encode(&scope);

    return format!("https://accounts.google.com/o/oauth2/v2/auth?\
         scope={encoded_scope}&\
         access_type=offline&\
         include_granted_scopes=true&\
         response_type=code&\
         state=state_parameter_passthrough_value&\
         redirect_uri={encoded_redirect_uri}&\
         client_id={client_id}");
}
