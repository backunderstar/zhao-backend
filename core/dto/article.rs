use common::AppResult;
use entity::article;
use salvo::macros::Extractible;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, prelude::DateTime};
use serde::{Deserialize, Serialize};

use crate::EntityWithId;

#[derive(Debug, Deserialize, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct ArticleAddRequest {
    pub title: String,
    pub summary: Option<String>,
    pub cover: String,
    pub content: String,
    pub published: bool,
}
impl ArticleAddRequest {
    pub async fn insert_into_db(&self, db: &DatabaseConnection) -> AppResult<()> {
        let article = article::ActiveModel {
            title: Set(self.title.clone()),
            summary: Set(self.summary.clone().unwrap_or_default()),
            cover: Set(self.cover.clone()),
            content: Set(self.content.clone()),
            published: Set(self.published),
            ..Default::default()
        };

        article.insert(db).await?;

        Ok(())
    }
}

#[derive(Deserialize, Debug, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct ArticleUpdateRequest {
    #[salvo(extract(source(from = "param")))]
    pub id: i32,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub published: Option<bool>,
}
impl ArticleUpdateRequest {
    pub async fn update_into_db(
        &self,
        mut article: article::ActiveModel,
        db: &DatabaseConnection,
    ) -> AppResult<()> {
        if let Some(title) = self.title.clone() {
            article.title = Set(title);
        }
        if let Some(summary) = self.summary.clone() {
            article.summary = Set(summary);
        }
        if let Some(content) = self.content.clone() {
            article.content = Set(content);
        }
        if let Some(published) = self.published {
            article.published = Set(published);
        }
        article.update(db).await?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Default)]
pub struct Author {
    pub id: i32,
    pub nickname: String,
    pub avatar: String,
}

#[derive(Debug, Serialize, Default)]
pub struct ArticleInfoResponse {
    pub id: i32,
    pub title: String,
    pub cover: String,
    pub author: Vec<Author>,
    pub summary: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Default)]
pub struct ArticleInfoWithoutContentResponse {
    pub id: i32,
    pub title: String,
    pub cover: String,
    pub author: Vec<Author>,
    pub summary: String,
    pub published: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Default)]
pub struct ArticleListResponse {
    pub count: u64,
    pub list: Vec<ArticleInfoWithoutContentResponse>,
}
impl ArticleListResponse {
    pub fn new(count: u64, list: Vec<ArticleInfoWithoutContentResponse>) -> Self {
        Self { count, list }
    }
}

impl From<entity::article::Model> for ArticleInfoResponse {
    fn from(article: entity::article::Model) -> Self {
        Self {
            id: article.id,
            title: article.title,
            cover: article.cover,
            author: vec![],
            summary: article.summary,
            content: article.content,
            published: article.published,
            created_at: article.created_at,
            updated_at: article.updated_at,
        }
    }
}

impl From<entity::article::Model> for ArticleInfoWithoutContentResponse {
    fn from(article: entity::article::Model) -> Self {
        Self {
            id: article.id,
            title: article.title,
            cover: article.cover,
            author: vec![],
            summary: article.summary,
            published: article.published,
            created_at: article.created_at,
            updated_at: article.updated_at,
        }
    }
}

impl EntityWithId for entity::article::Entity {
    type IdColumn = entity::article::Column;
    const ID_COLUMN: Self::IdColumn = entity::article::Column::Id;
}
