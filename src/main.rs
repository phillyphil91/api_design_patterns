use std::collections::HashMap;

use axum::{
    extract::{Path, State},
    routing::get,
    Json, Router,
};

mod types;
use axum::http::StatusCode;
use types::{ChatRoom, User};

async fn json(
    Path(room_id): Path<String>,
    State(state): State<HashMap<String, ChatRoom>>,
) -> Result<Json<ChatRoom>, StatusCode> {
    find_chatroom(room_id, state).await
}

async fn find_chatroom(
    room_id: String,
    state: HashMap<String, ChatRoom>,
) -> Result<Json<ChatRoom>, StatusCode> {
    match state.get(&room_id) {
        Some(x) => Ok(Json(x.to_owned())),
        None => Err(StatusCode::UNAUTHORIZED),
    }
}

#[tokio::main]
async fn main() {
    let user1 = User::new(1, "philip".to_string(), Some("bla@bla.com".to_string()));
    let user2 = User::new(2, "tim".to_string(), None);

    let chatroom1 = ChatRoom::new(1, "philip's room".to_string(), vec![user1, user2]);

    let mut state = HashMap::new();
    state.insert("1".to_string(), chatroom1);
    // build our application with a single route
    let app = Router::new()
        .route("/:room_id", get(json))
        .with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
