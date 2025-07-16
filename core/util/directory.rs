use std::{fs, path::Path};

use anyhow::Context;
use common::AppResult;

pub fn ensure_directory(dir: &str) -> AppResult<()> {
    let path = Path::new(dir);

    fs::create_dir_all(path).context("create directory failed")?;

    Ok(())
}
