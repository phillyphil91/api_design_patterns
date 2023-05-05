use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};

mod handlers;
mod types;
use handlers::{create_chatroom, create_user, get_chatroom, list_chatrooms, list_names};
use types::AppState;

#[tokio::main]
async fn main() {
    let state = AppState {
        data: Arc::new(Mutex::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/chatrooms", get(list_chatrooms).post(create_chatroom))
        .route("/chatrooms/:room_id", get(get_chatroom))
        .route(
            "/chatrooms/:room_id/member",
            get(list_names).post(create_user),
        )
        .with_state(state);

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
