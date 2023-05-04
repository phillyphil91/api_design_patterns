use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    id: i8,
    pub name: String,
    email: Option<String>,
}

#[allow(dead_code)]
impl User {
    pub fn new(id: i8, name: String, email: Option<String>) -> Self {
        Self { id, name, email }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChatRoom {
    pub id: i8,
    name: String,
    pub members: Vec<User>,
}

#[allow(dead_code)]
impl ChatRoom {
    pub fn new(id: i8, name: String, members: Vec<User>) -> Self {
        Self { id, name, members }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub data: Arc<Mutex<HashMap<String, ChatRoom>>>,
}
