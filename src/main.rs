mod app;
mod handlers;
mod routes;
mod utils;
mod models;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    app::init_and_run().await;
}
