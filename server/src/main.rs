use axum::Server;
use std::sync::Arc;
use std::time::Duration;

use tokio::time::interval;
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
use crate::p2p::{MyBehaviour};
use crate::decentralized_storage::Storage;
use crate::connexa::EncryptedMessage;
use libp2p::{
    gossipsub,
    identity,
    kad,
    noise,
    tcp,
    yamux,
    PeerId,
    SwarmBuilder
};
use tokio::sync::mpsc;
use sqlx::SqlitePool;
use tokio::sync::Mutex;

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
mod config;

pub mod connexa {
    tonic::include_proto!("connexa");
}

#[tokio::main]
async fn main() {
    let store = Arc::new(MessageStore::new());
    let group_state = Arc::new(Mutex::new(Vec::new()));
    let group_sessions = Arc::new(Mutex::new(Vec::<GroupSession>::new()));
    let devices = Arc::new(Mutex::new(Vec::<Device>::new()));
    let config = config::Config::from_env();
    let pool = SqlitePool::connect(&config.database_url).await.unwrap();
    let federation_state = Arc::new(FederationState::new());
    let storage = Arc::new(Storage::new());
    let oprf_key = Arc::new(psi::generate_server_oprf_key());

    let (message_queue_sender, mut message_queue_receiver) = mpsc::channel(100);

    let local_keys = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_keys.public());

    let mut swarm = SwarmBuilder::with_existing_identity(local_keys.clone())
        .with_tokio()
        .with_tcp(
            tcp::Config::default(),
            noise::Config::new,
            yamux::Config::default,
        )
        .unwrap()
        .with_behaviour(|key| {
            let gossipsub = gossipsub::Gossipsub::new(
                gossipsub::MessageAuthenticity::from(key.public()),
                gossipsub::GossipsubConfig::default(),
            )
            .unwrap();
            let kademlia = kad::Kademlia::new(local_peer_id, kad::store::MemoryStore::new(local_peer_id.clone()));
            MyBehaviour { kademlia, gossipsub }
        })
        .unwrap()
        .build();

    // Start listening on configured address for libp2p and subscribe to configured gossip topics
    swarm
        .listen_on(config.p2p_listen.parse().expect("valid multiaddr"))
        .expect("start listening");
    for t in &config.gossip_topics {
        let topic = gossipsub::IdentTopic::new(t);
        swarm
            .behaviour_mut()
            .gossipsub
            .subscribe(&topic)
            .expect("subscribe to topic");
    }
    println!("Local PeerId: {}", local_peer_id);

    let swarm = Arc::new(Mutex::new(swarm));

    tokio::spawn(async move {
        p2p::new(swarm.clone()).await.unwrap();
    });

    let app = app(
        store.clone(),
        federation_state.clone(),
        swarm.clone(),
        storage.clone(),
        oprf_key.clone(),
        message_queue_sender.clone(),
        group_state.clone()
    )
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
                            store_clone_for_batching.store_message("recipient_id", message, true).await.unwrap();
                        }
                    }
                }
                Some(message) = message_queue_receiver.recv() => {
                    message_batch.push(message);
                }
            }
        }
    });

    let addr = config.http_addr;
    println!("HTTP listening on {}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}