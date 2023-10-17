use smart_house::api;
use smart_house::repository::room::InMemoryRepository;
use std::net::TcpListener;
use std::sync::Arc;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let repo = Arc::new(InMemoryRepository::new());
    let listener = TcpListener::bind("127.0.0.1:8888").expect("Unable to bind to port");
    api::spawn(listener, repo)?.await
}
