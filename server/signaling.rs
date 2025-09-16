use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    extract::{State, TypedHeader},
    headers::Authorization,
    http::Request,
    routing::get,
    Router,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, RwLock};
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::Deserialize as SerdeDeserialize;

#[derive(Serialize, Deserialize)]
pub struct SignalMessage {
    pub from: String,
    pub to: String,         // For direct signaling, can be empty for group
    pub msg_type: String,   // "offer", "answer", "ice", "join"
    pub payload: String,
    pub room_id: Option<String>, // NEW: for group calls
}

#[derive(Clone)]
pub struct SignalingState {
    pub peers: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>,
    pub tx: broadcast::Sender<String>,
}

#[derive(Clone)]
pub struct GroupCallState {
    pub rooms: Arc<RwLock<HashMap<String, HashSet<String>>>>, // room_id -> set of user_ids
    pub peers: Arc<RwLock<HashMap<String, mpsc::UnboundedSender<Message>>>>,
    pub tx: broadcast::Sender<String>,
}

// JWT claims struct
#[derive(Debug, SerdeDeserialize)]
pub struct Claims {
    pub sub: String, // user_id
    pub exp: usize,
}

// Extract user_id from JWT in the Sec-WebSocket-Protocol header (or Authorization header)
async fn extract_user_id<B>(req: &Request<B>) -> Result<String, String> {
    if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if let Some(token) = auth_str.strip_prefix("Bearer ") {
                let key = DecodingKey::from_secret(b"your-secret-key"); // Use your real secret!
                let validation = Validation::new(Algorithm::HS256);
                let token_data = decode::<Claims>(token, &key, &validation)
                    .map_err(|e| format!("JWT decode error: {e}"))?;
                return Ok(token_data.claims.sub);
            }
        }
    }
    Err("Missing or invalid Authorization header".to_string())
}

pub async fn signaling_ws(
    ws: WebSocketUpgrade,
    State(state): State<SignalingState>,
    req: Request<()>,
) -> impl IntoResponse {
    match extract_user_id(&req).await {
        Ok(user_id) => ws.on_upgrade(move |socket| handle_socket(socket, state, user_id)),
        Err(e) => (
            axum::http::StatusCode::UNAUTHORIZED,
            format!("WebSocket auth failed: {e}"),
        )
            .into_response(),
    }
}

async fn handle_socket(mut socket: WebSocket, state: SignalingState, user_id: String) {
    let (tx, mut rx) = mpsc::unbounded_channel();
    let mut broadcast_rx = state.tx.subscribe();

    // Register the authenticated user
    state.peers.write().await.insert(user_id.clone(), tx);

    loop {
        tokio::select! {
            Some(Ok(msg)) = socket.recv() => {
                if let Message::Text(text) = msg {
                    if let Ok(signal) = serde_json::from_str::<SignalMessage>(&text) {
                        if let Some(room_id) = &signal.room_id {
                            if signal.msg_type == "join" {
                                join_room(&state, room_id, &user_id).await;
                            } else {
                                broadcast_to_room(&state, room_id, &user_id, &signal).await;
                            }
                        } else if !signal.to.is_empty() {
                            // Direct signaling (1:1)
                            if let Some(peer_tx) = state.peers.read().await.get(&signal.to) {
                                let _ = peer_tx.send(Message::Text(text));
                            }
                        }
                    }
                }
            }
            Ok(msg) = broadcast_rx.recv() => {
                let _ = socket.send(Message::Text(msg)).await;
            }
        }
    }
}

pub fn signaling_router(state: SignalingState) -> Router {
    Router::new().route("/signaling", get(signaling_ws)).with_state(state)
}

// Suggested code change: ICE server configuration
// This configuration should be used on the client-side, typically in the WebRTC configuration.
// For example:
// const iceServers = [
//   { urls: "stun:your.turn.server:3478" },
//   { urls: "turn:your.turn.server:3478", username: "user", credential: "pass" }
// ];
//
// TURN server configuration (for server-side)
// Add this to your TURN server configuration file (e.g., turnserver.conf):
// listening-port=3478
// fingerprint
// lt-cred-mech
// use-auth-secret
// static-auth-secret=YOUR_SECRET_KEY
// realm=yourdomain.com
// total-quota=100
// bps-capacity=0
// stale-nonce
// cert=/path/to/your/fullchain.pem   # Optional, for TLS
// pkey=/path/to/your/privkey.pem     # Optional, for TLS

// Add a function to join a room
pub async fn join_room(state: &GroupCallState, room_id: &str, user_id: &str) {
    let mut rooms = state.rooms.write().await;
    rooms.entry(room_id.to_string()).or_default().insert(user_id.to_string());
}

// Add a function to broadcast signaling messages to all users in a room (except sender)
pub async fn broadcast_to_room(
    state: &GroupCallState,
    room_id: &str,
    from_user: &str,
    msg: &SignalMessage,
) {
    let rooms = state.rooms.read().await;
    if let Some(users) = rooms.get(room_id) {
        let peers = state.peers.read().await;
        for user in users {
            if user != from_user {
                if let Some(peer_tx) = peers.get(user) {
                    let _ = peer_tx.send(Message::Text(serde_json::to_string(msg).unwrap()));
                }
            }
        }
    }
}