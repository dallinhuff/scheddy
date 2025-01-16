use crate::{routes, state};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Config<'a> {
    pub port: &'a str,
}

#[derive(Debug)]
pub struct Server {
    router: axum::Router,
    listener: tokio::net::TcpListener,
}

impl Server {
    pub async fn new(config: Config<'_>) -> std::io::Result<Self> {
        let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
            |request: &axum::extract::Request<_>| {
                let uri = request.uri().to_string();
                tracing::info_span!("http_request", method = ?request.method(), uri)
            },
        );

        let state = state::AppState {};

        let router = axum::Router::new()
            .nest("/api", routes::api_routes())
            .layer(trace_layer)
            .with_state(state);

        let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port)).await?;

        Ok(Self { router, listener })
    }

    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router).await?;
        Ok(())
    }
}
