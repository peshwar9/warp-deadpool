// Standard lib
use std::convert::Infallible;

// External crates - Primary
use warp::{http::StatusCode, reject, Reply, Rejection};

// External crates - Utilities: None
// Other internal modules
use crate::app::AppState;
use crate::Result;
use crate::models::{TodoCreate, 
    TodoUpdate, 
    Todo, 
    fetch_to_dos, 
    create_todo,
    update_todo,
    delete_todo,    
};
use crate::errors;
use crate::errors::{MyError};

// Const and type declarations:None
// Struct declarations:None

// Index handler
pub async fn index_handler(_state: AppState) -> Result<impl Reply> {
    Ok(warp::reply::json(&String::from("Hello from handler")))
}
// Health handler
pub async fn health_handler(_state: AppState) -> Result<impl Reply> {
    //Ok(warp::reply::json(&String::from("Health is ok")))
    Ok(StatusCode::OK)
}

// Todo: List handler
pub async fn todos_list_handler(state: AppState) -> Result<impl Reply> {
    let r: Vec<Todo> = fetch_to_dos(&state.db_pool)
    .await
    .map_err(|e| reject::custom(MyError::InternalServerError(e.to_string())))?;
    Ok(warp::reply::json(&r))
}

// Todo: Create handler
pub async fn todos_create_handler(todo: TodoCreate, state: AppState) -> Result<impl Reply> {
    let r = create_todo(&state.db_pool,todo)
    .await
    .map_err(|e| reject::custom(e))?;
  
Ok(warp::reply::json(&r))
    }
   
// Todo: Update handler
pub async fn todos_update_handler(id: i32, todo: TodoUpdate, state: AppState) -> Result<impl Reply> {
    let r = update_todo(&state.db_pool,id, todo)
    .await
    .map_err(|e| reject::custom(e))?;
    Ok(warp::reply::json(&r))
}

// Todo: Delete handler
pub async fn todos_delete_handler(id: i32, state: AppState) -> Result<impl Reply> {
    delete_todo(&state.db_pool,id)
    .await
    .map_err(|e| reject::custom(e))?;
    Ok(StatusCode::OK)
}


