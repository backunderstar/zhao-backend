use std::sync::OnceLock;

use figment::Figment;
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;

mod db_config;
mod jwt_config;
mod log_config;
pub use db_config::DbConfig;
pub use jwt_config::JwtConfig;
pub use log_config::LogConfig;

pub static CONFIG: OnceLock<ServerConfig> = OnceLock::new();
/// 初始化配置
pub fn init() {
    let raw_config = Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("APP_").global());

    let config = match raw_config.extract::<ServerConfig>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("It looks like your config is invalid. The following error occurred: {e}");
            std::process::exit(1);
        }
    };
    
    CONFIG.set(config).expect("config should be set");
}
pub fn get() -> &'static ServerConfig {
    CONFIG.get().expect("config should be set")
}

#[derive(Deserialize, Clone, Debug)]
pub struct ServerConfig {
    #[serde(default = "default_listen_addr")]
    pub listen_addr: String,

    pub db: DbConfig,
    pub log: LogConfig,
    pub jwt: JwtConfig,
}

pub fn default_false() -> bool {
    false
}
pub fn default_true() -> bool {
    true
}
pub fn default_string() -> String {
    "zhao".into()
}
fn default_listen_addr() -> String {
    "127.0.0.1:5432".into()
}
