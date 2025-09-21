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
use crate::federation_api::federation_router;
use crate::federation::FederationState;
use crate::p2p::{MyBehaviour, MyBehaviourEvent};
use parking_lot::Mutex;
use tokio::time::{interval, Duration};
use libp2p::{
    core::upgrade,
    identity,
    mplex,
    noise,
    swarm::{SwarmBuilder},
    tcp::TokioTcpConfig,
    Transport,
};
use libp2p::kad::{Kademlia, store::MemoryStore};
use libp2p_gossipsub::{Gossipsub, MessageAuthenticity};

use tokio::sync::mpsc;

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
mod federation;
mod federation_api;
mod p2p;
mod decentralized_storage;
mod psi;

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
    let federation_state = Arc::new(FederationState::new());
    let storage = Arc::new(Storage::new());
    let oprf_key = Arc::new(psi::generate_server_oprf_key());

    let (message_queue_sender, mut message_queue_receiver) = mpsc::channel(100);

    let local_keys = identity::Keypair::generate_ed25519();
    let local_peer_id = local_keys.public().to_peer_id();

    let transport = TokioTcpConfig::new()
        .nodelay(true)
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseAuthenticated::xx(&local_keys).unwrap())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let behaviour = MyBehaviour {
        kademlia: Kademlia::new(local_peer_id, MemoryStore::new(local_peer_id.clone())),
        gossipsub: Gossipsub::new(MessageAuthenticity::Signed(local_keys.clone()), Default::default()).unwrap(),
    };

    let swarm = Arc::new(Mutex::new(swarm));

    tokio::spawn(async move {
        p2p::new(swarm.clone(), store.clone()).await.unwrap();
    });

    let app = app(store.clone(), federation_state.clone(), swarm.clone(), storage.clone(), oprf_key.clone(), message_queue_sender.clone())
        .merge(group_router(group_state))
        .merge(group_message_router(group_sessions))
        .merge(device_router(devices, pool))
        .merge(sfu_router())
        .merge(webrtc_router())
        .merge(federation_router(federation_state.clone(), store.clone()));

    // Spawn background cleanup task
    let store_clone = store.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            store_clone.cleanup_expired().await;
        }
    });

    // Message batching and sending task
    let store_clone_for_batching = store.clone();
    tokio::spawn(async move {
        let mut batch_interval = interval(Duration::from_secs(5)); // Send every 5 seconds
        let mut message_batch: Vec<EncryptedMessage> = Vec::new();
        loop {
            tokio::select! {
                _ = batch_interval.tick() => {
                    if !message_batch.is_empty() {
                        println!("Sending batch of {} messages", message_batch.len());
                        for message in message_batch.drain(..) {
                            // Here you would implement the logic to send the batched message
                            // For now, we just store it in the message store
                            store_clone_for_batching.store_message(message).await.unwrap();
                        }
                    }
                }
                Some(message) = message_queue_receiver.recv() => {
                    message_batch.push(message);
                }
            }
        }
    });

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}