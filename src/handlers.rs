use warp::{Filter, Reply, Rejection, reject, http::StatusCode};
use crate::app::AppState;
use std::convert::Infallible;

pub async fn index_handler(state: AppState) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::json(&String::from("Hello from handler")))
}
pub async fn health_handler(state: AppState) -> Result<impl Reply, Infallible> {
    //Ok(warp::reply::json(&String::from("Health is ok")))
    Ok(StatusCode::OK)
}

