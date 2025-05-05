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

#[derive(Debug, Deserialize, Serialize)]
pub struct GoogleTokenResponse {
    pub access_token: String,
    pub expires_in: u64,
    pub refresh_token: Option<String>,
    pub scope: String,
    pub token_type: String,
    pub id_token: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct GoogleUserInfo {
    pub email: String,
    pub name: String,
    pub given_name: String,
    pub family_name: String,
    pub picture: Option<String>,
    pub email_verified: bool,
    pub locale: Option<String>,
}

