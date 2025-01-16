mod routes;
mod server;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let server = server::Server::new(server::Config { port: "8080" }).await?;
    server.run().await?;
    Ok(())
}
