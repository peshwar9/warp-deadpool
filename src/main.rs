pub mod app;
pub mod routes;
pub mod handlers;


#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    app::init_and_run().await;    

}
