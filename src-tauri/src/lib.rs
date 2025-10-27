mod app_state;
mod commands;
mod dbus;
mod pipewire;
mod utils;

use app_state::AppState;
use commands::{call_tray_menu_item, initialize, run_menu, set_current_workspace, set_layer};

use pipewire::commands::{
    set_default_sink, set_default_source, set_device_mute, set_device_profile, set_device_route,
    set_device_volume, set_node_mute, set_node_volume,
};
use tauri::{App, Manager};

fn setup<'a>(app: &'a mut App) -> Result<(), Box<dyn std::error::Error>> {
    tauri::async_runtime::block_on(async move {
        let state = AppState::new(&app).await;
        app.manage(state);
    });

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            initialize,
            set_layer,
            call_tray_menu_item,
            set_current_workspace,
            set_default_source,
            set_default_sink,
            set_node_volume,
            set_node_mute,
            set_device_volume,
            set_device_mute,
            set_device_route,
            set_device_profile,
            run_menu
        ])
        .run(tauri::generate_context!())
        .unwrap();
}
