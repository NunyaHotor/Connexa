use axum::Server;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::server::relay::MessageStore;
use crate::server::api::app;

#[tokio::main]
async fn main() {
    let store = Arc::new(MessageStore::new());
    let app = app(store);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}