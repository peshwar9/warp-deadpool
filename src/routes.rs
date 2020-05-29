// Standard lib: None

// External crates - Primary
use warp::{Filter, Rejection, Reply};

// External crates - Utilities: None

// Other internal modules
use crate::app::AppState;
use crate::handlers;
use crate::utils::{with_state, json_body_todocreate, json_body_todoupdate};

// Const and type declarations: None
// Struct declarations: None


// Filter function to construct routes
pub fn routes(
    state: AppState,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    index_route(state.clone())
    .or(health_route(state.clone()))
    .or(gettoken_route(state.clone()))
    .or(todos_list_route(state.clone()))
    .or(todos_create_route(state.clone()))
    .or(todos_update_route(state.clone()))
    .or(todos_delete_route(state.clone()))            
}

// http localhost:3030/hello
// Filter function for Index route
pub fn index_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("hello")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::index_handler)
}

pub fn gettoken_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("gettoken")
        .and(warp::get())
        .and(warp::header("user"))
        .and(warp::header("password"))
        .and(with_state(state))
        .and_then(handlers::gettoken_handler)
}

// http localhost:3030/health
// Filter function for Health route
pub fn health_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("health")
        .and(warp::get())
        .and(with_state(state))
        .and_then(handlers::health_handler)
}

// http localhost:3030/todo
// Filter function for listing To dos
pub fn todos_list_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todo")
    .and(warp::get())
    .and(with_state(state))
    .and_then(handlers::todos_list_handler)
   
}


// http post localhost:3030/todo name=chris
// Filter function for creating a Todo item
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
// Filter function for updating a Todo item
pub fn todos_update_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todo" / i32)
    .and(warp::put())
    .and(json_body_todoupdate())
    .and(with_state(state))
    .and_then(handlers::todos_update_handler)
   
}

// http delete localhost:3030/todo/1
// Filter function for deleting Todo item
pub fn todos_delete_route(
    state: AppState,
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("todo" / i32)
    .and(warp::delete())
    .and(with_state(state))
    .and_then(handlers::todos_delete_handler)
   
}

// ======= Unit tests section below ==========================================

// To run all tests: cargo test  -- --nocapture
// To run all tests and use println: cargo test  -- --nocapture
// To run single test: cargo test hello_test -- --nocapture
#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;
    use dotenv::dotenv;

    use super::{
        index_route,
        todos_list_route,
        todos_create_route,
        todos_update_route,
        todos_delete_route,

    };
    use crate:: {
        models,
        models::{create_pool,TodoCreate, TodoUpdate},
        app::AppState
    };



    // Test for index_route
    // cargo test hello_test -- --nocapture
    #[tokio::test]
    async fn hello_test() {
        dotenv::dotenv().ok();
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let db_pool = models::create_pool().expect("database pool cannot be created");
        let state = AppState { jwt_string, db_url, db_pool };
        let api = index_route(state);
        let response = request().method("GET").path("/hello").reply(&api).await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from  index_route is {:#?}", response.body());
    }
    // Test for todos_list_route
    // cargo test todos_list_test -- --nocapture
    #[tokio::test]
    async fn todos_list_test() {
        dotenv::dotenv().ok();
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let db_pool = models::create_pool().expect("database pool cannot be created");
        let state = AppState { jwt_string, db_url,db_pool };
        let api = todos_list_route(state);
        let response = request().method("GET").path("/todo").reply(&api).await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from todos_list_route is {:#?}", response.body());
    }
    // Test for todos_create_route
    //cargo test todos_create_test1 -- --nocapture
    #[tokio::test]
    async fn todos_create_test1() {
        dotenv::dotenv().ok();
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let db_pool = models::create_pool().expect("database pool cannot be created");
        let state = AppState { jwt_string, db_url, db_pool };
        let api = todos_create_route(state);
        let response = request()
            .method("POST")
            .path("/todo")
            .json(&TodoCreate {
                name: "chris".into()
            })
            .reply(&api).await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from todos_create_route is {:#?}", response.body());
    }

    // Test for todos_update_route
    // cargo test todos_update_test -- --nocapture
    #[tokio::test]
    async fn todos_update_test() {
        dotenv::dotenv().ok();
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let db_pool = models::create_pool().expect("database pool cannot be created");
        let state = AppState { jwt_string, db_url ,db_pool };
        let api = todos_update_route(state);
        let response = request().method("PUT")
        .path("/todo/1")
        .json(&TodoUpdate {
            name: Some("pam".into()),
            checked: Some(false),
        })
        .reply(&api).await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from todos_update_route is {:#?}", response.body());
    }
    // Test for todos_delete_route
    // cargo test todos_delete_test -- --nocapture
    #[tokio::test]
    async fn todos_delete_test() {
        dotenv::dotenv().ok();
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let db_pool = models::create_pool().expect("database pool cannot be created");
        let state = AppState { jwt_string, db_url, db_pool };
        let api = todos_delete_route(state);
        let response = request()
        .method("DELETE")
        .path("/todo/1")
        .reply(&api).await;
        assert_eq!(response.status(), StatusCode::OK);
        println!("response from todos_delete_route is {:#?}", response.body());
    }
    
    // Test for unknown path , expected return code NOT_FOUND
    //cargo test todos_unknown_path -- --nocapture
    #[tokio::test]
    async fn todos_unknown_path() {
        dotenv::dotenv().ok();
        let db_url = String::from("postgres://postgres");
        let jwt_string = String::from("afkjlksdf");
        let db_pool = models::create_pool().expect("database pool cannot be created");
        let state = AppState { jwt_string, db_url, db_pool };
        let api = todos_create_route(state);
        let response = request()
            .method("POST")
            .path("/todos")
            .json(&TodoCreate {
           
                name: "chris".into(),
                
            })
            .reply(&api).await;
        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
