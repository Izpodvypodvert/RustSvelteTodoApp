use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub description: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct NewTodo {
    pub description: String
}