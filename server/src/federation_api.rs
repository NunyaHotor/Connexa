use axum::{
    extract::{State, Json},
    routing::post,
    Router,
};
use std::sync::Arc;
use crate::federation::{FederationState, Server};
use crate::relay::MessageStore;
use crate::connexa::EncryptedMessage;

#[derive(Clone)]
pub struct FederationAppState {
    pub federation: Arc<FederationState>,
    pub store: Arc<MessageStore>,
}

pub async fn register_server(
    State(state): State<FederationAppState>,
    Json(server): Json<Server>,
) -> Result<(), String> {
    let mut guard = state.federation.servers.write().await;
    guard.push(server);
    Ok(())
}

pub async fn federation_message(
    State(state): State<FederationAppState>,
    Json(message): Json<EncryptedMessage>,
) -> Result<(), String> {
    state
        .store
        .store_message(&message.target_device_id, message, true)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn federation_router(federation_state: Arc<FederationState>, store: Arc<MessageStore>) -> Router {
    let state = FederationAppState { federation: federation_state, store };
    Router::new()
        .route("/federation/register", post(register_server))
        .route("/federation/message", post(federation_message))
        .with_state(state)
}