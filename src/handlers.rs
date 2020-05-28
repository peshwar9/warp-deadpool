use crate::app::AppState;
use std::convert::Infallible;
use warp::{http::StatusCode, reject, Reply, Rejection};
use crate::models::{TodoCreate, TodoUpdate};
use crate::errors::AppError;

pub async fn index_handler(_state: AppState) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::json(&String::from("Hello from handler")))
}
pub async fn todos_list_handler(_state: AppState) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::json(&String::from("Hello from list_todos")))
}
pub async fn todos_create_handler(todo: TodoCreate, _state: AppState) -> Result<impl Reply, Rejection> {
        let x = 1;
        if x == 0 {
            let response = format!("Hello from create_todo {:#?}", todo);
            Ok(warp::reply::json(&response))
        } else {
            Err(reject::custom(AppError)
        }

    }
   

pub async fn todos_update_handler(id: u64, todo: TodoUpdate, _state: AppState) -> Result<impl Reply, Infallible> {
    let response = format!("Hello from update_todo for {} is {:#?}", id, todo);
    Ok(warp::reply::json(&response))
}
pub async fn todos_delete_handler(id: u64, _state: AppState) -> Result<impl Reply, Infallible> {
    let response = format!("Deleted id {} ", id);
    Ok(warp::reply::json(&response))
}

pub async fn health_handler(_state: AppState) -> Result<impl Reply, Infallible> {
    //Ok(warp::reply::json(&String::from("Health is ok")))
    Ok(StatusCode::OK)
}
