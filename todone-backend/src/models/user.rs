use once_cell::sync::Lazy;
use regex::Regex;
use serde::Deserialize;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;
use validator::Validate;

use crate::error::{DbErrorResultExt, Error, Result};

static USERNAME_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[0-9A-Za-z_]+$").expect("username regex"));

#[derive(Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct NewUser {
    #[validate(length(min = 3, max = 16), regex = "USERNAME_RE")]
    pub username: String,
    #[validate(length(min = 8, max = 32))]
    pub password: String,
}

impl NewUser {
    pub async fn create(db: &PgPool, new_user: NewUser, password_hash: String) -> Result<()> {
        sqlx::query_scalar!(
            r#"
        insert into "user" (username, password_hash) 
        values ($1, $2)
        "#,
            new_user.username,
            password_hash
        )
        .execute(db)
        .await
        .on_constraint("user_username_key", |_| {
            Error::Conflict("username taken".into())
        })?;

        Ok(())
    }
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}

#[derive(FromRow)]
pub struct User {
    pub user_id: Uuid,
    pub username: String,
    pub password_hash: String,
}

impl User {
    pub async fn get_by_username(db: &PgPool, username: &str) -> Result<Option<User>> {
        let maybe_user = sqlx::query_as!(
            User,
            r#"
        select user_id, username, password_hash 
        from "user" 
        where username = $1"#,
            username
        )
        .fetch_optional(db)
        .await?;

        Ok(maybe_user)
    }
}
