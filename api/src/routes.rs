use crate::state::AppState;
use axum::Router;

mod booking;
mod health;
mod scheduling;

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .nest("/health", health::routes().with_state(()))
        .nest("/booking", booking::routes())
        .nest("/scheduling", scheduling::routes())
}
