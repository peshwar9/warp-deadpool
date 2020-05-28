// Standard lib
use std::convert::Infallible;

// External crates - Primary
use warp::{http::StatusCode, reject, Reply, Rejection};

// External crates - Utilities: None
// Other internal modules
use crate::app::AppState;
use crate::models::{TodoCreate, TodoUpdate};
use crate::errors::AppError;
// Const and type declarations:None
// Struct declarations:None

// Index handler
pub async fn index_handler(_state: AppState) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::json(&String::from("Hello from handler")))
}
// Health handler
pub async fn health_handler(_state: AppState) -> Result<impl Reply, Infallible> {
    //Ok(warp::reply::json(&String::from("Health is ok")))
    Ok(StatusCode::OK)
}

// Todo: List handler
pub async fn todos_list_handler(_state: AppState) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::json(&String::from("Hello from list_todos")))
}

// Todo: Create handler
pub async fn todos_create_handler(todo: TodoCreate, _state: AppState) -> Result<impl Reply, Rejection> {
        let name = &todo.name;
       if let Ok(_) = name.parse::<String>() {
            let response = format!("Hello from create_todo {:#?}", todo);
            Ok(warp::reply::json(&response))
        } else {
            Err(reject::custom(AppError))
        }

    }
   
// Todo: Update handler
pub async fn todos_update_handler(id: u64, todo: TodoUpdate, _state: AppState) -> Result<impl Reply, Infallible> {
    let response = format!("Hello from update_todo for {} is {:#?}", id, todo);
    Ok(warp::reply::json(&response))
}

// Todo: Delete handler
pub async fn todos_delete_handler(id: u64, _state: AppState) -> Result<impl Reply, Infallible> {
    let response = format!("Deleted id {} ", id);
    Ok(warp::reply::json(&response))
}


