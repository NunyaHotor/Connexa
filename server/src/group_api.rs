use axum::{extract::{State, Json, Path}, routing::{post, get}, Router, response::IntoResponse};
use std::sync::Arc;
use uuid::Uuid;
use chrono::Utc;
use crate::auth::AuthenticatedUser;
use crate::group::{Group, GroupType};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct GroupState {
    pub groups: Arc<Mutex<Vec<Group>>>,
}

#[derive(serde::Deserialize)]
pub struct CreateGroupPayload {
    pub name: String,
}

pub async fn create_group(
    State(state): State<GroupState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateGroupPayload>,
) -> impl IntoResponse {
    let mut groups = state.groups.lock();
    let group = Group {
        id: Uuid::new_v4(),
        name: payload.name,
        members: vec![user_id],
    };
    groups.push(group.clone());
    Json(group)
}

pub async fn join_group(
    State(state): State<GroupState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(group_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut groups = state.groups.lock();
    if let Some(group) = groups.iter_mut().find(|g| g.id == group_id) {
        if !group.members.contains(&user_id) {
            group.members.push(user_id);
        }
        return Ok(Json(group.clone()));
    }
    Err("Group not found".to_string())
}

pub async fn leave_group(
    State(state): State<GroupState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(group_id): Path<Uuid>,
) -> impl IntoResponse {
    let mut groups = state.groups.lock();
    if let Some(group) = groups.iter_mut().find(|g| g.id == group_id) {
        group.members.retain(|m| m != &user_id);
        return Ok(Json(group.clone()));
    }
    Err("Group not found".to_string())
}

pub async fn get_group(
    State(state): State<GroupState>,
    Path(group_id): Path<Uuid>,
) -> impl IntoResponse {
    let groups = state.groups.lock();
    if let Some(group) = groups.iter().find(|g| g.id == group_id) {
        return Ok(Json(group.clone()));
    }
    Err("Group not found".to_string())
}

pub fn group_router(groups: Arc<Mutex<Vec<Group>>>) -> Router {
    Router::new()
        .route("/group", post(create_group))
        .route("/group/:group_id/join", post(join_group))
        .route("/group/:group_id/leave", post(leave_group))
        .route("/group/:group_id", get(get_group))
        .with_state(GroupState { groups })
}








