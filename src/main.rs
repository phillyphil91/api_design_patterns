use std::collections::{hash_map::Entry, HashMap};
use std::sync::{Arc, Mutex};

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

mod types;
use axum::http::StatusCode;
use types::{AppState, ChatRoom};

// chatroom get handler
async fn get_chatroom(
    Path(room_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<ChatRoom>, StatusCode> {
    find_chatroom(room_id, state).await
}

async fn find_chatroom(room_id: String, state: AppState) -> Result<Json<ChatRoom>, StatusCode> {
    let data = state.data.lock().unwrap();
    match data.get(&room_id) {
        Some(x) => Ok(Json(x.to_owned())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn list_chatrooms(State(state): State<AppState>) -> Json<Vec<String>> {
    let mut keys = Vec::new();
    let data = state.data.lock().unwrap();
    for key in data.keys() {
        keys.push(key.to_owned())
    }
    Json(keys)
}

async fn list_names(
    Path(room_id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<Vec<String>>, StatusCode> {
    let data = state.data.lock().unwrap();
    match data.get(&room_id) {
        Some(room) => {
            let mut names = Vec::new();
            for user in room.members.clone() {
                names.push(user.name)
            }
            Ok(Json(names))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_chatroom(
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

#[tokio::main]
async fn main() {
    let state = AppState {
        data: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/chatrooms", get(list_chatrooms).post(create_chatroom))
        .route("/chatrooms/:room_id", get(get_chatroom))
        .route("/chatrooms/:room_id/member_names", get(list_names))
        .with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
