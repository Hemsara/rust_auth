use axum::Json;
use serde_json::json;

pub async fn login() -> Json<serde_json::Value> {
    Json(json!({ "message": "login successful" }))
}
