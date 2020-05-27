use warp::{Filter, Reply, Rejection, reject, http::StatusCode};
use crate::app::AppState;
use crate::handlers;
use std::convert::Infallible;

pub fn routes(state: AppState) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection > + Clone {
       index_route(state.clone())
           .or(health_route(state.clone()))
    
}

pub fn index_route(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("hello")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::index_handler)
}



pub fn health_route(state: AppState) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::health_handler)
}

fn with_state(state: AppState) -> impl Filter<Extract= (AppState,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

