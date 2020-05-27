use warp::Filter;
mod app;
mod routes;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    app::init_and_run().await;    

}
