#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

use blackhole_wheel::db::Database;
use blackhole_wheel::errors::{AppError, Result};
use blackhole_wheel::hotkeys::HotkeyState;
use blackhole_wheel::ingest;
use blackhole_wheel::models::Item;
use blackhole_wheel::open;
use blackhole_wheel::paths::resolve_paths;
use blackhole_wheel::windows;
use chrono::Utc;
use parking_lot::Mutex;
use tauri::{Manager, State};

struct AppState {
    db: Database,
    paths: paths::Paths,
    hotkey_state: Arc<Mutex<HotkeyState>>,
}

#[tauri::command]
fn add_item(
    item_type: String,
    payload: String,
    title: Option<String>,
    state: State<'_, AppState>,
) -> Result<serde_json::Value> {
    if payload.trim().is_empty() {
        return Err(AppError::InvalidItem("Payload cannot be empty".into()));
    }

    match item_type.as_str() {
        "text" => {}
        "url" => {
            if !ingest::looks_like_url(&payload) {
                return Err(AppError::InvalidItem("Invalid URL".into()));
            }
        }
        "file" | "image" => {
            ingest::validate_file_path(&payload)?;
        }
        other => {
            return Err(AppError::InvalidItem(format!(
                "Unsupported item type {other}"
            )))
        }
    };

    let id = uuid::Uuid::new_v4().to_string();
    let created_at = Utc::now().timestamp();
    let thumb_path = if item_type == "image" {
        ingest::maybe_generate_thumbnail(&state.paths, std::path::Path::new(&payload))?
    } else {
        None
    };

    let item = Item {
        id: id.clone(),
        created_at,
        item_type,
        title,
        payload,
        thumb_path: thumb_path.map(|p| p.to_string_lossy().to_string()),
        pinned: 0,
    };

    state.db.insert_item(&item)?;

    Ok(serde_json::json!({ "id": id }))
}

#[tauri::command]
fn list_items(
    limit: Option<i64>,
    include_pinned: Option<bool>,
    state: State<'_, AppState>,
) -> Result<Vec<Item>> {
    let limit = limit.unwrap_or(16);
    let include_pinned = include_pinned.unwrap_or(true);
    state.db.list_items(limit, include_pinned)
}

#[tauri::command]
fn open_item(id: String, state: State<'_, AppState>) -> Result<serde_json::Value> {
    let item = state.db.get_item(&id)?;
    open::open_item(&item)?;
    Ok(serde_json::json!({ "ok": true }))
}

#[tauri::command]
fn delete_item(id: String, state: State<'_, AppState>) -> Result<serde_json::Value> {
    let item = state.db.delete_item(&id)?;
    if item.item_type == "image" {
        if let Some(thumb) = &item.thumb_path {
            let _ = std::fs::remove_file(thumb);
        }
    }
    Ok(serde_json::json!({ "ok": true }))
}

#[tauri::command]
fn clipboard_snapshot() -> Result<serde_json::Value> {
    let mut clipboard = arboard::Clipboard::new()
        .map_err(|err| AppError::Clipboard(err.to_string()))?;

    if clipboard.get_image().is_ok() {
        return Ok(serde_json::json!({ "kind": "image" }));
    }

    match clipboard.get_text() {
        Ok(text) => Ok(serde_json::json!({ "kind": "text", "text": text })),
        Err(_) => Ok(serde_json::json!({ "kind": "empty" })),
    }
}

#[tauri::command]
fn save_clipboard_image(state: State<'_, AppState>) -> Result<serde_json::Value> {
    let (image_path, thumb_path) = ingest::save_clipboard_image(&state.paths)?;
    Ok(serde_json::json!({
        "path": image_path.to_string_lossy(),
        "thumb_path": thumb_path.map(|p| p.to_string_lossy().to_string())
    }))
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .setup(|app| {
            let paths = resolve_paths()?;
            let db = Database::new(&paths.db_path)?;
            let hotkey_state = Arc::new(Mutex::new(HotkeyState::new()));
            windows::create_overlay_windows(app.handle())?;
            hotkeys::register_hotkeys(app.handle(), hotkey_state.clone())?;

            app.manage(AppState {
                db,
                paths,
                hotkey_state,
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_item,
            list_items,
            open_item,
            delete_item,
            clipboard_snapshot,
            save_clipboard_image,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
