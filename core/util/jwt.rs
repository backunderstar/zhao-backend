use common::AppResult;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Validation, decode};
use serde::{Deserialize, Serialize};
use time::{Duration, OffsetDateTime};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    username: String,
    user_id: i32,
    issuer: String,
    exp: i64,
}

pub fn get_token(username: String, user_id: i32) -> AppResult<(String, i64)> {
    let config = config::get();
    let exp = OffsetDateTime::now_utc() + Duration::minutes(config.jwt.expiry);
    let claim = JwtClaims {
        username,
        user_id,
        issuer: config.jwt.issuer.clone(),
        exp: exp.unix_timestamp(),
    };
    let token: String = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claim,
        &EncodingKey::from_secret(config.jwt.secret.as_bytes()),
    )?;
    Ok((token, exp.unix_timestamp()))
}

pub fn decode_token(token: &str) -> bool {
    let config = config::get();
    let validation = Validation::new(Algorithm::HS256);
    decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(config.jwt.secret.as_bytes()),
        &validation,
    )
    .is_ok()
}
