use std::path::PathBuf;

use crate::errors::Result;

#[derive(Debug, Clone)]
pub struct Paths {
    pub base_dir: PathBuf,
    pub db_path: PathBuf,
    pub images_dir: PathBuf,
    pub thumbs_dir: PathBuf,
}

pub fn resolve_paths() -> Result<Paths> {
    let base_dir = dirs::data_dir()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "Missing data dir"))?
        .join("BlackHoleWheel");
    let images_dir = base_dir.join("images");
    let thumbs_dir = base_dir.join("thumbs");
    std::fs::create_dir_all(&images_dir)?;
    std::fs::create_dir_all(&thumbs_dir)?;
    let db_path = base_dir.join("db.sqlite");

    Ok(Paths {
        base_dir,
        db_path,
        images_dir,
        thumbs_dir,
    })
}
