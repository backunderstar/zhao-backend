use std::{fs, path::Path};

use anyhow::anyhow;
use common::{EmptyResult, JsonResult, empty_ok, json_ok};
use db::pool;
use dto::{
    IDListRequest, ImageListRequest, ImageListResponse, ImageUploadDto, ImageUploadInfo,
    image_into_res,
};
use entity::prelude::Image;
use salvo::prelude::*;
use util::ensure_directory;

#[handler]
pub async fn upload(req: &mut Request) -> JsonResult<Vec<ImageUploadInfo>> {
    let db = pool();
    // 解析请求，获取文件
    let files = req
        .files("images")
        .await
        .ok_or(anyhow::anyhow!("images not found in request"))?;

    let mut dtos = ImageUploadDto::new(files);

    ensure_directory("upload/image")?;

    for dto in &mut dtos {
        if !dto.is_success {
            continue;
        }

        dto.image_hash(&db).await?;

        if !dto.is_success {
            continue;
        }

        // 重命名：name_hash[..8].ext
        dto.image_rename();

        // 跳过重命名失败的图片
        if !dto.is_success {
            continue;
        }

        // 复制保存图片
        dto.image_save();

        // 跳过复制失败的图片
        if !dto.is_success {
            continue;
        }

        // 插入数据库
        dto.insert_into_db(db).await;

        // 跳过插入数据库成功的图片
        if dto.is_success {
            continue;
        }

        // 删除插入数据库失败的图片
        dto.image_remove_fail(dto.path.clone());
    }

    json_ok(image_into_res(&dtos))
}

#[handler]
pub async fn list(req: &mut Request) -> JsonResult<ImageListResponse> {
    let db = pool();

    let list_req: ImageListRequest = req.extract().await?;

    let res = list_req.get_list_from_db(db).await?;

    json_ok(res)
}

#[handler]
pub async fn delete(req: &mut Request) -> EmptyResult {
    let ids: IDListRequest = req.extract().await?;

    let db = pool();

    let images = ids.get_datas_by_ids::<Image>(db).await?;

    ids.delete_datas_by_ids::<Image>(db).await?;

    for image in images {
        let path = image.path;

        let path = Path::new(&path);

        if path.exists() {
            fs::remove_file(path).map_err(|e| anyhow!("删除图片(id:{})失败: {}", image.id, e))?;
        }
    }

    empty_ok()
}
