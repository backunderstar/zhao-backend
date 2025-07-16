use std::{collections::HashSet, path::Path};

use anyhow::anyhow;
use common::AppResult;


pub fn get_file_extension(file_path: String) -> Option<String> {
    let path = Path::new(&file_path);
    path.extension()
        .and_then(|ext| ext.to_string_lossy().into_owned().into())
}

pub fn is_image(file_path: String) -> bool {
    // 定义常见的图片文件扩展名集合
    let image_extensions: HashSet<&str> = [
        "jpg", "jpeg", "png", "gif", "bmp", "svg", "tiff", "webp", "ico", "heic", "heif",
    ]
    .iter()
    .cloned()
    .collect();

    // 获取文件扩展名并检查是否在图片扩展名集合中
    if let Some(ext) = get_file_extension(file_path) {
        return image_extensions.contains(ext.to_lowercase().as_str());
    }

    false
}

pub fn rename_with_hash(name: String, hash: String) -> AppResult<String> {
    // 截取哈希值的前8位
    let hash_prefix = &hash[..8];

    // 分离文件名和扩展名
    let file_name = Path::new(name.as_str())
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("default");

    let extension = Path::new(name.as_str())
        .extension()
        .and_then(|s| s.to_str())
        .ok_or(anyhow!("Invalid file extension"))?;

    // 生成新的文件名
    Ok(format!("{}_{}.{}", file_name, hash_prefix, extension))
}
