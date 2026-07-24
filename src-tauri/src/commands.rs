use std::process::Command;
use tauri::{AppHandle, Emitter, Manager, command};

use crate::state::state::AppState;

#[command]
pub async fn initialize(app: AppHandle) {
    app.state::<AppState>().mark_initialized_bar().await;
}

#[command]
pub async fn set_layer(app: AppHandle, bar: usize, layer: String) {
    let handle = app.clone();
    let layer_clone = layer.clone();

    app.run_on_main_thread(move || {
        handle
            .state::<AppState>()
            .set_layer_for_bar(bar, layer_clone);
    })
    .expect("Error setting layer (main thread)");

    app.emit("bar_set_layer", layer).unwrap();
}

#[command]
pub async fn run_menu() {
    let _ = Command::new("rofi")
        .args(["-show", "drun", "-show-icons"])
        .output();
}

#[command]
pub async fn logout() {
    let _ = Command::new("hyprshutdown").output();
}

#[command]
pub async fn restart() {
    let _ = Command::new("hyprshutdown")
        .args(["-t", "Restarting...", "--post-cmd", "reboot"])
        .spawn();
}

#[command]
pub async fn poweroff() {
    let _ = Command::new("hyprshutdown")
        .args(["-t", "Shutting down...", "--post-cmd", "shutdown -P 0"])
        .spawn();
}
