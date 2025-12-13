use axum::{
    body::Body,
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use redis::Commands;

use crate::{services::response, state::AppState};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = req
        .headers()
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok());
    if auth_header.is_none() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = auth_header
        .unwrap()
        .trim_start_matches("Bearer ")
        .to_string();
    let user_id = match state.jwt.validate_token(&token) {
        Ok(id) => id,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    let mut conn = state
        .redis
        .get_connection()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let key = "auth_token:".to_string() + &user_id.sub;
    let exists: bool = conn.exists(key).unwrap_or(false);
    if !exists {
        return Ok(response::error(StatusCode::UNAUTHORIZED, "Session expired"));
    }
    req.extensions_mut().insert(user_id.sub);

    Ok(next.run(req).await)
}
