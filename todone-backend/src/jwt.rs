#![allow(dead_code, unused_imports)]

use async_trait::async_trait;
use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::{RequestPartsExt, TypedHeader};
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

use crate::error::{Error, Result};

use crate::config::CONFIG;

static KEYS: Lazy<Keys> = Lazy::new(|| Keys::new(CONFIG.jwt_secret.as_bytes()));

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

pub fn create_jwt(user_id: &Uuid) -> Result<String> {
    let now = Utc::now();

    let expiration = now
        .checked_add_signed(chrono::Duration::minutes(20))
        .expect("valid timestamp")
        .timestamp();

    let now = now.timestamp();

    let claims = Claims {
        sub: user_id.clone(),
        iat: now,
        exp: expiration,
    };

    let header = Header::new(Algorithm::HS512);

    let str = encode(&header, &claims, &KEYS.encoding)
        .map_err(|e| anyhow::anyhow!(e).context("failed encode JWT"))?;

    Ok(str)
}

struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| Error::Unauthorized)?;

        let token_data = decode::<Claims>(
            bearer.token(),
            &KEYS.decoding,
            &Validation::new(Algorithm::HS512),
        )
        .map_err(|_| Error::Unauthorized)?;

        if Utc::now().timestamp() > token_data.claims.exp {
            return Err(Error::Unauthorized);
        }

        Ok(token_data.claims)
    }
}
