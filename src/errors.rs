// Standard lib
use std::convert::Infallible;

// External crates - Primary
use warp::{http::StatusCode, Rejection, Reply};

// External crates - Utilities
use serde::Serialize;

// Other internal modules: None
// Const and type declarations: None
// Struct declarations

#[derive(Serialize)]
struct ErrorResponse {
message: String,
}

#[derive(Debug)]
pub struct AppError;    

impl warp::reject::Reject for AppError {}

// Function to provide meaningful error messages to caller of APIs.
pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {
let code;
let message;

if err.is_not_found() {
    code = StatusCode::NOT_FOUND;
    message = "Path is not found";
} else if let Some(_) = err.find::<warp::filters::body::BodyDeserializeError>() {
    code = StatusCode::BAD_REQUEST;
    message = "Please pass right parameters";  
} 
  else if let Some(_) = err.find::<warp::reject::MethodNotAllowed>() {
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
