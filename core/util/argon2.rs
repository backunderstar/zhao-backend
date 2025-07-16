use argon2::{
    Argon2, PasswordHash,
    password_hash::{SaltString, rand_core::OsRng},
};
use common::AppResult;


pub fn verify_password(password: &str, password_hash: &str) -> AppResult<()> {
    let hash = PasswordHash::new(&password_hash)
        .map_err(|e| anyhow::anyhow!("invalid password hash: {}", e))?;
    hash.verify_password(&[&Argon2::default()], password)
        .map_err(|e| anyhow::anyhow!("invalid password: {}", e))?;
    Ok(())
}

pub fn hash_password(password: &str) -> AppResult<String> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(PasswordHash::generate(Argon2::default(), password, &salt)
        .map_err(|e| anyhow::anyhow!("failed to generate password hash: {}", e))?
        .to_string())
}
