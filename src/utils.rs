use warp::Filter;
use crate::app::AppState;
use std::convert::Infallible;
use crate::models::Todo;

pub fn with_state(state: AppState) -> impl Filter<Extract = (AppState,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

pub fn json_body() -> impl Filter<Extract = (Todo,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}