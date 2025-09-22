use axum::{
    extract::{State, Json},
    routing::post,
    Router,
};
use std::sync::Arc;
use crate::federation::{FederationState, Server};
use crate::relay::MessageStore;
use crate::connexa::EncryptedMessage;

pub async fn register_server(
    State(federation_state): State<Arc<FederationState>>,
    Json(server): Json<Server>,
) -> Result<(), String> {
    federation_state.servers.push(server);
    Ok(())
}

pub async fn federation_message(
    State(store): State<Arc<MessageStore>>,
    Json(message): Json<EncryptedMessage>,
) -> Result<(), String> {
    store.store_message(&message.recipient_id, message, true).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub fn federation_router(federation_state: Arc<FederationState>, store: Arc<MessageStore>) -> Router {
    Router::new()
        .route("/federation/register", post(register_server))
        .with_state(federation_state)
        .route("/federation/message", post(federation_message))
        .with_state(store)
}
