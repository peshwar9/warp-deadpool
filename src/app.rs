use warp::Filter;

pub async fn init_and_run() {
      let hello = warp::path!("hello").map(|| "Hello");

    let routes = hello
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127,0,0,1],3030)).await;
}
