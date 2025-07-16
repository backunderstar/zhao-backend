use std::{fs, io, path::Path};

use common::AppResult;
use entity::{
    image::{self, ActiveModel, Model},
    prelude::Image,
};
use salvo::{http::form::FilePart, macros::Extractible};
use sea_orm::{
    ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, prelude::*,
};
use serde::{Deserialize, Serialize};
use util::{get_file_extension, hash_file, is_image, rename_with_hash};

use crate::EntityWithId;

use super::PaginationRequest;

#[derive(Debug, Clone)]
pub struct ImageUploadDto {
    pub name: String,
    pub path: String,
    pub hash: String,
    pub size: u32,
    pub image_type: String,
    pub is_success: bool,
    pub message: String,
}
impl ImageUploadDto {
    // 创建图片信息dto
    pub fn new(files: &Vec<FilePart>) -> Vec<Self> {
        files
            .iter()
            .map(|file| {
                let mut dto = Self {
                    name: String::new(),
                    path: file.path().to_string_lossy().into_owned(),
                    hash: String::new(),
                    size: file.size() as u32,
                    image_type: String::new(),
                    is_success: true,
                    message: String::new(),
                };
                dto.valid_image_name_type(file);
                dto.valid_file_size();
                dto
            })
            .collect()
    }
    // 校验图片名称和格式
    fn valid_image_name_type(&mut self, file: &FilePart) {
        match file.name() {
            Some(n) => self.name = n.to_string(),
            None => {
                self.is_success = false;
                self.message = "image name not found".to_string();
                return;
            }
        };

        match get_file_extension(self.path.clone()) {
            Some(ext) => {
                if is_image(self.path.clone()) {
                    self.image_type = ext;
                } else {
                    self.is_success = false;
                    self.message = "unsupported image formats".to_string();
                    return;
                }
            }
            None => {
                self.is_success = false;
                self.message = "unsupported image formats".to_string();
            }
        };
    }
    // 校验图片大小
    fn valid_file_size(&mut self) {
        if self.size > 1024 * 1024 * 5 {
            self.is_success = false;
            self.message = "image size exceeds limit".to_string();
        }
    }
    // 图片哈希
    pub async fn image_hash(&mut self, db: &DatabaseConnection) -> AppResult<()> {
        match fs::read(self.path.clone()) {
            Ok(file) => match hash_file(&file) {
                Ok(hash) => match self.image_find_by_hash(&db, hash.clone()).await? {
                    Some(model) => {
                        self.name = model.path.clone();
                        self.is_success = false;
                        self.message = format!("The image already exists, name: {}", model.name);
                    }
                    None => {
                        self.hash = hash;
                    }
                },
                Err(e) => {
                    self.is_success = false;
                    self.message = e.to_string();
                }
            },
            Err(e) => {
                self.is_success = false;
                self.message = e.to_string();
            }
        }
        Ok(())
    }
    // 图片重命名
    pub fn image_rename(&mut self) {
        match rename_with_hash(self.name.clone(), self.hash.clone()) {
            Ok(new_name) => {
                self.name = new_name;
            }
            Err(e) => {
                self.is_success = false;
                self.message = e.to_string();
            }
        }
    }
    // 图片保存
    pub fn image_save(&mut self) {
        let dest = format!("upload/image/{}", self.name);
        match fs::copy(&self.path, Path::new(&dest)) {
            Ok(_) => {
                self.path = dest;
            }
            Err(e) => {
                self.is_success = false;
                self.message = e.to_string();
            }
        }
    }
    // 通过哈希查找图片
    async fn image_find_by_hash(
        &self,
        db: &DatabaseConnection,
        hash: String,
    ) -> AppResult<Option<Model>> {
        let image = Image::find()
            .filter(image::Column::Hash.eq(hash))
            .one(db)
            .await?;
        Ok(image)
    }
    // 插入数据库
    pub async fn insert_into_db(&mut self, db: &DatabaseConnection) {
        // 创建图片的数据模型
        let image = ActiveModel {
            name: Set(self.name.clone()),
            path: Set(self.path.clone()),
            hash: Set(self.hash.clone()),
            size: Set(self.size),
            image_type: Set(self.image_type.clone()),
            ..Default::default()
        };

        // 插入数据库
        let res = image.insert(db).await;

        // 处理插入结果
        match res {
            Ok(image) => {
                self.name = image.path;
            }
            Err(err) => {
                self.message = err.to_string();
                self.is_success = false;
            }
        };
    }
    // 删除插入数据库失败的以保存的图片
    pub fn image_remove_fail(&self, path: String) {
        match fs::remove_file(path.clone()) {
            Ok(_) => (),
            Err(e) => {
                let msg = match e.kind() {
                    io::ErrorKind::NotFound => {
                        format!("remove image path {} not found; ", path)
                    }
                    io::ErrorKind::PermissionDenied => {
                        format!("remove image path {} permission denied; ", path)
                    }
                    _ => format!("remove image path {} unknown error; ", path),
                };
                tracing::error!("{}", msg);
            }
        }
    }
}

// 将图片上传信息转换为响应信息
pub fn image_into_res(dto: &Vec<ImageUploadDto>) -> Vec<ImageUploadInfo> {
    let res = dto
        .iter()
        .map(|dto| ImageUploadInfo {
            name: dto.name.clone(),
            is_success: dto.is_success,
            message: dto.message.clone(),
        })
        .collect::<Vec<ImageUploadInfo>>();
    res
}

#[derive(Debug, Default, Serialize)]
pub struct ImageUploadInfo {
    pub name: String,
    pub is_success: bool,
    pub message: String,
}

#[derive(Deserialize, Debug, Extractible)]
#[salvo(extract(default_source(from = "query")))]
pub struct ImageListRequest {
    #[salvo(extract(flatten))]
    pub pagination: PaginationRequest,
}
impl ImageListRequest {
    pub async fn get_list_from_db(&self, db: &DatabaseConnection) -> AppResult<ImageListResponse> {
        let query = self.pagination.get_query::<image::Entity>();

        // if let Some(keyword) = self.pagination.keyword.as_deref() {
        //     query = query.filter(
        //         image::Column::Name
        //             .contains(keyword)
        //     );
        // }

        // if let Some(role) = self.role.clone() {
        //     query = query.filter(user::Column::Role.eq(role));
        // }

        let count = query.clone().count(db).await?;

        let (page, limit) = self.pagination.get_page_limit();

        let images = query.paginate(db, limit).fetch_page(page).await?;

        let images_info = images.into_iter().map(|image| image.into()).collect();

        Ok(ImageListResponse {
            count,
            list: images_info,
        })
    }
}

#[derive(Debug, Serialize, Default)]
pub struct ImageListResponse {
    pub count: u64,
    pub list: Vec<ImageInfoResponse>,
}
#[derive(Debug, Serialize)]
pub struct ImageInfoResponse {
    pub id: u32,
    pub name: String,
    pub path: String,
    pub image_type: String,
    pub size: u32,
    pub hash: String,
    pub created_at: DateTime,
}

impl From<image::Model> for ImageInfoResponse {
    fn from(user: image::Model) -> Self {
        Self {
            id: user.id,
            name: user.name,
            path: user.path,
            image_type: user.image_type,
            size: user.size,
            hash: user.hash,
            created_at: user.created_at,
        }
    }
}

impl EntityWithId for entity::image::Entity {
    type IdColumn = entity::image::Column;
    const ID_COLUMN: Self::IdColumn = entity::image::Column::Id;


}
