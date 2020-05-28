// Standard lib
use std::convert::Infallible;
// External crates - Primary
use warp::Filter;
// External crates - Utilities: None

// Other internal modules
use crate::app::AppState;
use crate::models::{TodoUpdate, TodoCreate};

// Const and type declarations: None
// Struct declarations:None




// Filter Function to attach state with every request
pub fn with_state(state: AppState) -> impl Filter<Extract = (AppState,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

// Filter function to check request body size and extract contents for Todo create requests
pub fn json_body_todocreate() -> impl Filter<Extract = (TodoCreate,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}

// Filter function to check request body size and extract contents for Todo update requests
pub fn json_body_todoupdate() -> impl Filter<Extract = (TodoUpdate,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}