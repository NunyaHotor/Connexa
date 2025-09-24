use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize, FromRow)]
pub struct Device {
    pub id: Uuid,
    pub user_id: String,
    pub name: String,
    pub added_at: DateTime<Utc>,
    pub verified: bool,
    pub link_token: Option<String>, // For linking flow
}