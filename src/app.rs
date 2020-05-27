use warp::Filter;
use crate::routes;
use std::env;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone,Debug)]
pub struct AppState {
    jwt_string: String,
    db_url: String,
}

pub async fn init_and_run() {
    
    let db_url = env::var("DATABASE_URL").expect("Database URL must be set");
    let jwt_string = env::var("JWT_SECRET").expect("JWT Secret must be set");

    let app_state = AppState {
    jwt_string,
    db_url
    };
    let routes = routes::routes(app_state).with(warp::log("APP_NAME"));
    warp::serve(routes).run(([127,0,0,1],3030)).await;
}
