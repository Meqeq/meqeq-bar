// mod app_state;
// mod battery;
mod commands;
mod dbus;
mod hyprland;
mod pipewire;
mod state;
mod utils;

use commands::{initialize, logout, poweroff, restart, run_menu, set_layer};

use hyprland::{commands::set_current_workspace, run::run_hyprland};
use pipewire::commands::{
    set_default_sink, set_default_source, set_device_mute, set_device_profile, set_device_route,
    set_device_volume, set_node_mute, set_node_volume,
};
use pipewire::run::run_pipewire;
use tauri::{App, Manager, async_runtime};

use tokio::join;
use tokio::sync::mpsc::channel;

use utils::gtk::create_bars;

// use crate::battery::run::init_battery;
use crate::dbus::commands::dbus_tray_item_call_menu;

use crate::dbus::mpris::{
    commands::{player_next, player_pause, player_play, player_prev, player_seek, player_shuffle},
    run::run_mpris,
};
use crate::dbus::run::run_dbus;
use crate::state::commands::{Command, pass_commands};
use crate::state::events::receive_events;
use crate::state::state::AppState;

fn setup(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    let bars = create_bars(app);

    let (command_tx, mut command_rx) = channel::<Command>(32);

    let app_state = AppState::new(bars, command_tx);
    app.manage(app_state);

    let handle = app.handle().clone();
    async_runtime::spawn(async move {
        handle.state::<AppState>().wait_for_initialize().await;

        let (
            command_listener,
            mut dbus_command_rx,
            mut player_command_rx,
            mut hyprland_command_rx,
            mut pipewire_command_rx,
        ) = pass_commands(&mut command_rx);

        let (dbus_listener, mut dbus_event_rx) = run_dbus(&mut dbus_command_rx);
        let (mpris_listener, mut mpris_event_rx) = run_mpris(&mut player_command_rx);
        let (hyprland_listener, mut hyprland_event_rx) = run_hyprland(&mut hyprland_command_rx);
        let (pipewire_listener, mut pipewire_event_rx) = run_pipewire(&mut pipewire_command_rx);

        let _ = join!(
            dbus_listener,
            mpris_listener,
            hyprland_listener,
            pipewire_listener,
            command_listener,
            receive_events(
                &handle,
                &mut dbus_event_rx,
                &mut mpris_event_rx,
                &mut hyprland_event_rx,
                &mut pipewire_event_rx
            ),
        );
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
            poweroff,
            player_play,
            player_pause,
            player_next,
            player_prev,
            player_seek,
            player_shuffle
        ])
        .run(tauri::generate_context!())
        .unwrap();
}
