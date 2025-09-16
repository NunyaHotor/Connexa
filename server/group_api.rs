use axum::{
    extract::{State, Json, Path},
    routing::{post, get},
    Router,
};
use std::sync::Arc;
use uuid::Uuid;
use crate::server::group::{Group, GroupInvite};
use crate::server::auth::AuthenticatedUser;
use chrono::Utc;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct GroupState {
    pub groups: Arc<Mutex<Vec<Group>>>,
}

#[derive(Deserialize)]
pub struct CreateGroupPayload {
    pub name: String,
}

#[derive(Deserialize)]
pub struct InvitePayload {
    pub user_id: String,
}

#[post("/group")]
pub async fn create_group(
    State(state): State<GroupState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Json(payload): Json<CreateGroupPayload>,
) -> Json<Group> {
    let group = Group {
        id: Uuid::new_v4(),
        name: payload.name,
        created_at: Utc::now(),
        members: vec![user_id],
    };
    state.groups.lock().push(group.clone());
    Json(group)
}

#[get("/group/:group_id")]
pub async fn get_group(
    State(state): State<GroupState>,
    Path(group_id): Path<Uuid>,
) -> Option<Json<Group>> {
    let groups = state.groups.lock();
    groups.iter().find(|g| g.id == group_id).cloned().map(Json)
}

#[post("/group/:group_id/invite")]
pub async fn invite_to_group(
    State(state): State<GroupState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<InvitePayload>,
) -> Result<Json<Group>, String> {
    let mut groups = state.groups.lock();
    if let Some(group) = groups.iter_mut().find(|g| g.id == group_id) {
        if !group.members.contains(&user_id) {
            return Err("Only group members can invite.".into());
        }
        if !group.members.contains(&payload.user_id) {
            group.members.push(payload.user_id.clone());
        }
        Ok(Json(group.clone()))
    } else {
        Err("Group not found".into())
    }
}

#[post("/group/:group_id/leave")]
pub async fn leave_group(
    State(state): State<GroupState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(group_id): Path<Uuid>,
) -> Result<Json<Group>, String> {
    let mut groups = state.groups.lock();
    if let Some(group) = groups.iter_mut().find(|g| g.id == group_id) {
        if let Some(pos) = group.members.iter().position(|m| m == &user_id) {
            group.members.remove(pos);
            Ok(Json(group.clone()))
        } else {
            Err("You are not a member of this group.".into())
        }
    } else {
        Err("Group not found".into())
    }
}

#[post("/group/:group_id/remove")]
pub async fn remove_from_group(
    State(state): State<GroupState>,
    AuthenticatedUser { user_id }: AuthenticatedUser,
    Path(group_id): Path<Uuid>,
    Json(payload): Json<InvitePayload>,
) -> Result<Json<Group>, String> {
    let mut groups = state.groups.lock();
    if let Some(group) = groups.iter_mut().find(|g| g.id == group_id) {
        // Only allow removal by group members (could restrict to admins)
        if !group.members.contains(&user_id) {
            return Err("Only group members can remove.".into());
        }
        if let Some(pos) = group.members.iter().position(|m| m == &payload.user_id) {
            group.members.remove(pos);
            Ok(Json(group.clone()))
        } else {
            Err("User is not a member of this group.".into())
        }
    } else {
        Err("Group not found".into())
    }
}

pub fn group_router(groups: Arc<Mutex<Vec<Group>>>) -> Router {
    Router::new()
        .route("/group", post(create_group))
        .route("/group/:group_id", get(get_group))
        .route("/group/:group_id/invite", post(invite_to_group))
        .route("/group/:group_id/leave", post(leave_group))
        .route("/group/:group_id/remove", post(remove_from_group))
        .with_state(GroupState { groups })
}

openmls = "0.11"
openmls_rust_crypto = "0.11"