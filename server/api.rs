use axum::{
    extract::{Path, State, Json},
    routing::{get},
    Router,
    response::IntoResponse,
};
use crate::relay::{MessageStore};
use crate::connexa::EncryptedMessage;
use std::sync::Arc;
use crate::auth::AuthenticatedUser;

use serde::Deserialize;

#[derive(Clone)]
pub struct AppState {
    pub store: Arc<MessageStore>,
}

#[derive(Deserialize)]
pub struct SendMessagePayload {
    pub recipient_user_id: String,
    pub plaintext: Vec<u8>,
}

/*
pub async fn post_message(
    State(state): State<AppState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<SendMessagePayload>,
) -> Result<(), String> {
    // Lookup all recipient devices
    //let recipient_devices = state.device_store.get_devices_for_user(&payload.recipient_user_id).await;
    //send_message_to_all_devices(
    //    &state.session, // sender's session
    //    &recipient_devices,
    //    &payload.plaintext,
    //    &[], // AAD
    //).await;
    Ok(())
}
*/

pub async fn get_messages(
    State(state): State<AppState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> Result<Json<Vec<EncryptedMessage>>, String> {
    let msgs = state
        .store
        .fetch_messages(&user_id.to_string(), true)
        .await
        .map_err(|e| e.to_string())?;
    Ok(Json(msgs))
}

pub fn app(store: Arc<MessageStore>) -> Router {
    Router::new()
        //.route("/message", post(post_message))
        .route("/messages/:recipient_id", get(get_messages))
        .with_state(AppState { store })
}