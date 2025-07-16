use anyhow::anyhow;
use common::AppResult;
use salvo::macros::Extractible;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PrimaryKeyTrait, QueryFilter, QueryOrder, Select,
};
use serde::Deserialize;

mod login;
pub use login::*;
mod user;
pub use user::*;
mod article;
pub use article::*;
mod image;
pub use image::*;
mod system;
pub use system::*;

/// id 请求
/// 和通用的方法
/// 1. 根据id获取数据
/// 2. 根据id删除数据
#[derive(Debug, Deserialize, Extractible)]
pub struct IDRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
}
impl IDRequest {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
    pub async fn get_data_by_id<T>(&self, db: &DatabaseConnection) -> AppResult<T::Model>
    where
        T: EntityTrait,
        <T::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        let id: <T::PrimaryKey as PrimaryKeyTrait>::ValueType = self.id.into();
        let model = T::find_by_id(id)
            .one(db)
            .await?
            .ok_or_else(|| anyhow!("data not found for type: {}", std::any::type_name::<T>()))?;

        Ok(model)
    }
    pub async fn delete_data_by_id<T>(&self, db: &DatabaseConnection) -> AppResult<()>
    where
        T: EntityTrait,
        <T::PrimaryKey as PrimaryKeyTrait>::ValueType: From<i32>,
    {
        let id: <T::PrimaryKey as PrimaryKeyTrait>::ValueType = self.id.into();
        T::delete_by_id(id).exec(db).await?;
        Ok(())
    }
}

/// id列表 请求
/// 和通用的方法
/// 根据id列表删除数据
#[derive(Debug, Deserialize, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct IDListRequest {
    pub id_list: Vec<u32>,
}
impl IDListRequest {
    pub async fn delete_datas_by_ids<T>(&self, db: &DatabaseConnection) -> AppResult<()>
    where
        T: EntityTrait + EntityWithId,
    {
        T::delete_many()
            .filter(T::ID_COLUMN.is_in(self.id_list.clone()))
            .exec(db)
            .await?;

        Ok(())
    }
    pub async fn get_datas_by_ids<T>(&self, db: &DatabaseConnection) -> AppResult<Vec<T::Model>>
    where
        T: EntityTrait + EntityWithId,
    {
        let data = T::find()
            .filter(T::ID_COLUMN.is_in(self.id_list.clone()))
            .all(db)
            .await?;

        Ok(data)
    }
}
pub trait EntityWithId {
    type IdColumn: ColumnTrait;
    const ID_COLUMN: Self::IdColumn;
}

#[derive(Deserialize, Debug, Extractible)]
#[salvo(extract(default_source(from = "query")))]
pub struct PaginationRequest {
    pub page: Option<u64>,
    pub limit: Option<u64>,
    pub order: Option<Order>,
    pub keyword: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub enum Order {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}
impl PaginationRequest {
    pub fn get_query<T>(&self) -> Select<T>
    where
        T: EntityTrait + EntityWithId,
    {
        let mut query = T::find();

        match self.order.clone().unwrap_or(Order::Asc) {
            Order::Asc => query = query.order_by_asc(T::ID_COLUMN),
            Order::Desc => query = query.order_by_desc(T::ID_COLUMN),
        }

        query
    }
    pub fn get_page_limit(&self) -> (u64, u64) {
        let limit = match self.limit {
            Some(limit) => limit,
            None => 12,
        };

        let page = match self.page {
            Some(page) => page - 1,
            None => 0,
        };

        (page, limit)
    }
}
