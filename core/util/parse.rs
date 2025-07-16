use entity::user::Role;
use serde::{Deserialize, Deserializer};


fn parse_opt<'de, T, D, F>(deserializer: D, parse_fn: F) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    F: Fn(&str) -> Result<T, String>,
{
    let opt_str: Option<String> = Option::deserialize(deserializer)?;
    match opt_str {
        Some(s) => {
            if s.trim().is_empty() {
                Ok(None)
            } else {
                parse_fn(&s).map(Some).map_err(serde::de::Error::custom)
            }
        }
        None => Ok(None),
    }
}

pub fn parse_role_opt<'de, D>(deserializer: D) -> Result<Option<Role>, D::Error>
where
    D: Deserializer<'de>,
{
    parse_opt(deserializer, |s| {
        s.parse::<Role>().map_err(|e| e.to_string())
    })
}

#[allow(dead_code)]
pub fn parse_i8_opt<'de, D>(deserializer: D) -> Result<Option<i8>, D::Error>
where
    D: Deserializer<'de>,
{
    parse_opt(deserializer, |s| s.parse::<i8>().map_err(|e| e.to_string()))
}

#[allow(dead_code)]
pub fn parse_i32_opt<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
where
    D: Deserializer<'de>,
{
    parse_opt(deserializer, |s| {
        s.parse::<i32>().map_err(|e| e.to_string())
    })
}

#[allow(dead_code)]
pub fn parse_string_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    parse_opt(deserializer, |s| Ok(s.to_string()))
}
