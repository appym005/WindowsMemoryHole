use std::path::{Path, PathBuf};

use image::{imageops::FilterType, ImageFormat};
use uuid::Uuid;

use crate::errors::{AppError, Result};
use crate::paths::Paths;

pub fn looks_like_url(input: &str) -> bool {
    let trimmed = input.trim();
    !trimmed.is_empty() && regex::Regex::new(r"^https?://[^\s]+$")
        .map(|re| re.is_match(trimmed))
        .unwrap_or(false)
}

pub fn validate_file_path(path: &str) -> Result<()> {
    if path.is_empty() {
        return Err(AppError::InvalidItem("Missing file path".into()));
    }
    let path = Path::new(path);
    if !path.exists() {
        return Err(AppError::InvalidItem("File path does not exist".into()));
    }
    Ok(())
}

pub fn save_clipboard_image(paths: &Paths) -> Result<(PathBuf, Option<PathBuf>)> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|err| AppError::Clipboard(err.to_string()))?;
    let image = clipboard
        .get_image()
        .map_err(|err| AppError::Clipboard(err.to_string()))?;

    let id = Uuid::new_v4().to_string();
    let image_path = paths.images_dir.join(format!("{id}.png"));
    let thumb_path = paths.thumbs_dir.join(format!("{id}_thumb.png"));

    let img_buffer = image::ImageBuffer::<image::Rgba<u8>, _>::from_raw(
        image.width as u32,
        image.height as u32,
        image.bytes.into_owned(),
    )
    .ok_or_else(|| AppError::Clipboard("Invalid image buffer".into()))?;

    let dynamic = image::DynamicImage::ImageRgba8(img_buffer);
    dynamic.save_with_format(&image_path, ImageFormat::Png)?;

    let thumb = dynamic.thumbnail(256, 256);
    thumb.save_with_format(&thumb_path, ImageFormat::Png)?;

    Ok((image_path, Some(thumb_path)))
}

pub fn maybe_generate_thumbnail(paths: &Paths, source: &Path) -> Result<Option<PathBuf>> {
    let Ok(image) = image::open(source) else {
        return Ok(None);
    };
    let id = Uuid::new_v4().to_string();
    let thumb_path = paths.thumbs_dir.join(format!("{id}_thumb.png"));
    let thumb = image.resize(256, 256, FilterType::Triangle);
    thumb.save_with_format(&thumb_path, ImageFormat::Png)?;
    Ok(Some(thumb_path))
}
