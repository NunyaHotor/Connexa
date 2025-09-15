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
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<EncryptedMessage>,
) -> Result<(), String> {
    let authenticated = true;
    store_message_for_recipient(&state.store, &user_id, payload, authenticated)
        .await
        .map_err(|e| e.to_string())
}

pub async fn get_messages(
    State(state): State<AppState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> Result<Json<Vec<EncryptedMessage>>, String> {
    let authenticated = true;
    let msgs = state
        .store
        .fetch_messages(&user_id, authenticated)
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