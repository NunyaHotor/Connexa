use axum::{routing::post, Router, Json};
use webrtc::api::APIBuilder;
use webrtc::peer_connection::RTCPeerConnection;
use webrtc::ice_transport::ice_candidate::{RTCIceCandidate, RTCIceCandidateInit};
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::configuration::RTCConfiguration;
use webrtc::error::{Result, Error};
use std::sync::Arc;
use tokio::sync::Mutex;
use serde::{Deserialize, Serialize};

// In a real application, you'd manage peer connections per user/session
// For simplicity, we'll use a global one for now.
// This needs to be replaced with a proper session management system.
lazy_static::lazy_static! {
    static ref PEER_CONNECTION: Arc<Mutex<Option<RTCPeerConnection>>> = Arc::new(Mutex::new(None));
}

pub async fn create_peer_connection_instance() -> Result<RTCPeerConnection> {
    let api = APIBuilder::new().build();
    let config = RTCConfiguration::default();
    let peer_connection = api.new_peer_connection(config).await?;
    Ok(peer_connection)
}

pub async fn handle_offer_route(Json(offer): Json<RTCSessionDescription>) -> Result<Json<RTCSessionDescription>> {
    let mut pc_guard = PEER_CONNECTION.lock().await;
    if pc_guard.is_none() {
        let new_pc = create_peer_connection_instance().await.map_err(|e| Error::new(e.to_string()))?;
        *pc_guard = Some(new_pc);
    }
    let pc = pc_guard.as_mut().unwrap();

    pc.set_remote_description(offer).await.map_err(|e| Error::new(e.to_string()))?;
    let answer = pc.create_answer(None).await.map_err(|e| Error::new(e.to_string()))?;
    pc.set_local_description(answer.clone()).await.map_err(|e| Error::new(e.to_string()))?;

    Ok(Json(answer))
}

pub async fn handle_answer_route(Json(answer): Json<RTCSessionDescription>) -> Result<()> {
    let mut pc_guard = PEER_CONNECTION.lock().await;
    if let Some(pc) = pc_guard.as_mut() {
        pc.set_remote_description(answer).await.map_err(|e| Error::new(e.to_string()))?;
        Ok(())
    } else {
        Err(Error::new("Peer connection not initialized.".to_string()))
    }
}

pub async fn handle_ice_candidate_route(Json(candidate): Json<RTCIceCandidate>) -> Result<()> {
    let mut pc_guard = PEER_CONNECTION.lock().await;
    if let Some(pc) = pc_guard.as_mut() {
        let ice_candidate_init = candidate.to_json();
        pc.add_ice_candidate(ice_candidate_init).await.map_err(|e| Error::new(e.to_string()))?;
        Ok(())
    } else {
        Err(Error::new("Peer connection not initialized.".to_string()))
    }
}

pub fn webrtc_router() -> Router {
    Router::new()
        .route("/webrtc/offer", post(handle_offer_route))
        .route("/webrtc/answer", post(handle_answer_route))
        .route("/webrtc/candidate", post(handle_ice_candidate_route))
}
