use axum::{extract::{State, Json, Path}, routing::{post, get, delete}, Router, response::IntoResponse};
use uuid::Uuid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::Mutex;
use crate::auth::AuthenticatedUser;
use crate::device::Device;
use sqlx::SqlitePool;
use qrcode::QrCode;
use base64::{engine::general_purpose, Engine as _};
use image::{ImageOutputFormat, GrayImage, RgbImage, Luma};
use std::io::Cursor;

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

pub async fn initiate_link(
    State(state): State<DeviceState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<InitiateLinkPayload>,
) -> impl IntoResponse {
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

    let id = device.id.to_string();
    let added_at = device.added_at.to_rfc3339();
    let verified = device.verified as i32;
    // Insert the device into the database
    sqlx::query!(
        "INSERT INTO devices (id, user_id, name, added_at, verified, link_token) VALUES (?, ?, ?, ?, ?, ?)",
        id,
        device.user_id,
        device.name,
        added_at,
        verified,
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

pub async fn complete_link(
    State(state): State<DeviceState>,
    Json(payload): Json<CompleteLinkPayload>,
) -> impl IntoResponse {
    let mut devices = state.devices.lock();
    if let Some(device) = devices.iter_mut().find(|d| d.link_token.as_deref() == Some(&payload.link_token)) {
        device.verified = true;
        device.link_token = None;
        device.name = payload.device_name.clone();
        return Ok(Json(device.clone()))
    } else {
        return Err("Invalid or expired link token".to_string())
    }
}

pub async fn list_devices(
    State(state): State<DeviceState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
) -> impl IntoResponse {
    let devices = state.devices.lock();
    let user_devices: Vec<Device> = devices.iter().filter(|d| d.user_id == user_id).cloned().collect();
    Json(user_devices)
}

pub async fn unlink_device(
    State(state): State<DeviceState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(device_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut devices = state.devices.lock();
    if let Some(pos) = devices.iter().position(|d| d.id == device_id && d.user_id == user_id) {
        devices.remove(pos);
        return Ok(Json("Device unlinked successfully".to_string()))
    } else {
        return Err("Device not found or not owned by user".to_string())
    }
}

/*
// Commented out due to persistent compilation issues with qrcode crate
pub async fn get_link_qr(
    Path(link_token): Path<String>,
) -> impl IntoResponse {
    let code = QrCode::new(link_token.as_bytes()).unwrap();
    // Render to grayscale image first
    let gray_image: GrayImage = code.render::<Luma<u8>>().build();

    // Create a new RGB image with the same dimensions
    let (width, height) = gray_image.dimensions();
    let mut rgb_image = RgbImage::new(width, height);

    // Define light and dark colors
    let light_color = image::Rgb([0xFF, 0xFF, 0xFF]); // White
    let dark_color = image::Rgb([0x00, 0x00, 0x00]); // Black

    // Iterate over pixels and set colors
    for y in 0..height {
        for x in 0..width {
            let pixel = gray_image.get_pixel(x, y);
            if pixel[0] == 0 { // Dark pixel
                rgb_image.put_pixel(x, y, dark_color);
            } else { // Light pixel
                rgb_image.put_pixel(x, y, light_color);
            }
        }
    }

    let mut buf = Vec::<u8>::new();
    rgb_image
        .write_to(&mut Cursor::new(&mut buf), ImageOutputFormat::Png)
        .unwrap();
    let b64 = general_purpose::URL_SAFE_NO_PAD.encode(&buf);
    Json(b64) // Return as base64 PNG
}
*/

pub fn device_router(devices: Arc<Mutex<Vec<Device>>>, pool: SqlitePool) -> Router {
    Router::new()
        .route("/device/link/initiate", post(initiate_link))
        .route("/device/link/complete", post(complete_link))
        .route("/devices", get(list_devices))
        .route("/device/:device_id", delete(unlink_device))
        // .route("/device/link/qr/:link_token", get(get_link_qr)) // Commented out
        .with_state(DeviceState { devices, pool })
}