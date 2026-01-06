use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error: {0}")]
    Db(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Image error: {0}")]
    Image(#[from] image::ImageError),
    #[error("Clipboard error: {0}")]
    Clipboard(String),
    #[error("Invalid item: {0}")]
    InvalidItem(String),
    #[error("Item not found")]
    NotFound,
}

#[cfg(feature = "windows-app")]
impl From<tauri::Error> for AppError {
    fn from(value: tauri::Error) -> Self {
        Self::InvalidItem(value.to_string())
    }
}

#[cfg(feature = "windows-app")]
impl From<tauri_plugin_global_shortcut::Error> for AppError {
    fn from(value: tauri_plugin_global_shortcut::Error) -> Self {
        Self::InvalidItem(value.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
