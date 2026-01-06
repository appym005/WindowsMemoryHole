use std::sync::Arc;

use parking_lot::Mutex;
use tauri::{AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut};

use crate::errors::Result;
use crate::windows;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverlayState {
    None,
    Hole,
    Wheel,
}

#[derive(Debug)]
pub struct HotkeyState {
    pub current: OverlayState,
    pub locked: bool,
}

impl HotkeyState {
    pub fn new() -> Self {
        Self {
            current: OverlayState::None,
            locked: false,
        }
    }
}

pub fn register_hotkeys(app: &AppHandle, state: Arc<Mutex<HotkeyState>>) -> Result<()> {
    let hole_shortcut = Shortcut::new("Ctrl+Shift+Space")?;
    let wheel_shortcut = Shortcut::new("Ctrl+Shift+E")?;

    let app_handle = app.clone();
    let state_handle = state.clone();
    app.global_shortcut().register(hole_shortcut, move || {
        let mut state = state_handle.lock();
        if state.locked {
            return;
        }
        state.locked = true;
        let result = match state.current {
            OverlayState::Hole => {
                if let Some(window) = app_handle.get_window("hole") {
                    windows::hide_window(&window)
                } else {
                    Ok(())
                }
                .map(|_| {
                    state.current = OverlayState::None;
                })
            }
            OverlayState::Wheel => windows::hide_all(&app_handle).map(|_| {
                windows::show_hole(&app_handle).ok();
                state.current = OverlayState::Hole;
            }),
            OverlayState::None => windows::show_hole(&app_handle).map(|_| {
                state.current = OverlayState::Hole;
            }),
        };
        if result.is_err() {
            state.current = OverlayState::None;
        }
        state.locked = false;
    })?;

    let app_handle = app.clone();
    let state_handle = state.clone();
    app.global_shortcut().register(wheel_shortcut, move || {
        let mut state = state_handle.lock();
        if state.locked {
            return;
        }
        state.locked = true;
        let result = match state.current {
            OverlayState::Wheel => {
                if let Some(window) = app_handle.get_window("wheel") {
                    windows::hide_window(&window)
                } else {
                    Ok(())
                }
                .map(|_| {
                    state.current = OverlayState::None;
                })
            }
            OverlayState::Hole => windows::hide_all(&app_handle).map(|_| {
                windows::show_wheel(&app_handle).ok();
                state.current = OverlayState::Wheel;
            }),
            OverlayState::None => windows::show_wheel(&app_handle).map(|_| {
                state.current = OverlayState::Wheel;
            }),
        };
        if result.is_err() {
            state.current = OverlayState::None;
        }
        state.locked = false;
    })?;

    Ok(())
}
