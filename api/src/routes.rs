use crate::state::AppState;

pub fn api_routes() -> axum::Router<AppState> {
    axum::Router::new().nest("/health", health_routes())
}

pub fn health_routes() -> axum::Router<AppState> {
    axum::Router::new().route("/", axum::routing::get(|| async { "Healthy" }))
}
