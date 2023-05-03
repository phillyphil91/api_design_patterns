use axum::{extract::Path, routing::get, Json, Router};

mod types;
use types::{ChatRoom, User};

async fn json(Path(room_id): Path<i8>) -> Json<ChatRoom> {
    find_chatroom(room_id)
    Json(chatroom1)
}

async fn find_chatroom(room_id: i8) -> ChatRoom {
    let user1 = User::new(1, "philip".to_string(), Some("bla@bla.com".to_string()));
    let user2 = User::new(2, "johannes".to_string(), None);
    let chatroom1 = ChatRoom::new(room_id, "philip's room".to_string(), vec![user1, user2]);
    chatroom1
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(json));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
