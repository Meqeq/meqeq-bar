mod app_state;
mod battery;
mod commands;
mod dbus;
mod hyprland;
mod pipewire;
mod utils;

use app_state::AppState;
use commands::{initialize, logout, poweroff, restart, run_menu, set_layer};

use dbus::run::init_dbus;
use hyprland::{commands::set_current_workspace, init::init_hyprland};
use pipewire::commands::{
    set_default_sink, set_default_source, set_device_mute, set_device_profile, set_device_route,
    set_device_volume, set_node_mute, set_node_volume,
};
use pipewire::run::init_pipewire;
use tauri::{App, Manager};

use utils::gtk::create_bars;

use crate::battery::run::init_battery;
use crate::dbus::commands::dbus_tray_item_call_menu;

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let bars = create_bars(app);

    let hyprland = init_hyprland(app.handle());
    let pipewire = init_pipewire(app.handle());
    let dbus = init_dbus(app.handle());
    init_battery(app.handle());

    app.manage(AppState::new(bars, hyprland, pipewire, dbus));

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup)
        .invoke_handler(tauri::generate_handler![
            initialize,
            set_layer,
            dbus_tray_item_call_menu,
            set_current_workspace,
            set_default_source,
            set_default_sink,
            set_node_volume,
            set_node_mute,
            set_device_volume,
            set_device_mute,
            set_device_route,
            set_device_profile,
            run_menu,
            logout,
            restart,
            poweroff
        ])
        .run(tauri::generate_context!())
        .unwrap();
}
