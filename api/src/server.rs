use crate::routes::api_routes;
use crate::state::AppState;
use axum::extract::Request;
use axum::Router;
use std::error::Error;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config<'a> {
    pub port: &'a str,
}

#[derive(Debug)]
pub struct Server {
    router: Router,
    listener: TcpListener,
}

impl Server {
    pub async fn new(config: Config<'_>) -> std::io::Result<Self> {
        let trace_layer = TraceLayer::new_for_http().make_span_with(|request: &Request<_>| {
            let uri = request.uri().to_string();
            tracing::info_span!("http_request", method = ?request.method(), uri)
        });

        let state = AppState {};

        let router = Router::new()
            .nest("/api", api_routes())
            .layer(trace_layer)
            .with_state(state);

        let listener = TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> Result<(), Box<dyn Error>> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router).await?;
        Ok(())
    }
}
