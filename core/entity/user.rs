use std::str::FromStr;

use sea_orm::{
    Set, entity::prelude::*, prelude::async_trait::async_trait, sqlx::types::chrono::Utc,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "user")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub role: Role,
    #[sea_orm(unique)]
    pub username: String,
    pub password: String,
    pub nickname: String,
    pub email: String,
    pub avatar: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, EnumIter, Serialize, Deserialize, DeriveActiveEnum)]
#[sea_orm(
    rs_type = "String",
    db_type = "String(StringLen::N(10))",
    enum_name = "role"
)]
pub enum Role {
    #[sea_orm(string_value = "admin")]
    #[serde(rename = "admin")]
    Admin,
    #[sea_orm(string_value = "user")]
    #[serde(rename = "user")]
    User,
    #[sea_orm(string_value = "guest")]
    #[serde(rename = "guest")]
    Guest,
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "admin" => Ok(Role::Admin),
            "user" => Ok(Role::User),
            "guest" => Ok(Role::Guest),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl Related<super::article::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_article::Relation::Article.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_article::Relation::User.def().rev())
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = Utc::now().naive_utc();
        if insert {
            self.created_at = Set(now);
        }
        self.updated_at = Set(now);
        Ok(self)
    }
}
