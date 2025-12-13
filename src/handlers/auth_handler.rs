use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};

use serde::Deserialize;

use crate::services::response;
use crate::{
    entities::{UserColumn, UserEntity},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

pub async fn login(
    State(state): State<AppState>,

    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    let user = match UserEntity::find()
        .filter(UserColumn::Email.eq(payload.email))
        .one(&state.db)
        .await
    {
        Ok(u) => u,
        Err(_) => {
            return response::error(StatusCode::INTERNAL_SERVER_ERROR, "Internal error");
        }
    };

    if user.is_none() {
        return response::error(StatusCode::UNAUTHORIZED, "Invalid credentials");
    }

    response::ok("Login successful")
}
