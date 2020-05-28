// Standard lib: None
// External crates - Primary: None

// External crates - Utilities
use serde::{Serialize, Deserialize};

// Other internal modules: None
// Const and type declarations: None

// Struct declarations

// To do struct that represents the database table
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Todo {
    pub id: u64,
    pub name: String,
    pub completed: bool,
    pub priority: Option<String>,
}

// Struct for sending parameters to create Todo item
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoCreate {
    pub name: String,
    pub priority: Option<String>,
}

//Struct for sending parameters to update Todo item
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct TodoUpdate {
    pub name: Option<String>,
    pub completed: Option<bool>,
}

// Functions: None