use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatRoom {
    pub id: i8,
    pub name: String,
    pub messages: Vec<Message>,
}

#[allow(dead_code)]
impl ChatRoom {
    pub fn new(id: i8, name: String, messages: Vec<Message>) -> Self {
        Self { id, name, messages }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    pub id: i8,
    pub content: String,
    pub user: User,
}

#[allow(dead_code)]
impl Message {
    pub fn new(id: i8, content: String, user: User) -> Self {
        Self { id, content, user }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i8,
    pub name: String,
    pub email: Option<String>,
}

#[allow(dead_code)]
impl User {
    pub fn new(id: i8, name: String, email: Option<String>) -> Self {
        Self { id, name, email }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub data: HashMap<String, ChatRoom>,
}
