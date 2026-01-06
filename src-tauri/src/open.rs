use crate::errors::{AppError, Result};
use crate::models::Item;

pub fn open_item(item: &Item) -> Result<()> {
    match item.item_type.as_str() {
        "url" | "file" | "image" => {
            open::that(&item.payload).map_err(|err| AppError::InvalidItem(err.to_string()))?;
            Ok(())
        }
        "text" => {
            let mut clipboard = arboard::Clipboard::new()
                .map_err(|err| AppError::Clipboard(err.to_string()))?;
            clipboard
                .set_text(item.payload.clone())
                .map_err(|err| AppError::Clipboard(err.to_string()))?;
            Ok(())
        }
        other => Err(AppError::InvalidItem(format!(
            "Unsupported item type {other}"
        ))),
    }
}
