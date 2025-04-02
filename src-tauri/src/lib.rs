mod commands;
mod dbus;
mod gtk_utils;
mod hyprland_utils;
mod pipewire_utils;

use std::sync::Mutex;

use commands::{
    dbus, on_active_window_change, on_workspace_add, on_workspace_remove, set_default, set_volume,
    AppState,
};
use commands::{initialize, set_layer};
use gtk::glib::ObjectExt;
use gtk::prelude::BuildableExt;
use gtk::prelude::ContainerExt;
use gtk::prelude::GtkWindowExt;
use gtk::prelude::WidgetExt;
use gtk::ApplicationWindow;
use gtk_layer_shell::Edge;
use gtk_layer_shell::Layer;
use gtk_layer_shell::LayerShell;
use gtk_utils::MonitorInfo;
use hyprland_utils::{get_current_workspaces, WorkspaceInfo};
use pipewire_utils::set_up_pipewire;

use serde::Deserialize;
use serde::Serialize;
use tauri::App;
use tauri::Emitter;
use tauri::WebviewUrl;
use tauri::WebviewWindowBuilder;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize)]
struct WorkspacesInfo {
    workspaces: Vec<WorkspaceInfo>,
    active: i32,
}

#[tauri::command]
fn set_current_workspace(id: i32, app: AppHandle) {
    hyprland_utils::set_current_workspace(id, app);
}

fn make_bar(app: &App, monitor: &MonitorInfo) -> ApplicationWindow {
    println!("WINDOW: {}", monitor.index);
    println!("{}", format!("/bar/{}", monitor.index));

    let window = WebviewWindowBuilder::new(
        app,
        format!("bar{}", monitor.index),
        WebviewUrl::App(format!("/bar/{}", monitor.index).into()),
    )
    .transparent(true)
    .build()
    .unwrap();

    let gtk_window = ApplicationWindow::new(&window.gtk_window().unwrap().application().unwrap());

    gtk_window.set_app_paintable(true);

    let gtk_box = window.default_vbox().unwrap();
    window.gtk_window().unwrap().remove(&gtk_box);
    gtk_window.add(&gtk_box);

    gtk_window.init_layer_shell();
    gtk_window.set_layer(Layer::Bottom);
    gtk_window.set_anchor(Edge::Bottom, true);

    gtk_window.set_exclusive_zone(40);
    gtk_window.set_height_request(monitor.height);
    gtk_window.set_width_request(monitor.width);
    gtk_window.set_monitor(&monitor.monitor);

    gtk_window.show_all();

    window.hide().unwrap();

    gtk_window
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let monitor_info = gtk_utils::get_monitor_info();

            let bars: Vec<ApplicationWindow> = monitor_info
                .into_iter()
                .map(|monitor| make_bar(app, &monitor))
                .collect();

            let workspaces = get_current_workspaces();

            app.manage(Mutex::new(AppState::new(bars, workspaces)));

            // let w3 = make_window(app, "C".into());
            // let w4 = make_window(app, "D".into());
            // let w5 = make_window(app, "E".into());

            // bar1.gtk_window().unwrap().set_ex

            //             let window = tauri::WebviewWindowBuilder::new(
            //     app,
            //     format!("bar{}", info.index),
            //     tauri::WebviewUrl::App(info.index.to_string().into()),
            // )
            // .transparent(true)
            // .build()
            // .unwrap();

            // window.hide().unwrap();

            // let gtk_window =
            //     gtk::ApplicationWindow::new(&window.gtk_window().unwrap().application().unwrap());

            // gtk_window.set_app_paintable(true);

            // let vbox = window.default_vbox().unwrap();
            // window.gtk_window().unwrap().remove(&vbox);
            // gtk_window.add(&vbox);

            // gtk_window.init_layer_shell();

            // gtk_window.set_layer(gtk_layer_shell::Layer::Bottom);
            // gtk_window.set_anchor(gtk_layer_shell::Edge::Bottom, true);

            // gtk_window.set_exclusive_zone(40);
            // gtk_window.set_height_request(40);

            // gtk_window.set_width_request(info.width);
            // gtk_window.set_monitor(&info.monitor);

            // gtk_window.show_all();

            // let main_window = app.get_webview_window("main").unwrap();
            // main_window.hide().unwrap();

            // let mut popups = Vec::with_capacity(monitor_info.len());

            // for info in &monitor_info {
            //     popups.push(gtk_utils::create_popup_window(app, info));
            // }

            // let workspaces = get_current_workspaces();

            // app.manage(Mutex::new(AppState { popups, workspaces }));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // open_window,
            // close_window,
            initialize,
            dbus,
            set_layer,
            set_volume,
            set_default,
            set_up_pipewire,
            set_current_workspace,
            on_workspace_add,
            on_workspace_remove,
            on_active_window_change,
        ])
        .run(tauri::generate_context!())
        .unwrap();
}
