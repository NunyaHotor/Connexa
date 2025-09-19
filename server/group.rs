use uuid::Uuid;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Group {
    pub id: Uuid,
    pub name: String,
    pub created_at: DateTime<Utc>,
    pub members: Vec<String>, // user IDs
}

#[derive(Clone, Debug)]
pub struct GroupInvite {
    pub group_id: Uuid,
    pub invited_user_id: String,
    pub invited_by: String,
    pub created_at: DateTime<Utc>,
}