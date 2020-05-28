mod app;
mod handlers;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    app::init_and_run().await;
}
