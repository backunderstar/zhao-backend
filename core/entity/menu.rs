use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize, DeriveEntityModel)]
#[sea_orm(table_name = "menu")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub parent_id: i32,
    pub name: String,
    pub path: String,
    pub component: String,
    pub title: String,
    pub icon: String,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}
impl Related<super::user_menu::Entity> for Entity {
    fn to() -> RelationDef {
        super::user_menu::Relation::Menu.def()
    }
    fn via() -> Option<RelationDef> {
        Some(super::user_menu::Relation::User.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
