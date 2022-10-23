use std::time::Duration;

use anyhow::Context;
use argon2::{
    password_hash::{self, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use axum::{http::StatusCode, response::IntoResponse, routing::post, Extension, Json, Router};
use once_cell::sync::Lazy;
use rand::Rng;
use regex::Regex;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tokio::task;
use validator::Validate;

use crate::{
    error::{Error, Result},
    jwt::create_jwt,
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

    let password_hash = hash_password(new_user.password.to_string()).await?;

    sqlx::query_scalar!(
        r#"
        insert into "user" (username, password_hash) 
        values ($1, $2)
        "#,
        new_user.username,
        password_hash
    )
    .fetch_one(&db.0)
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
        if verify_password(login_user.password, user.password_hash).await? {
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

pub async fn hash_password(password: String) -> anyhow::Result<String> {
    task::spawn_blocking(move || {
        let salt = SaltString::generate(rand::thread_rng());
        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!(e).context("BUG: failed to hash password"))?
            .to_string())
    })
    .await
    .context("panic in hash")?
}

pub async fn verify_password(password: String, password_hash: String) -> Result<bool> {
    task::spawn_blocking(move || -> Result<bool> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| anyhow::anyhow!(e).context("BUG: invalid password hash"))?;

        let res = Argon2::default().verify_password(password.as_bytes(), &hash);

        match res {
            Ok(_) => Ok(true),
            Err(password_hash::Error::Password) => Ok(false),
            Err(e) => Err(anyhow::anyhow!(e).context("BUG: failed to verify password"))?,
        }
    })
    .await
    .context("panic in veryifying password hash")?
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
