use warp::Filter;
use crate::app::AppState;
use std::convert::Infallible;
use crate::models::{TodoUpdate, TodoCreate};

pub fn with_state(state: AppState) -> impl Filter<Extract = (AppState,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}


pub fn json_body_todocreate() -> impl Filter<Extract = (TodoCreate,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

pub fn json_body_todoupdate() -> impl Filter<Extract = (TodoUpdate,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}