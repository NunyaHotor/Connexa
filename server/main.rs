use axum::Server;
use std::net::SocketAddr;
use std::sync::Arc;
use crate::relay::MessageStore;
use crate::api::app;
use crate::group_api::group_router;
use crate::group_message_api::group_message_router;
use crate::group_mls::GroupSession;
use crate::device_api::device_router;
use crate::device::Device;
use crate::sfu_api::sfu_router;
use crate::webrtc::webrtc_router;
use parking_lot::Mutex;
use tokio::time::{interval, Duration};

use sqlx::SqlitePool;

mod api;
mod auth;
mod device;
mod device_api;
mod group;
mod group_api;
mod group_message_api;
mod group_mls;
mod messaging;
mod relay;
mod sfu_api;
mod crypto;
mod webrtc;
pub mod connexa {
    tonic::include_proto!("connexa");
}


#[tokio::main]
async fn main() {
    let store = Arc::new(MessageStore::new());
    let group_state = Arc::new(Mutex::new(Vec::new()));
    let group_sessions = Arc::new(Mutex::new(Vec::<GroupSession>::new()));
    let devices = Arc::new(Mutex::new(Vec::<Device>::new()));
    let pool = SqlitePool::connect("sqlite:connexa.db").await.unwrap();

    let app = app(store.clone())
        .merge(group_router(group_state))
        .merge(group_message_router(group_sessions))
        .merge(device_router(devices, pool))
        .merge(sfu_router())
        .merge(webrtc_router());

    // Spawn background cleanup task
    let store_clone = store.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            store_clone.cleanup_expired().await;
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}