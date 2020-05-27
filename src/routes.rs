use warp::{Filter, Reply, Rejection, reject, http::StatusCode};
use crate::app::AppState;
use crate::error::handle_rejection;
use std::convert::Infallible;

pub fn routes(state: AppState) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection > + Clone {
       index_route(state.clone())
           .or(health_route(state.clone()))
    
}

pub fn index_route(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("hello")
        .and(warp::get())
        .and(with_state(state))
        .and_then(index_handler)
}



pub fn health_route(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and(with_state(state))
        .and_then(health_handler)
}

// template
//pub fn health_route(state: AppState -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone{
//
//}
fn with_state(state: AppState) -> impl Filter<Extract= (AppState,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

// Handlers

pub async fn index_handler(state: AppState) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::json(&String::from("Hello from hendler")))
}
pub async fn health_handler(state: AppState) -> Result<impl Reply, Infallible> {
    Ok(warp::reply::json(&String::from("Health is ok")))
}
