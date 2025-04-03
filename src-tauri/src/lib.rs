mod app_state;
mod commands;
mod dbus;
mod utils;

use std::sync::Mutex;

use app_state::AppState;
use commands::{initialize, set_current_workspace, set_default, set_layer, set_volume};

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let state = AppState::new(&app);
            app.manage(Mutex::new(state));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            initialize,
            set_layer,
            set_volume,
            set_default,
            set_current_workspace,
        ])
        .run(tauri::generate_context!())
        .unwrap();
}
