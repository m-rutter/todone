#![allow(dead_code, unused_imports)]

use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;

use crate::config::CONFIG;

static KEYS: Lazy<Keys> = Lazy::new(|| Keys::new(CONFIG.jwt_secret.as_bytes()));

pub struct Keys {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl Keys {
    fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
