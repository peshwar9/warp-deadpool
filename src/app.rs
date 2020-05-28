use crate::routes;
use console::Style;
use std::env;
use warp::Filter;

const APP_NAME: &str = env!("CARGO_PKG_NAME");

#[derive(Clone, Debug)]
pub struct AppState {
    pub jwt_string: String,
    pub db_url: String,
}

pub async fn init_and_run() {
    // Environment variables
    let db_url = env::var("DATABASE_URL").expect("Database URL must be set");
    let jwt_string = env::var("JWT_SECRET").expect("JWT Secret must be set");

    //Local variable declarations
    let blue = Style::new().blue();
    let url: String = "127.0.0.1:3030".parse().unwrap();

    // Shared application state across async tasks
    let app_state = AppState { jwt_string, db_url };

    //Routes
    let routes = routes::routes(app_state)
        .with(warp::log(APP_NAME))
        .with(warp::cors().allow_any_origin());

    // Console messages
    println!("\nWarp todo server ready at {}", blue.apply_to(&url));
    println!("Use $curl localhost:3030/hello to test end points");
    //Serve
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
