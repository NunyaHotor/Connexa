use axum::{
    extract::{State, Json, Path},
    routing::{get, post},
    Router,
    body::Bytes,
};
use crate::relay::{MessageStore};
use crate::connexa::EncryptedMessage;
use std::sync::Arc;
use crate::auth::AuthenticatedUser;
use crate::p2p::MyBehaviour;
use libp2p::swarm::Swarm;
use parking_lot::Mutex;
use libp2p::PeerId;
use std::str::FromStr;
use libp2p_gossipsub::IdentTopic as Topic;
use tokio::sync::mpsc;
use uuid::Uuid;
use serde::{Deserialize, Serialize};

use crate::group::Group;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<MessageStore>,
    pub federation_state: Arc<FederationState>,
    pub swarm: Arc<Mutex<Swarm<MyBehaviour>>>,
    pub storage: Arc<Storage>,
    pub oprf_key: Arc<EphemeralSecret>,
    pub message_queue_sender: mpsc::Sender<EncryptedMessage>,
    pub groups: Arc<Mutex<Vec<Group>>>,
}

#[derive(Deserialize)]
pub struct SendMessagePayload {
    pub recipient_user_id: String,
    pub plaintext: Vec<u8>,
}

pub async fn post_message(
    State(state): State<AppState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<SendMessagePayload>,
) -> Result<(), String> {
    let recipient_user_id = payload.recipient_user_id;

    // Check if the recipient is a group
    if let Ok(group_id) = Uuid::from_str(&recipient_user_id) {
        let groups = state.groups.lock();
        if let Some(group) = groups.iter().find(|g| g.id == group_id) {
            // If it's a group, publish to gossipsub topic for the group
            let topic = Topic::new(format!("connexa-group-{}", group_id));
            state.swarm.lock().behaviour_mut().publish_message(&topic, payload.plaintext)?;
            return Ok(());
        }
    }

    if let Ok(peer_id) = PeerId::from_str(&recipient_user_id) {
        if state.swarm.lock().is_connected(&peer_id) {
            let topic = Topic::new("connexa-messages");
            state.swarm.lock().behaviour_mut().publish_message(&topic, payload.plaintext)?;
        } else {
            // Not connected to peer, fallback to store and forward
            let message = EncryptedMessage {
                sender_id: user_id.to_string(),
                recipient_id: recipient_user_id,
                ciphertext: payload.plaintext,
                timestamp: chrono::Utc::now().timestamp() as u64,
            };
            state.message_queue_sender.send(message).await.map_err(|e| e.to_string())?;
        }
    } else if recipient_user_id.contains("@") {
        let parts: Vec<&str> = recipient_user_id.split("@").collect();
        let server_name = parts[1];
        let server = state.federation_state.servers.iter().find(|s| s.name == server_name);
        if let Some(server) = server {
            let message = EncryptedMessage {
                sender_id: user_id.to_string(),
                recipient_id: recipient_user_id,
                ciphertext: payload.plaintext,
                timestamp: chrono::Utc::now().timestamp() as u64,
            };
            send_federated_message(server, &message).await.map_err(|e| e.to_string())?;
        } else {
            return Err("Server not found".to_string());
        }
    } else {
        let message = EncryptedMessage {
            sender_id: user_id.to_string(),
            recipient_id: recipient_user_id,
            ciphertext: payload.plaintext,
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        state.message_queue_sender.send(message).await.map_err(|e| e.to_string())?;
    }
    Ok(())
}

pub async fn get_messages(
    State(state): State<AppState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> Result<Json<Vec<EncryptedMessage>>, String> {
    let mut msgs = state
        .store
        .fetch_messages(&user_id.to_string(), true)
        .await
        .map_err(|e| e.to_string())?;

    // Fetch messages for groups the user is a member of
    let groups = state.groups.lock();
    for group in groups.iter() {
        if group.members.contains(&user_id.to_string()) {
            let group_msgs = state.store.fetch_messages(&group.id.to_string(), true).await.map_err(|e| e.to_string())?;
            msgs.extend(group_msgs);
        }
    }

    Ok(Json(msgs))
}

pub async fn upload_media(
    State(state): State<AppState>,
    body: Bytes,
) -> Result<String, String> {
    let hash = state.storage.add_file(body.to_vec()).await?;
    Ok(hash)
}

pub async fn download_media(
    State(state): State<AppState>,
    Path(hash): Path<String>,
) -> Result<Vec<u8>, String> {
    let content = state.storage.get_file(&hash).await?;
    Ok(content)
}

#[derive(Debug)]
pub struct BlindedElementPayload {
    pub element: PublicKey,
}

pub async fn blind_element(
    Json(payload): Json<BlindedElementPayload>,
) -> Result<Json<BlindedElementPayload>, String> {
    let (client_secret, blinded_element) = psi::blind(&payload.element.to_bytes());
    Ok(Json(BlindedElementPayload { element: blinded_element }))
}

pub async fn evaluate_element(
    State(state): State<AppState>,
    Json(payload): Json<BlindedElementPayload>,
) -> Result<Json<BlindedElementPayload>, String> {
    let evaluated_element = psi::evaluate(&state.oprf_key, &payload.element);
    Ok(Json(BlindedElementPayload { element: evaluated_element }))
}

pub fn app(store: Arc<MessageStore>, federation_state: Arc<FederationState>, swarm: Arc<Mutex<Swarm<MyBehaviour>>>, storage: Arc<Storage>, oprf_key: Arc<EphemeralSecret>, message_queue_sender: mpsc::Sender<EncryptedMessage>, groups: Arc<Mutex<Vec<Group>>>) -> Router {
    Router::new()
        .route("/message", post(post_message))
        .route("/messages/:recipient_id", get(get_messages))
        .route("/media/upload", post(upload_media))
        .route("/media/download/:hash", get(download_media))
        .route("/psi/blind", post(blind_element))
        .route("/psi/evaluate", post(evaluate_element))
        .with_state(AppState { store, federation_state, swarm, storage, oprf_key, message_queue_sender, groups })
}
