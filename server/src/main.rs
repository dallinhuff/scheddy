//! [server][crate] is an executable entry point to configure and start the scheddy
//! application. It selects, provides, and injects the concrete [infrastructure]
//! adapters needed into the [application]'s provided ports in order to run the
//! application as a single executable webserver.
//!
//! [infrastructure] ../infrastructure
//! [application] ../application

#![warn(clippy::pedantic)]

use infrastructure::postgres::Postgres;

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let db_conn = std::env::var("DB_URL")
        .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string());
    let host = std::env::var("HOST").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("4000".to_string());

    let db = Postgres::new(&db_conn).await?;
    db.migrate().await?;

    let app = routes::routes();
    let listener = tokio::net::TcpListener::bind(format!("{host}:{port}")).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
