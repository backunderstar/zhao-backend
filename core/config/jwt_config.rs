use super::default_string;
use serde::Deserialize;

#[derive(Deserialize, Clone, Debug)]
pub struct JwtConfig {
    #[serde(default = "default_string")]
    pub issuer: String,
    #[serde(default = "default_string")]
    pub secret: String,
    #[serde(default = "default_expiry")]
    pub expiry: i64,
}

fn default_expiry() -> i64 {
    4320
}
