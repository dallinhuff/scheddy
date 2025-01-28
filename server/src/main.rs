//! [server][crate] is an executable entry point to configure and start the scheddy
//! application. It selects, provides, and injects the concrete [infrastructure]
//! adapters needed into the [application]'s provided ports in order to run the
//! application as a single executable webserver.
//!
//! [infrastructure] ../infrastructure
//! [application] ../application

mod routes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = routes::routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;

    Ok(())
}
