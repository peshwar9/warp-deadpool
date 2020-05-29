mod app;
mod handlers;
mod routes;
mod utils;
mod models;
mod errors;
mod auth;

type Result<T> = std::result::Result<T, warp::Rejection>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    app::init_and_run().await;
}
