use gtk_layer_shell::{KeyboardMode, Layer, LayerShell};
use std::process::Command;
use tauri::{AppHandle, Emitter, Manager, command};

use crate::app_state::AppState;

#[command]
pub async fn initialize(app: AppHandle) {
    app.state::<AppState>().initialize().await;
}

#[command]
pub async fn set_layer(app: AppHandle, bar: usize, layer: String) {
    let app_clone = app.clone();
    let layer_clone = layer.clone();

    app.run_on_main_thread(move || {
        let state = app_clone.state::<AppState>();
        let bar = state.bars.get(bar).unwrap();

        if layer_clone == "top" {
            bar.gtk_window.set_keyboard_mode(KeyboardMode::OnDemand);
            bar.gtk_window.set_layer(Layer::Top);
        } else {
            bar.gtk_window.set_keyboard_mode(KeyboardMode::None);
            bar.gtk_window.set_layer(Layer::Bottom);
        }
    })
    .unwrap();

    app.emit("bar_set_layer", layer).unwrap();
}

#[command]
pub async fn run_menu() {
    let _ = Command::new("rofi")
        .args(["-show", "drun", "-show-icons"])
        .output();
}
