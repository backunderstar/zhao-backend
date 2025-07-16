use common::AppResult;
use entity::user::{self, Role};
use salvo::macros::Extractible;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, PaginatorTrait,
    QueryFilter, prelude::DateTime,
};
use serde::{Deserialize, Serialize};
// use util::parse_role_opt;

use crate::EntityWithId;

use super::PaginationRequest;

#[derive(Debug, Deserialize, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct UserAddRequest {
    pub role: Role,
    pub username: String,
    pub password: String,
}
impl UserAddRequest {
    pub fn encrypt_password(&mut self) -> AppResult<&Self> {
        self.password = util::hash_password(&self.password)?;
        Ok(self)
    }
    pub async fn insert_into_db(&self, db: &DatabaseConnection) -> AppResult<()> {
        let user = user::ActiveModel {
            role: Set(self.role.clone()),
            username: Set(self.username.clone()),
            nickname: Set(self.username.clone()),
            password: Set(self.password.clone()),
            email: Set("".to_string()),
            avatar: Set("/default/avatar/keli.webp".to_string()),
            ..Default::default()
        };

        user.insert(db).await?;

        Ok(())
    }
}

#[derive(Deserialize, Debug, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct UserUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub role: Option<Role>,
    pub nickname: Option<String>,
    pub email: Option<String>,
    pub avatar: Option<String>,
}
impl UserUpdateRequest {
    pub async fn update_into_db(
        &self,
        mut user: user::ActiveModel,
        db: &DatabaseConnection,
    ) -> AppResult<()> {
        if let Some(role) = self.role.clone() {
            user.role = Set(role);
        }
        if let Some(nickname) = self.nickname.clone() {
            user.nickname = Set(nickname);
        }
        if let Some(email) = self.email.clone() {
            user.email = Set(email);
        }
        if let Some(avatar) = self.avatar.clone() {
            user.avatar = Set(avatar);
        }
        user.update(db).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Default)]
pub struct Work {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Serialize)]
pub struct UserInfoResponse {
    pub id: i32,
    pub role: Role,
    pub username: String,
    pub nickname: String,
    pub email: String,
    pub works: i32,
    pub avatar: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Deserialize, Debug, Extractible)]
#[salvo(extract(default_source(from = "query")))]
pub struct UserListRequest {
    #[serde(default)]
    // #[serde(deserialize_with = "parse_role_opt")]
    pub role: Option<Role>,
    #[salvo(extract(flatten))]
    pub pagination: PaginationRequest,
}
impl UserListRequest {
    pub async fn get_list_from_db(&self, db: &DatabaseConnection) -> AppResult<UserListResponse> {
        let mut query = self.pagination.get_query::<user::Entity>();

        if let Some(keyword) = self.pagination.keyword.as_deref() {
            query = query.filter(
                user::Column::Username
                    .contains(keyword)
                    .or(user::Column::Nickname.contains(keyword))
                    .or(user::Column::Email.contains(keyword)),
            );
        }

        if let Some(role) = self.role.clone() {
            query = query.filter(user::Column::Role.eq(role));
        }

        let count = query.clone().count(db).await?;

        let (page, limit) = self.pagination.get_page_limit();

        let users = query.paginate(db, limit).fetch_page(page).await?;

        let users_info = users.into_iter().map(|user| user.into()).collect();

        Ok(UserListResponse {
            count,
            list: users_info,
        })
    }
}

#[derive(Debug, Serialize, Default)]
pub struct UserListResponse {
    pub count: u64,
    pub list: Vec<UserInfoResponse>,
}

impl From<user::Model> for UserInfoResponse {
    fn from(user: user::Model) -> Self {
        Self {
            id: user.id,
            role: user.role,
            username: user.username,
            nickname: user.nickname,
            email: user.email,
            works: 0,
            avatar: user.avatar,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

impl EntityWithId for entity::user::Entity {
    type IdColumn = entity::user::Column;
    const ID_COLUMN: Self::IdColumn = entity::user::Column::Id;
}
