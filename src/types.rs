use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct User {
    id: i8,
    name: String,
    email: Option<String>,
}

impl User {
    pub fn new(id: i8, name: String, email: Option<String>) -> Self {
        Self { id, name, email }
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct ChatRoom {
    id: i8,
    name: String,
    members: Vec<User>,
}

impl ChatRoom {
    pub fn new(id: i8, name: String, members: Vec<User>) -> Self {
        Self { id, name, members }
    }
}
