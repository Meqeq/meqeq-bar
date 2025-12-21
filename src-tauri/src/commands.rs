use gtk_layer_shell::{KeyboardMode, Layer, LayerShell};
use std::process::Command;
use tauri::{command, AppHandle, Manager};

use crate::{
    app_state::AppState,
    // dbus::{
    //     dbus_menu::DbusMenuProxy, status_notifier_host::StatusNotifierHost,
    //     status_notifier_watcher::StatusNotifierWatcher,
    // },
    // pipewire::run::run_pipewire,
};

#[command]
pub async fn initialize(app: AppHandle) {
    app.state::<AppState>().initialize().await;

    // app.emit(
    //     "active_window_change",
    //     serde_json::to_string(&active_window).unwrap(),
    // )
    // .unwrap();

    // app.emit("workspaces", serde_json::to_string(&workspaces).unwrap())
    //     .unwrap();

    // let (command_tx, handle) = run_pipewire(app.clone());

    // let res2 = task::spawn_blocking(move || {
    //     handle.join().unwrap();
    // });

    // {
    //     let state = app.state::<Mutex<AppState>>();
    //     let mut state = state.lock().unwrap();

    //     state.set_pw_sender(command_tx);

    //     if state.is_initialized() {
    //         return;
    //     }

    //     state.initialize(workspaces);
    // }

    // // let aaa = kek(app.clone());

    // // let res = task::spawn_blocking(move || {
    // //     aaa.join().unwrap();
    // // });

    // let _ = join!(
    //     init_hyprland(app.clone()),
    //     // init_pipewire(app.clone()),
    //     init_dbus(app.clone()),
    //     // res,
    //     res2
    // );
}

#[command]
pub async fn set_layer(app: AppHandle, bar: usize, layer: String) {
    let app_clone = app.clone();
    app.run_on_main_thread(move || {
        let state = app_clone.state::<AppState>();
        let bar = state.bars.get(bar).unwrap();

        if layer == "top" {
            bar.gtk_window.set_keyboard_mode(KeyboardMode::OnDemand);
            bar.gtk_window.set_layer(Layer::Top);
        } else {
            bar.gtk_window.set_keyboard_mode(KeyboardMode::None);
            bar.gtk_window.set_layer(Layer::Bottom);
        }
    })
    .unwrap();
}

// #[command]
// pub async fn call_tray_menu_item(service: String, path: String, id: i32, app: AppHandle) {
//     // let connection = {
//     //     let state = app.state::<Mutex<AppState>>();
//     //     let state = state.lock().unwrap();
//     //     state.connection.clone()
//     // };

//     // let proxy = DbusMenuProxy::new(&connection, service, path)
//     //     .await
//     //     .unwrap();

//     // proxy.event(id, "clicked", &Value::I32(0), 0).await.unwrap();
// }

// #[command]
// pub async fn init_dbus(app: AppHandle) {
//     // let notifier_host = StatusNotifierHost::connect(app).await;

//     // join!(StatusNotifierWatcher::serve(), notifier_host.serve());
// }

#[command]
pub async fn run_menu() {
    let _ = Command::new("rofi")
        .args(["-show", "drun", "-show-icons"])
        .output();
}
