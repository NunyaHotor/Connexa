use axum::{
    extract::{State, Json, Path},
    routing::post,
    Router,
};
use uuid::Uuid;
use serde::Deserialize;
use crate::server::auth::AuthenticatedUser;
use crate::server::group_mls::GroupSession;
use std::sync::Arc;
use parking_lot::Mutex;

#[derive(Clone)]
pub struct GroupMlsState {
    pub sessions: Arc<Mutex<Vec<GroupSession>>>,
}

#[derive(Deserialize)]
pub struct GroupMessagePayload {
    pub group_id: Uuid,
    pub plaintext: Vec<u8>,
}

#[post("/group/:group_id/message")]
pub async fn send_group_message(
    State(state): State<GroupMlsState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<GroupMessagePayload>,
) -> Result<Json<Vec<u8>>, String> {
    let backend = openmls_rust_crypto::OpenMlsRustCrypto::default();
    let mut sessions = state.sessions.lock();
    if let Some(session) = sessions.iter_mut().find(|s| s.group_id == group_id) {
        // TODO: Check user is a group member
        let ciphertext = session
            .encrypt_message(&backend, &payload.plaintext)
            .map_err(|e| format!("MLS error: {e}"))?;
        Ok(Json(ciphertext.into_bytes()))
    } else {
        Err("Group session not found".into())
    }
}

#[post("/group/:group_id/receive")]
pub async fn receive_group_message(
    State(state): State<GroupMlsState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(group_id): Path<Uuid>,
    Json(ciphertext): Json<Vec<u8>>,
) -> Result<Json<Vec<u8>>, String> {
    let backend = openmls_rust_crypto::OpenMlsRustCrypto::default();
    let mut sessions = state.sessions.lock();
    if let Some(session) = sessions.iter_mut().find(|s| s.group_id == group_id) {
        // TODO: Check user is a group member
        let mls_ciphertext = openmls::prelude::MlsCiphertext::from_bytes(&ciphertext)
            .map_err(|e| format!("MLS decode error: {e}"))?;
        let plaintext = session
            .decrypt_message(&backend, &mls_ciphertext)
            .map_err(|e| format!("MLS error: {e}"))?;
        Ok(Json(plaintext))
    } else {
        Err("Group session not found".into())
    }
}

pub fn group_message_router(sessions: Arc<Mutex<Vec<GroupSession>>>) -> Router {
    Router::new()
        .route("/group/:group_id/message", post(send_group_message))
        .route("/group/:group_id/receive", post(receive_group_message))
        .with_state(GroupMlsState { sessions })
}