use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::Value;

use std::{
    collections::hash_map::Entry,
    sync::{Arc, Mutex},
};

use crate::types::{AppState, ChatRoom, Message};

#[derive(Deserialize)]
pub struct QueryParams {
    pub field_mask: Option<String>,
}

// chatroom get handler
pub async fn get_chatroom<'a>(
    Path(room_id): Path<String>,
    Query(query_params): Query<QueryParams>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Json<Value>, StatusCode> {
    let room = match state.lock().unwrap().data.get(&room_id) {
        Some(x) => {
            let serialized =
                serde_json::to_value(x).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            match query_params.field_mask {
                Some(fields) => {
                    let requested_field_mask: Vec<&str> = fields.split(',').collect();
                    let mut filtered_data = serde_json::Map::new();
                    for field in requested_field_mask {
                        if let Some(value) = serialized.get(field) {
                            filtered_data.insert(field.to_string(), value.clone());
                        }
                    }
                    serde_json::Value::Object(filtered_data)
                }
                None => serialized,
            }
        }
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
    match state
        .lock()
        .expect("mutex lock couln't be accquired")
        .data
        .get_mut(&room_id)
    {
        Some(room) => match &mut room.messages {
            Some(messages) => messages.push(payload.clone()),
            None => room.messages = Some(vec![payload.clone()]),
        },
        None => return Err(StatusCode::NOT_FOUND),
    };
    Ok(Json(payload))
}

pub async fn list_messages(
    Path(room_id): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<Json<Option<Vec<Message>>>, StatusCode> {
    let room = match state.lock().unwrap().data.get(&room_id) {
        Some(x) => x.to_owned(),
        None => return Err(StatusCode::NOT_FOUND),
    };
    Ok(Json(room.messages))
}

pub async fn update_chatroom(
    Path(room_id): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<ChatRoom>,
) -> Result<Json<ChatRoom>, StatusCode> {
    match state.lock().unwrap().data.get_mut(&room_id) {
        Some(room) => *room = payload.clone(),
        None => return Err(StatusCode::NOT_FOUND),
    };
    Ok(Json(payload))
}

pub async fn delete_chatroom(
    Path(room_id): Path<String>,
    State(state): State<Arc<Mutex<AppState>>>,
) -> Result<(), StatusCode> {
    if state.lock().unwrap().data.remove(&room_id).is_none() {
        // attempting to delete a resource that doesn’t exist should result in a failure
        return Err(StatusCode::NOT_FOUND);
    }
    Ok(())
}
