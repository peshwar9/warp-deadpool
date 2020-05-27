use warp::{Filter, Rejection};
mod app;
mod routes;
mod error;

type Result<T> = std::result::Result<T, Rejection>;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    app::init_and_run().await;    

}
