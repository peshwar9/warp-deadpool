use crate::app::AppState;
use crate::handlers;
use warp::{Filter, Rejection, Reply};
use crate::utils::{with_state, json_body_todocreate, json_body_todoupdate};


pub fn routes(
    state: AppState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    index_route(state.clone())
    .or(health_route(state.clone()))
    .or(todos_list_route(state.clone()))
    .or(todos_create_route(state.clone()))
    .or(todos_update_route(state.clone()))
    .or(todos_delete_route(state.clone()))            
}

// http localhost:3030/hello
pub fn index_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("hello")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::index_handler)
}

// http localhost:3030/health
pub fn health_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::health_handler)
}

// http localhost:3030/todo
pub fn todos_list_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todo")
    .and(warp::get())
    .and(with_state(state))
    .and_then(handlers::todos_list_handler)
   
}


// http post localhost:3030/todo name=chris
pub fn todos_create_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todo")
    .and(warp::post())
    .and(json_body_todocreate())
    .and(with_state(state))
    .and_then(handlers::todos_create_handler)
   
}

// http put localhost:3030/todo/1 name=Pam completed:=true
pub fn todos_update_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todo" / u64)
    .and(warp::put())
    .and(json_body_todoupdate())
    .and(with_state(state))
    .and_then(handlers::todos_update_handler)
   
}

// http delete localhost:3030/todo/1
pub fn todos_delete_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todo" / u64)
    .and(warp::delete())
    .and(with_state(state))
    .and_then(handlers::todos_delete_handler)
   
}



// To run all tests: cargo test  -- --nocapture
// To run all tests and use println: cargo test  -- --nocapture
// To run single test: cargo test hello_test -- --nocapture
#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use super::{
        index_route,
        todos_list_route,
        todos_create_route,
        todos_update_route,
        todos_delete_route,
    };
    use crate::app::AppState;
    use crate::models::{Todo, TodoCreate, TodoUpdate};

    // cargo test hello_test -- --nocapture
    #[tokio::test]
    async fn hello_test() {
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let state = AppState { jwt_string, db_url };
        let api = index_route(state);
        let response = request().method("GET").path("/hello").reply(&api).await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from  index_route is {:#?}", response.body());
    }
    // cargo test todos_list_test -- --nocapture
    #[tokio::test]
    async fn todos_list_test() {
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let state = AppState { jwt_string, db_url };
        let api = todos_list_route(state);
        let response = request().method("GET").path("/todo").reply(&api).await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from todos_list_route is {:#?}", response.body());
    }
    // Create todo success
    //cargo test todos_create_test1 -- --nocapture
    #[tokio::test]
    async fn todos_create_test1() {
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let state = AppState { jwt_string, db_url };
        let api = todos_create_route(state);
        let response = request()
            .method("POST")
            .path("/todo")
            .json(&TodoCreate {
                name: "chris".into(),
            })
            .reply(&api).await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from todos_create_route is {:#?}", response.body());
    }
    // Unknown path
    //cargo test todos_unknown_path -- --nocapture
    #[tokio::test]
    async fn todos_unknown_path() {
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let state = AppState { jwt_string, db_url };
        let api = todos_create_route(state);
        let response = request()
            .method("POST")
            .path("/todos")
            .json(&Todo {
                id: 1,
                name: "chris".into(),
                completed: false,
            })
            .reply(&api).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
    // cargo test todos_update_test -- --nocapture
    #[tokio::test]
    async fn todos_update_test() {
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let state = AppState { jwt_string, db_url };
        let api = todos_update_route(state);
        let response = request().method("PUT")
        .path("/todo/1")
        .json(&TodoUpdate {
            name: "pam".into(),
            completed: false,
        })
        .reply(&api).await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from todos_update_route is {:#?}", response.body());
    }
    // cargo test todos_delete_test -- --nocapture
    #[tokio::test]
    async fn todos_delete_test() {
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let state = AppState { jwt_string, db_url };
        let api = todos_delete_route(state);
        let response = request()
        .method("DELETE")
        .path("/todo/1")
        .reply(&api).await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from todos_delete_route is {:#?}", response.body());
    }
}
