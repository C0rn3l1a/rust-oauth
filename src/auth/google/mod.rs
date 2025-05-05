use axum::{routing::get, Router};

mod controller;
mod types;
mod utils;

// Auth Routes
pub fn routes() -> Router {
    Router::new()
        .route("/redirect", get(controller::redirect))
        .route("/login", get(controller::login))
}  
