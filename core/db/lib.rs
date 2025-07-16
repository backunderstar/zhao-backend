use std::{fs, path::Path, sync::OnceLock, time::Duration};

use config::DbConfig;
use sea_orm::{ConnectOptions, Database, DatabaseConnection};


mod data;
use data::insert_data;
mod table;
use table::create_tables;

pub static SEAORM_POOL: OnceLock<DatabaseConnection> = OnceLock::new();

/// 初始化数据库
pub async fn init() {
    let config = config::get();

    let url = get_url(&config.db);

    let mut opt = ConnectOptions::new(url);
    opt.max_connections(config.db.max_connections)
        .min_connections(config.db.min_connections)
        .connect_timeout(Duration::from_secs(config.db.connect_timeout))
        .idle_timeout(Duration::from_secs(config.db.idle_timeout))
        .sqlx_logging(config.db.sqlx_logging);

    let pool = Database::connect(opt)
        .await
        .expect("db connection should connect");

    // create database tables
    create_tables(&pool)
        .await
        .expect("create tables should success");

    // insert default data
    insert_data(&pool)
        .await
        .expect("insert data should success");

    SEAORM_POOL.set(pool).expect("seaorm pool should be set");
}

pub fn pool() -> &'static DatabaseConnection {
    SEAORM_POOL.get().expect("seaorm pool should set")
}

/// 确保sqlite数据库目录存在
fn ensure_data_directory(path: &String) {
    let path = Path::new(path).parent();

    if let Some(path) = path {
        fs::create_dir_all(path).expect("create data directory should success");
    };
}

fn get_url(config: &DbConfig) -> String {
    match config.db_type.as_str() {
        "sqlite" => {
            ensure_data_directory(&config.sqlite_path);
            format!("sqlite:{}?mode=rwc", config.sqlite_path)
        }
        "mysql" => format!(
            "mysql://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.database
        ),
        "postgresql" => format!(
            "postgresql://{}:{}@{}:{}/{}",
            config.username, config.password, config.host, config.port, config.database
        ),
        _ => panic!("db type should be sqlite, mysql or postgresql"),
    }
}
