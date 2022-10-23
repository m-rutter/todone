use anyhow::Context;
use argon2::{
    password_hash::{self, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use tokio::task;

use crate::error::Result;

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
