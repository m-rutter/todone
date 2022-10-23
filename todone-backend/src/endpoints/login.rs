use std::time::Duration;

use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use once_cell::sync::Lazy;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    error::{Error, Result},
    jwt::create_jwt,
    password,
};

static USERNAME_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").expect("username regex"));

pub fn router() -> Router {
    Router::new()
        .route("/register", post(create_user))
        .route("/login", post(login_user))
}

async fn create_user(db: Extension<PgPool>, Json(new_user): Json<NewUser>) -> Result<StatusCode> {
    new_user.validate()?;

    let password_hash = password::hash_password(new_user.password.to_string()).await?;

    sqlx::query_scalar!(
        r#"
        insert into "user" (username, password_hash) 
        values ($1, $2)
        "#,
        new_user.username,
        password_hash
    )
    .execute(&db.0)
    .await
    .map_err(|e| match e {
        sqlx::Error::Database(db_error) if db_error.constraint() == Some("user_username_key") => {
            Error::Conflict("username taken".into())
        }
        _ => e.into(),
    })?;

    Ok(StatusCode::CREATED)
}

async fn login_user(
    db: Extension<PgPool>,
    Json(login_user): Json<LoginUser>,
) -> Result<impl IntoResponse> {
    let maybe_user = sqlx::query!(
        r#"
        select user_id, username, password_hash 
        from "user" 
        where username = $1"#,
        login_user.username
    )
    .fetch_optional(&db.0)
    .await?;

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

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
struct NewUser {
    #[validate(length(min = 3, max = 16), regex = "USERNAME_RE")]
    username: String,
    #[validate(length(min = 8, max = 32))]
    password: String,
}

#[derive(Deserialize)]
struct LoginUser {
    username: String,
    password: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Auth {
    access_token: String,
}
