use std::time::Duration;

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use rand::Rng;
use serde::Serialize;
use sqlx::PgPool;
use validator::Validate;

use crate::{
    error::{Error, Result},
    jwt::create_jwt,
    models::user::{LoginUser, NewUser, User},
    password,
};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/register", post(create_user))
        .route("/login", post(login_user))
}

async fn create_user(
    State(pool): State<PgPool>,
    Json(new_user): Json<NewUser>,
) -> Result<StatusCode> {
    new_user.validate()?;

    let password_hash = password::hash_password(new_user.password.to_string()).await?;

    NewUser::create(&pool, new_user, password_hash).await?;

    Ok(StatusCode::CREATED)
}

async fn login_user(
    State(pool): State<PgPool>,
    Json(login_user): Json<LoginUser>,
) -> Result<impl IntoResponse> {
    let maybe_user = User::get_by_username(&pool, &login_user.username).await?;

    if let Some(user) = maybe_user {
        if password::verify_password(login_user.password, user.password_hash).await? {
            return Ok((
                StatusCode::OK,
                Json(Auth {
                    access_token: create_jwt(&user.user_id)?,
                }),
            ));
        }
    }

    // sleeping for a random duration to hide whether the username exists
    let sleep_duration =
        rand::thread_rng().gen_range(Duration::from_millis(100)..=Duration::from_millis(500));
    tokio::time::sleep(sleep_duration).await;

    Err(Error::UnprocessableEntity(
        "invalid username/password".into(),
    ))
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub access_token: String,
}
