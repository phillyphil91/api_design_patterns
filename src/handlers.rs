use axum::{
    extract::{Path, State},
    Json,
};
use std::collections::hash_map::Entry;

use crate::types::{AppState, ChatRoom, User};
use axum::http::StatusCode;

// chatroom get handler
pub async fn get_chatroom(
    Path(room_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ChatRoom>, StatusCode> {
    let room = find_chatroom(room_id, state).await.map(Json)?;
    Ok(room)
}

async fn find_chatroom(room_id: String, state: AppState) -> Result<ChatRoom, StatusCode> {
    let data = state.data.lock().unwrap();
    match data.get(&room_id) {
        Some(x) => Ok(x.to_owned()),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn list_chatrooms(State(state): State<AppState>) -> Json<Vec<String>> {
    let mut keys = Vec::new();
    let data = state.data.lock().unwrap();
    for key in data.keys() {
        keys.push(key.to_owned())
    }
    Json(keys)
}

pub async fn list_names(
    Path(room_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<User>>, StatusCode> {
    let room = find_chatroom(room_id, state).await?;
    let mut names = Vec::new();
    for user in room.members {
        names.push(user)
    }
    Ok(Json(names))
}

pub async fn create_chatroom(
    State(state): State<AppState>,
    Json(payload): Json<ChatRoom>,
) -> Result<Json<ChatRoom>, StatusCode> {
    let mut data = state.data.lock().unwrap();
    if let Entry::Vacant(e) = data.entry(payload.id.to_string()) {
        e.insert(payload.clone());
        Ok(Json(payload))
    } else {
        Err(StatusCode::NOT_ACCEPTABLE)
    }
}

pub async fn create_user(
    Path(room_id): Path<String>,
    State(state): State<AppState>,
    Json(payload): Json<User>,
) -> Result<Json<User>, StatusCode> {
    let mut room = find_chatroom(room_id, state).await?;
    room.members.push(payload.clone());

    Ok(Json(payload))
}
