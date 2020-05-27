use warp::Filter;
use crate::app::AppState;
use std::convert::Infallible;

pub fn routes(state: AppState) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection > + Clone {
    let index_route = warp::path("hello")
        .and(with_state(state.clone()))
        .map(|state: AppState| format!("Hello db is {:?}",state));

    let health_route = warp::path("health")
        .and(with_state(state.clone()))
        .map(|state: AppState| format!("Health is ok; db is {:?}",state));

    let routes = index_route
        .or(health_route);
    routes
}

fn with_state(state: AppState) -> impl Filter<Extract= (AppState,), Error = Infallible> + Clone {
    warp::any().map(move || state.clone())
}

 

