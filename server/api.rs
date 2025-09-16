use axum::{
    extract::{Path, State, Json},
    routing::{post, get},
    Router,
};
use crate::server::relay::{MessageStore, store_message_for_recipient, RelayError};
use crate::proto::message::EncryptedMessage;
use std::sync::Arc;
use crate::server::auth::AuthenticatedUser;
use axum::extract::RequestParts;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<MessageStore>,
}

pub async fn post_message(
    State(state): State<AppState>,
    AuthenticatedUser { user_id, device_id }: AuthenticatedUser,
    Json(payload): Json<SendMessagePayload>,
) -> Result<(), String> {
    // Lookup all recipient devices
    let recipient_devices = state.device_store.get_devices_for_user(&payload.recipient_user_id).await;
    send_message_to_all_devices(
        &state.session, // sender's session
        &recipient_devices,
        &payload.plaintext,
        &[], // AAD
        &state.message_store,
    ).await;
    Ok(())
}

#[get("/messages")]
pub async fn get_messages(
    State(state): State<AppState>,
    AuthenticatedUser { device_id, .. }: AuthenticatedUser,
) -> Result<Json<Vec<EncryptedMessage>>, String> {
    let msgs = state
        .message_store
        .fetch_messages(&device_id.to_string(), true)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(msgs))
}

pub fn app(store: Arc<MessageStore>) -> Router {
    Router::new()
        .route("/message", post(post_message))
        .route("/messages/:recipient_id", get(get_messages))
        .with_state(AppState { store })
}