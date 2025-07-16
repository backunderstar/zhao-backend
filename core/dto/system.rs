use common::{AppError, AppResult};
use entity::system;
use salvo::prelude::*;
use sea_orm::{ActiveValue::Set, prelude::*};
use serde::{Deserialize, Serialize};



#[derive(Debug, Deserialize, Extractible)]
#[salvo(extract(default_source(from = "body")))]
pub struct SystemUpdateRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub keywords: Option<String>,
    pub domain: Option<String>,
    pub copyright: Option<String>,
    pub logo: Option<String>,
}
impl SystemUpdateRequest {
    pub fn check_none(&self) -> AppResult<()> {
        match self.name.is_none()
            && self.description.is_none()
            && self.keywords.is_none()
            && self.domain.is_none()
            && self.copyright.is_none()
            && self.logo.is_none()
        {
            false => Ok(()),
            true => Err(AppError::Public("Invalid data".to_string())),
        }
    }
    pub async fn update_into_db(
        &self,
        mut system: system::ActiveModel,
        db: &DatabaseConnection,
    ) -> AppResult<()> {
        if let Some(name) = &self.name {
            system.name = Set(name.clone());
        }
        if let Some(description) = &self.description {
            system.description = Set(description.clone());
        }
        if let Some(keywords) = &self.keywords {
            system.keywords = Set(keywords.clone());
        }
        if let Some(domain) = &self.domain {
            system.domain = Set(domain.clone());
        }
        if let Some(copyright) = &self.copyright {
            system.copyright = Set(copyright.clone());
        }
        if let Some(logo) = &self.logo {
            system.logo = Set(logo.clone());
        }

        system.update(db).await?;
        Ok(())
    }
}

#[derive(Debug, Serialize, Default, Clone)]
pub struct SystemInfoResponse {
    pub name: String,
    pub description: String,
    pub keywords: String,
    pub domain: String,
    pub copyright: String,
    pub created_at: DateTime,
    pub logo: String,
}

impl From<system::Model> for SystemInfoResponse {
    fn from(model: system::Model) -> Self {
        Self {
            name: model.name,
            description: model.description,
            keywords: model.keywords,
            domain: model.domain,
            copyright: model.copyright,
            created_at: model.created_at,
            logo: model.logo,
        }
    }
}
