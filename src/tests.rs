use super::*;
use axum::routing::post;
use axum_test_helper::TestClient;
use serde_json::json;

#[tokio::test]
async fn test_get_chatroom() {
    let state = Arc::new(Mutex::new(AppState {
        data: HashMap::new(),
    }));

    // Create default chatroom with default message
    state
        .lock()
        .unwrap()
        .data
        .insert("0".to_string(), types::ChatRoom::default());

    let app = Router::new()
        .route("/chatrooms", get(list_chatrooms))
        .with_state(state);

    let server = TestClient::new(app);

    let response = server.get("/chatrooms").send().await;

    assert_eq!(response.text().await, r#"["0"]"#);
}

#[tokio::test]
async fn test_create_chatroom() {
    let state = Arc::new(Mutex::new(AppState {
        data: HashMap::new(),
    }));

    let app = Router::new()
        .route("/chatrooms", post(create_chatroom))
        .with_state(state);

    let server = TestClient::new(app);

    let response = server
        .post("/chatrooms")
        .json(&json!({
        "id": 1, "name": "first created room", "messages": []
        }
        ))
        .send()
        .await;

    assert_eq!(
        response.text().await,
        r#"{"id":1,"name":"first created room","messages":[]}"#
    );
}

#[tokio::test]
async fn test_get_message() {
    let state = Arc::new(Mutex::new(AppState {
        data: HashMap::new(),
    }));

    // Create default chatroom with default message
    state
        .lock()
        .unwrap()
        .data
        .insert("1".to_string(), types::ChatRoom::default());

    let app = Router::new()
        .route("/chatrooms/:room_id/messages", get(list_messages))
        .with_state(state);

    let server = TestClient::new(app);

    let response = server.get("/chatrooms/1/messages").send().await;

    assert_eq!(
        response.text().await,
        r#"[{"id":0,"content":"default message","user":{"id":0,"name":"","email":null}}]"#
    );
}

#[tokio::test]
async fn test_create_message() {
    let state = Arc::new(Mutex::new(AppState {
        data: HashMap::new(),
    }));

    // Create default chatroom with default message
    state
        .lock()
        .unwrap()
        .data
        .insert("1".to_string(), types::ChatRoom::default());

    let app = Router::new()
        .route("/chatrooms/:room_id/messages", post(create_message))
        .with_state(state);

    let server = TestClient::new(app);

    let response = server
        .post("/chatrooms/1/messages")
        .json(&json!({
        "id": 1, "content": "newest message", "user": {"id": 1, "name": "philly", "email": "non of your business"}
        }
        ))
        .send()
        .await;

    assert_eq!(
        response.text().await,
        r#"{"id":1,"content":"newest message","user":{"id":1,"name":"philly","email":"non of your business"}}"#
    );
}
