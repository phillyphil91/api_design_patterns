use super::*;
use axum_test::TestServer;
use serde_json::json;

#[tokio::test]
async fn test_get_chatroom() {
    let state = Arc::new(Mutex::new(AppState {
        data: HashMap::new(),
    }));

    // Create dummy chatroom
    state
        .lock()
        .unwrap()
        .data
        .insert("0".to_string(), types::ChatRoom::default());

    // Create the app with the repository as state.
    let app = Router::new()
        .route("/chatrooms", get(list_chatrooms))
        .with_state(state)
        .into_make_service();

    let server = TestServer::new(app).unwrap();

    let response = server
        .get("/chatrooms")
        .json(&json!({
            "username": "Terrance Pencilworth",
        }))
        .await;

    assert_eq!(response.text(), r#"["0"]"#);
}
