use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;

pub fn ok<T: Serialize>(data: T) -> Response {
    (
        StatusCode::OK,
        Json(json!({
            "success": true,
            "data": data
        })),
    )
        .into_response()
}

pub fn created<T: Serialize>(data: T) -> Response {
    (
        StatusCode::CREATED,
        Json(json!({
            "success": true,
            "data": data
        })),
    )
        .into_response()
}

pub fn error(status: StatusCode, message: &str) -> Response {
    (
        status,
        Json(json!({
            "success": false,
            "error": message
        })),
    )
        .into_response()
}
