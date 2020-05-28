use warp::Filter;
use crate::app::AppState;
use std::convert::Infallible;

pub fn with_state(state: AppState) -> impl Filter<Extract = (AppState,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}