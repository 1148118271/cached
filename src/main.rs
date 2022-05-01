
#[tokio::main]
async fn main() {
    local_log::enable_logging();
    let server = server::Server::new().await.unwrap();
    server.run().await.unwrap();
}