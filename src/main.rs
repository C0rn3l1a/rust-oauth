extern crate dotenv;

use axum::{
    routing::get,
    Router,
};
use dotenv::dotenv;
use std::{env, u16};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{filter, EnvFilter, Layer};

mod auth;

#[tokio::main]
async fn main() {
    dotenv().ok();
    // initialize tracing
    setup_logger();
    startup_message();

    // Basic Routes
    let app = Router::new()
        .route("/", get(health))
        .nest("/auth", auth::routes());

    // define listener
    let port = env::var("PORT")
        .unwrap_or(String::from("3000"))
        .parse::<u16>()
        .unwrap_or(3000);

    // initialize server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await.unwrap();

    axum::serve(listener, app.into_make_service()).await.unwrap();
}

fn setup_logger() {
    let logger = tracing_subscriber::fmt::layer().with_filter(filter::LevelFilter::DEBUG);

    tracing_subscriber::registry()
        .with(logger)
        .with(EnvFilter::from_env("LOG_LEVEL"))
        .init();
}

fn startup_message() {
    let port = env::var("PORT").unwrap_or(String::from("3000"));
    let log_level = env::var("LOG_LEVEL").unwrap_or(String::from("None"));

    println!("---");
    println!("\x1b[32mServer started:\x1b[0m");
    println!("- PORT: \x1b[1m\x1b[33m{port}\x1b[0m");
    println!("- LOG_LEVEL: \x1b[1m\x1b[33m{log_level}\x1b[0m");
    println!("---");
}

// basic handler that responds with a static string
async fn health() -> &'static str {
    "healthy"
}
