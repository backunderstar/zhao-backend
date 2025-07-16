use serde::{Deserialize, Serialize};

use super::default_false;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct DbConfig {
    /// Settings for the primary database. This is usually writeable, but will be read-only in
    /// some configurations.
    /// An optional follower database. Always read-only.
    #[serde(alias = "type", default = "default_db_type")]
    pub db_type: String,

    #[serde(default = "default_sqlite_path")]
    pub sqlite_path: String,
    #[serde(default = "default_host")]
    pub host: String,
    #[serde(default = "default_port")]
    pub port: u16,
    #[serde(default = "default_database")]
    pub database: String,
    #[serde(default = "default_username")]
    pub username: String,
    #[serde(default = "default_password")]
    pub password: String,

    #[serde(default = "default_min_connections")]
    pub min_connections: u32,
    #[serde(default = "default_max_connections")]
    pub max_connections: u32,

    #[serde(default = "default_idle_timeout")]
    pub idle_timeout: u64,
    #[serde(default = "default_connect_timeout")]
    pub connect_timeout: u64,

    #[serde(default = "default_false")]
    pub sqlx_logging: bool,
}

fn default_db_type() -> String {
    "sqlite".to_string()
}
fn default_sqlite_path() -> String {
    "./data/db.sqlite".to_string()
}
fn default_host() -> String {
    "127.0.0.1".to_string()
}
fn default_port() -> u16 {
    3306
}
fn default_database() -> String {
    "zhao".to_string()
}
fn default_username() -> String {
    "root".to_string()
}
fn default_password() -> String {
    "123456".to_string()
}
fn default_min_connections() -> u32 {
    5
}
fn default_max_connections() -> u32 {
    100
}
fn default_idle_timeout() -> u64 {
    8
}
fn default_connect_timeout() -> u64 {
    8
}
