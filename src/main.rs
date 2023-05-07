use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};

mod handlers;
mod types;
use handlers::{create_chatroom, create_message, get_chatroom, list_chatrooms, list_messages};
use types::AppState;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState {
        data: HashMap::new(),
    }));

    let app = Router::new()
        .route("/chatrooms", get(list_chatrooms).post(create_chatroom))
        .route("/chatrooms/:room_id", get(get_chatroom))
        .route(
            "/chatrooms/:room_id/messages",
            get(list_messages).post(create_message),
        )
        .with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
