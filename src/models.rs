use serde::{Serialize, Deserialize};


#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub name: String,
    pub completed: bool,
    pub priority: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoCreate {
    pub name: String,
    pub priority: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoUpdate {
    pub name: Option<String>,
    pub completed: Option<bool>,
}

