use axum::{extract::{State, Json, Path}, routing::{post, get, delete}, Router};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::server::auth::AuthenticatedUser;
use crate::server::device::Device;
use sqlx::SqlitePool;
use qrcode::{QrCode, Version, EcLevel};
use image::{Luma, ImageBuffer};
use base64::{encode_config, URL_SAFE_NO_PAD};
use base64::{engine::general_purpose, Engine as _};

#[derive(Clone)]
pub struct DeviceState {
    pub devices: Arc<Mutex<Vec<Device>>>,
    pub pool: SqlitePool,
}

#[derive(Deserialize)]
pub struct InitiateLinkPayload {
    pub device_name: String,
}

#[derive(Serialize)]
pub struct LinkTokenResponse {
    pub link_token: String,
}

#[post("/device/link/initiate")]
pub async fn initiate_link(
    State(state): State<DeviceState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<InitiateLinkPayload>,
) -> Json<LinkTokenResponse> {
    let link_token = Uuid::new_v4().to_string();
    let device = Device {
        id: Uuid::new_v4(),
        user_id,
        name: payload.device_name,
        added_at: Utc::now(),
        verified: false,
        link_token: Some(link_token.clone()),
    };
    state.devices.lock().push(device.clone());

    // Insert the device into the database
    sqlx::query!(
        "INSERT INTO devices (id, user_id, name, added_at, verified, link_token) VALUES (?, ?, ?, ?, ?, ?)",
        device.id.to_string(),
        device.user_id,
        device.name,
        device.added_at.to_rfc3339(),
        device.verified as i32,
        device.link_token
    )
    .execute(&state.pool)
    .await.unwrap(); // Handle error appropriately in production code

    Json(LinkTokenResponse { link_token })
}

#[derive(Deserialize)]
pub struct CompleteLinkPayload {
    pub link_token: String,
    pub device_name: String,
}

#[post("/device/link/complete")]
pub async fn complete_link(
    State(state): State<DeviceState>,
    Json(payload): Json<CompleteLinkPayload>,
) -> Result<Json<Device>, String> {
    let mut devices = state.devices.lock();
    if let Some(device) = devices.iter_mut().find(|d| d.link_token.as_deref() == Some(&payload.link_token)) {
        device.verified = true;
        device.link_token = None;
        device.name = payload.device_name.clone();
        Ok(Json(device.clone()))
    } else {
        Err("Invalid or expired link token".into())
    }
}

#[get("/devices")]
pub async fn list_devices(
    State(state): State<DeviceState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> Json<Vec<Device>> {
    let devices = state.devices.lock();
    let user_devices: Vec<Device> = devices.iter().filter(|d| d.user_id == user_id).cloned().collect();
    Json(user_devices)
}

#[delete("/device/:device_id")]
pub async fn unlink_device(
    State(state): State<DeviceState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(device_id): Path<Uuid>,
) -> Result<Json<String>, String> {
    let mut devices = state.devices.lock();
    if let Some(pos) = devices.iter().position(|d| d.id == device_id && d.user_id == user_id) {
        devices.remove(pos);
        Ok(Json("Device unlinked successfully".to_string()))
    } else {
        Err("Device not found or not owned by user".into())
    }
}

#[get("/device/link/qr/:link_token")]
pub async fn get_link_qr(
    Path(link_token): Path<String>,
) -> Result<Json<String>, String> {
    let code = QrCode::new(link_token.as_bytes()).map_err(|e| e.to_string())?;
    let image = code.render::<Luma<u8>>().build();
    let mut buf = Vec::new();
    image
        .write_to(&mut buf, image::ImageOutputFormat::Png)
        .map_err(|e| e.to_string())?;
    let b64 = general_purpose::STANDARD.encode(&buf);
    Ok(Json(b64)) // Return as base64 PNG
}

pub fn device_router(devices: Arc<Mutex<Vec<Device>>>, pool: SqlitePool) -> Router {
    Router::new()
        .route("/device/link/initiate", post(initiate_link))
        .route("/device/link/complete", post(complete_link))
        .route("/devices", get(list_devices))
        .route("/device/:device_id", delete(unlink_device))
        .route("/device/link/qr/:link_token", get(get_link_qr))
        .with_state(DeviceState { devices, pool })
}