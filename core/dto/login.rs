use anyhow::anyhow;
use common::AppResult;
use salvo::macros::Extractible;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use entity::{prelude::*, user};


#[derive(Debug, Deserialize, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct LoginInRequest {
    pub username: String,
    pub password: String,
}

impl LoginInRequest {
    pub async fn is_user_exist(&self, db: &DatabaseConnection) -> AppResult<user::Model> {
        let user = User::find()
            .filter(user::Column::Username.eq(self.username.clone()))
            .one(db)
            .await?
            .ok_or(anyhow!("User not found"))?;
        Ok(user)
    }
    pub fn check_password(&self, password_hash: &str) -> AppResult<&Self> {
        util::verify_password(&self.password.as_str(), password_hash)?;
        Ok(self)
    }
    pub fn generate_token(&self, user_id: i32) -> AppResult<LoginResponse> {
        let (token, expiry) = util::get_token(self.username.clone(), user_id)?;
        Ok(LoginResponse::new(token, expiry))
    }
}

#[derive(Debug, Serialize, Default)]
pub struct LoginResponse {
    pub token: String,
    pub expiry: i64,
}

impl LoginResponse {
    fn new(token: String, expiry: i64) -> Self {
        Self { token, expiry }
    }
}
