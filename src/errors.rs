// Standard lib
use std::convert::Infallible;

// External crates - Primary
use warp::{http::StatusCode, Rejection, Reply};
use tokio_postgres::error::Error as TokioError;
use deadpool_postgres::PoolError as PoolError;

// External crates - Utilities
use serde::Serialize;
use derive_more::{Display};
// Other internal modules: None
// Const and type declarations: None
// Struct declarations


#[derive(Display, Debug)]
pub enum MyError {
    NotFound(TokioError),
    DBPoolError(PoolError),
    DBQueryError(TokioError),
//    DBInitError(TokioError),
    InternalServerError(String),
}
//impl std::error::Error for MyError {}


#[derive(Serialize)]
struct ErrorResponse {
message: String,
}

impl warp::reject::Reject for MyError {}

// Function to provide meaningful error messages to caller of APIs.
pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
let code;
let message;
println!("In handle_rejection, err is {:#?}", err);
if err.is_not_found() {
    // 404: NotFound
    code = StatusCode::NOT_FOUND;
    message = "Path is not found";
} else if let Some(e) = err.find::<warp::filters::body::BodyDeserializeError>() {
    // 400 Bad Request : BodyDeserializeError
    code = StatusCode::BAD_REQUEST;
    message = "Please pass right parameters";  
    println!("System error message is {:#?}",e);
}  else if let Some(e) = err.find::<MyError>() {
    match e {
        MyError::DBQueryError(_) => {
            code = StatusCode::BAD_REQUEST;
            message = "Error in query, please send correct data";
        }
        MyError::NotFound(_) => {
            code = StatusCode::BAD_REQUEST;
            message = "The requested data is not found";
        }
        _ => {
            eprintln!("unhandled application error: {:?}", err);
            code = StatusCode::INTERNAL_SERVER_ERROR;
            message = "Internal Server Error";
        }
    }
    
} else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
    // 405: MethodNotAllowed  
    code = StatusCode::METHOD_NOT_ALLOWED;
    message = "Method not allowed";
} else {
    eprintln!("unhandled error: {:?}", err);
    code = StatusCode::INTERNAL_SERVER_ERROR;
    message = "Internal server error";
}

let json = warp::reply::json(&ErrorResponse {
    message: message.into(),
});

Ok(warp::reply::with_status(json,code))

}
