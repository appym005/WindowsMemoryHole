use tauri::{AppHandle, Manager, Window};

use crate::errors::Result;

pub fn create_overlay_windows(app: &AppHandle) -> Result<()> {
    let hole = tauri::WindowBuilder::new(
        app,
        "hole",
        tauri::WindowUrl::App("index.html?window=hole".into()),
    )
    .title("Hole")
    .inner_size(420.0, 220.0)
    .transparent(true)
    .decorations(false)
    .always_on_top(true)
    .visible(false)
    .build()?;

    let wheel = tauri::WindowBuilder::new(
        app,
        "wheel",
        tauri::WindowUrl::App("index.html?window=wheel".into()),
    )
    .title("Wheel")
    .transparent(true)
    .decorations(false)
    .always_on_top(true)
    .fullscreen(true)
    .visible(false)
    .build()?;

    hide_window(&hole)?;
    hide_window(&wheel)?;

    Ok(())
}

pub fn show_window(window: &Window) -> Result<()> {
    window.show()?;
    window.set_focus()?;
    Ok(())
}

pub fn hide_window(window: &Window) -> Result<()> {
    window.hide()?;
    Ok(())
}

pub fn show_hole(app: &AppHandle) -> Result<()> {
    let window = app.get_window("hole").expect("hole window");
    if let Some(monitor) = app.primary_monitor()? {
        let size = monitor.size();
        let pos_x = (size.width as f64 / 2.0) - 210.0;
        let pos_y = (size.height as f64 / 2.0) - 110.0;
        window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
            x: pos_x as i32,
            y: pos_y as i32,
        }))?;
    }
    show_window(&window)
}

pub fn show_wheel(app: &AppHandle) -> Result<()> {
    let window = app.get_window("wheel").expect("wheel window");
    show_window(&window)
}

pub fn hide_all(app: &AppHandle) -> Result<()> {
    if let Some(window) = app.get_window("hole") {
        hide_window(&window)?;
    }
    if let Some(window) = app.get_window("wheel") {
        hide_window(&window)?;
    }
    Ok(())
}
