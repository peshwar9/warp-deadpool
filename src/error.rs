use warp::{http::StatusCode, Rejection, Reply};
use serde::Serialize;
use std::convert::Infallible;

#[derive(Serialize)]
struct ErrorResponse {
message: String,
}

#[derive(Debug)]
pub struct AppError;

impl warp::reject::Reject for AppError {}

pub async fn handle_rejection(err: Rejection) -> std::result::Result<impl Reply, Infallible> {

let code = StatusCode::INTERNAL_SERVER_ERROR;

let json = warp::reply::json(&ErrorResponse {
    message: "Error in server processing".into(),
});

Ok(warp::reply::with_status(json,code))

}
