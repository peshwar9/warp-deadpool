use warp::Filter;
use crate::app::AppState;

pub fn routes(state: AppState) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection > + Clone {
    let index_route = warp::path("hello").map(|| "Hello");
    let health_route = warp::path("health").map(|| "Health Ok");
    let routes = index_route
        .or(health_route);
    routes
}

