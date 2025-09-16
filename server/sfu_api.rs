use axum::{extract::{Json, State}, routing::post, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct JoinSfuRoomRequest {
    pub room_id: String,
}

#[derive(Serialize)]
pub struct JoinSfuRoomResponse {
    pub sfu_url: String,
    pub sfu_room_id: String,
    pub sfu_token: Option<String>,
}

#[post("/sfu/join")]
pub async fn join_sfu_room(
    Json(req): Json<JoinSfuRoomRequest>,
) -> Json<JoinSfuRoomResponse> {
    // TODO: Call your SFU’s API to allocate/join the room if needed
    // For Jitsi, you may just return the room name and server URL
    Json(JoinSfuRoomResponse {
        sfu_url: "wss://your.sfu.server".to_string(),
        sfu_room_id: req.room_id,
        sfu_token: None, // If your SFU uses tokens
    })
}

pub fn sfu_router() -> Router {
    Router::new().route("/sfu/join", post(join_sfu_room))
}