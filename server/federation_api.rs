use axum::{
    extract::{State, Json},
    routing::{get, post},
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
    // In a real implementation, we would verify the server is reachable
    // and that it is a valid Connexa server.
    // For now, we will just add it to the list of servers.
    federation_state.servers.push(server);
    Ok(())
}

pub async fn receive_federated_message(
    State(store): State<Arc<MessageStore>>,
    Json(message): Json<EncryptedMessage>,
) -> Result<(), String> {
    store.store_message(message).await.map_err(|e| e.to_string())?;
    Ok(())
}

pub fn federation_router(federation_state: Arc<FederationState>, store: Arc<MessageStore>) -> Router {
    Router::new()
        .route("/federation/register", post(register_server))
        .route("/federation/message", post(receive_federated_message))
        .with_state(federation_state)
        .with_state(store)
}