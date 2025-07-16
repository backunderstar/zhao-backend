use sea_orm::{
    entity::prelude::*, prelude::async_trait::async_trait, sqlx::types::chrono::Utc, Set,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq, Serialize, Deserialize)]
#[sea_orm(table_name = "image")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: u32,
    pub name: String,
    pub path: String,
    pub image_type: String,
    pub size: u32,
    #[sea_orm(unique)]
    pub hash: String,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            let now = Utc::now().naive_utc();
            self.created_at = Set(now);
        }
        Ok(self)
    }
}
