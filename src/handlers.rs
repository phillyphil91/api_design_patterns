use axum::{
    extract::{Path, State},
    Json,
};
use std::{
    collections::hash_map::Entry,
    sync::{Arc, Mutex},
};

use crate::types::{AppState, ChatRoom, Message};
use axum::http::StatusCode;

// chatroom get handler
pub async fn get_chatroom<'a>(
    Path(room_id): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Json<ChatRoom>, StatusCode> {
    let room = match state.lock().unwrap().data.get(&room_id) {
        Some(x) => x.clone(),
        None => return Err(StatusCode::NOT_FOUND),
    };
    Ok(Json(room))
}

pub async fn list_chatrooms(State(state): State<Arc<Mutex<AppState>>>) -> Json<Vec<String>> {
    let mut keys = Vec::new();
    let data = &state.lock().unwrap().data;
    for key in data.keys() {
        keys.push(key.to_owned())
    }
    Json(keys)
}

pub async fn create_chatroom(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<ChatRoom>,
) -> Result<Json<ChatRoom>, StatusCode> {
    let data = &mut state.lock().unwrap().data;
    if let Entry::Vacant(e) = data.entry(payload.id.to_string()) {
        e.insert(payload.clone());
        Ok(Json(payload))
    } else {
        Err(StatusCode::NOT_ACCEPTABLE)
    }
}

pub async fn create_message(
    Path(room_id): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<Message>,
) -> Result<Json<Message>, StatusCode> {
    match state.lock().unwrap().data.get_mut(&room_id) {
        Some(room) => room.messages.push(payload.clone()),
        None => return Err(StatusCode::NOT_FOUND),
    };
    Ok(Json(payload))
}

pub async fn list_messages(
    Path(room_id): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Json<Vec<Message>>, StatusCode> {
    let room = match state.lock().unwrap().data.get(&room_id) {
        Some(x) => x.to_owned(),
        None => return Err(StatusCode::NOT_FOUND),
    };
    let mut messages = Vec::new();
    for message in room.messages {
        messages.push(message)
    }
    Ok(Json(messages))
}
