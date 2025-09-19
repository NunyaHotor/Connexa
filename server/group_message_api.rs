use axum::{extract::{State, Json, Path}, routing::post, Router, response::IntoResponse};
use uuid::Uuid;
use serde::Deserialize;
use crate::auth::AuthenticatedUser;
use crate::group_mls::GroupSession;
use std::sync::Arc;
use parking_lot::Mutex;
use openmls::prelude::HpkeCiphertext;

#[derive(Clone)]
pub struct GroupMlsState {
    pub sessions: Arc<Mutex<Vec<GroupSession>>>,
}

#[derive(Deserialize)]
pub struct GroupMessagePayload {
    pub group_id: Uuid,
    pub plaintext: Vec<u8>,
}

pub async fn send_group_message(
    State(state): State<GroupMlsState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<GroupMessagePayload>,
) -> impl IntoResponse {
    let backend = openmls_rust_crypto::OpenMlsRustCrypto::default();
    let mut sessions = state.sessions.lock();
    if let Some(session) = sessions.iter_mut().find(|s| s.group_id == group_id) {
        // TODO: Check user is a group member
        let ciphertext = session
            .encrypt_message(&backend, &payload.plaintext)
            .map_err(|e| format!("MLS error: {e}"));
        return Ok(Json(ciphertext.unwrap().to_vec()))
    } else {
        return Err("Group session not found".to_string())
    }
}

pub async fn receive_group_message(
    State(state): State<GroupMlsState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(group_id): Path<Uuid>,
    Json(ciphertext): Json<Vec<u8>>,
) -> impl IntoResponse {
    let backend = openmls_rust_crypto::OpenMlsRustCrypto::default();
    let mut sessions = state.sessions.lock();
    if let Some(session) = sessions.iter_mut().find(|s| s.group_id == group_id) {
        // TODO: Check user is a group member
        let mls_ciphertext = HpkeCiphertext::from_slice(&ciphertext);
        let plaintext = session
            .decrypt_message(&backend, &mls_ciphertext.unwrap())
            .map_err(|e| format!("MLS error: {e}"));
        return Ok(Json(plaintext.unwrap()))
    } else {
        return Err("Group session not found".to_string())
    }
}

pub fn group_message_router(sessions: Arc<Mutex<Vec<GroupSession>>>) -> Router {
    Router::new()
        .route("/group/:group_id/message", post(send_group_message))
        .route("/group/:group_id/receive", post(receive_group_message))
        .with_state(GroupMlsState { sessions })
}