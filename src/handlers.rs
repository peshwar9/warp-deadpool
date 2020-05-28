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

// cargo test -- --nocapture , if you want to use println
// To run single test cargo test hello_test -- --nocapture
#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::{
    index_handler,
    health_handler,
    
    };
    use crate::app::AppState;
    use crate::routes;

    #[tokio::test]
    async fn hello_test() {
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
    let state = AppState {
        jwt_string,
        db_url,
    };
    let api = routes::index_route(state);
        let response = request()
            .method("GET")
            .path("/hello")
            .reply(&api)
            .await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from test hello_route is {:#?}",response.body());
    }




}
