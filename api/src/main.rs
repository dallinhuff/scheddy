use crate::server::{Config, Server};
use std::error::Error;

mod routes;
mod server;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let server = Server::new(Config { port: "8080" }).await?;
    server.run().await?;
    Ok(())
}
