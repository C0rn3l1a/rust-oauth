use axum::Router;

mod google;

// Auth Routes
pub fn routes() -> Router {
    Router::new()
        .nest("/google", google::routes())
}  
