use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use axum::{routing::get, Router};

mod handlers;
mod types;

#[cfg(test)]
mod tests;

use handlers::{
    create_chatroom, create_message, delete_chatroom, get_chatroom, list_chatrooms, list_messages,
    update_chatroom,
};
use types::AppState;

#[tokio::main]
async fn main() {
    let state = Arc::new(Mutex::new(AppState {
        data: HashMap::new(),
    }));

    let app = Router::new()
        .route("/chatrooms", get(list_chatrooms).post(create_chatroom))
        .route(
            "/chatrooms/:room_id",
            get(get_chatroom)
                .patch(update_chatroom)
                .delete(delete_chatroom),
        )
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
