use axum::response::IntoResponse;
use axum::{extract::State, http::StatusCode, Json};
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};

use serde::Deserialize;

use crate::services::password::{hash_password, verify_password};
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

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub username: String,
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

    let user = user.unwrap();

    let pw = verify_password(&payload.password, &user.password_hash).unwrap_or(false);

    if !pw {
        return response::error(StatusCode::UNAUTHORIZED, "Invalid credentials");
    }

    let token = state.jwt.generate_token(&user.id.to_string()).unwrap();

    response::ok(token)
}

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    // Registration logic here

    let user = match UserEntity::find()
        .filter(UserColumn::Email.eq(&payload.email))
        .one(&state.db)
        .await
    {
        Ok(u) => u,
        Err(_) => {
            return response::error(StatusCode::INTERNAL_SERVER_ERROR, "Internal error");
        }
    };

    if user.is_some() {
        return response::error(StatusCode::CONFLICT, "User already exists");
    }

    let password_hash = hash_password(&payload.password);
    if password_hash.is_err() {
        return response::error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to hash password");
    }
    let new_user = crate::entities::users::ActiveModel {
        email: sea_orm::ActiveValue::Set(payload.email),
        password_hash: sea_orm::ActiveValue::Set(password_hash.unwrap()),
        ..Default::default()
    };

    let res = UserEntity::insert(new_user).exec(&state.db).await;
    if res.is_err() {
        return response::error(StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user");
    }


    response::ok("User registered successfully")
}
