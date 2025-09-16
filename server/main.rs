use axum::Server;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::server::relay::MessageStore;
use crate::server::api::app;
use crate::server::group_api::group_router;
use crate::server::group_message_api::group_message_router;
use crate::server::group_mls::GroupSession;
use crate::server::device_api::device_router;
use crate::server::device::Device;
use parking_lot::Mutex;

#[tokio::main]
async fn main() {
    let store = Arc::new(MessageStore::new());
    let group_state = Arc::new(Mutex::new(Vec::new()));
    let group_sessions = Arc::new(Mutex::new(Vec::<GroupSession>::new()));
    let devices = Arc::new(Mutex::new(Vec::<Device>::new()));

    let app = app(store)
        .merge(group_router(group_state))
        .merge(group_message_router(group_sessions))
        .merge(device_router(devices));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}