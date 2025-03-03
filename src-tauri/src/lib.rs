mod gtk_utils;
mod hyprland_utils;

use gtk::{
    prelude::{ContainerExt, GtkWindowExt, WidgetExt},
    ApplicationWindow,
};
use gtk_layer_shell::LayerShell;
use hyprland::data::*;
use hyprland::prelude::*;
use hyprland_utils::{get_current_workspaces, WorkspaceInfo};
use serde::Deserialize;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};

struct AppState {
    popup: ApplicationWindow,
    workspaces: Vec<WorkspaceInfo>,
}

use hyprland::event_listener::EventListener;

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

#[derive(Serialize, Deserialize)]
struct Kek {
    class: String,
    title: String,
}

#[derive(Serialize, Deserialize)]
struct WorkspacesInfo {
    workspaces: Vec<WorkspaceInfo>,
    active: i32,
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_window(app: AppHandle) {
    let data = app.state::<AppState>();
    data.popup.show_all();
}

#[tauri::command]
async fn close_window(app: AppHandle) {
    let data = app.state::<AppState>();
    data.popup.hide();
}

#[tauri::command]
fn set_current_workspace(id: i32) {
    hyprland_utils::set_current_workspace(id);
}

#[tauri::command]
async fn active_window(app: AppHandle) {
    let mut listener = EventListener::new();

    // let monitors = Monitors::get().unwrap().to_vec();
    // println!("{monitors:#?}");

    // let workspaces = Workspaces::get().unwrap().to_vec();
    // println!("{workspaces:#?}");

    // let clients = Clients::get().unwrap().to_vec();
    // println!("{clients:#?}");

    let active_window = Client::get_active().unwrap().unwrap();
    // println!("{active_window:#?}");

    let stringified = serde_json::to_string(&Kek {
        class: active_window.class,
        title: active_window.title,
    })
    .unwrap();

    app.emit("active_window_change", stringified).unwrap();

    
    let state = app.state::<AppState>();
    app.emit("workspaces", serde_json::to_string(&state.workspaces).unwrap()).unwrap();

    listener.add_active_window_changed_handler(move |data| {
        let event_data = data.unwrap();

        //    let state = app.state::<AppState>();
        // let workspace_info = serde_json::to_string(&WorkspacesInfo {
        //     workspaces: state.workspaces,
        //     active: Workspace::get_active().unwrap().id,
        // }).unwrap();

        let active_window = serde_json::to_string(&Kek {
            class: event_data.class,
            title: event_data.title,
        })
        .unwrap();

        app.emit("active_window_change", active_window).unwrap();
        app.emit(
            "active_workspace_change",
            Workspace::get_active().unwrap().id,
        )
        .unwrap();
    });

    // listener.add_workspace_changed_handler(|data| {
    //     let workspaces = Workspaces::get().unwrap().to_vec();
    //     // println!("{workspaces:#?}");
    //     // println!("{:?}", data);
    // });

    listener.start_listener().unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.hide().unwrap();

            let monitor_info = gtk_utils::get_monitor_info();

            for info in monitor_info {
                gtk_utils::display_status_bar(app, &info);
            }

            let popup_window = tauri::WebviewWindowBuilder::new(
                app,
                "label",
                tauri::WebviewUrl::App("/popup/calendar".into()),
            )
            .transparent(true)
            .build()?;

            popup_window.hide().unwrap();

            let popup = gtk::ApplicationWindow::new(
                &popup_window.gtk_window().unwrap().application().unwrap(),
            );

            popup.set_app_paintable(true);

            let vbox = popup_window.default_vbox().unwrap();

            popup_window.gtk_window().unwrap().remove(&vbox);
            popup.add(&vbox);

            // Doesn't throw errors.
            popup.init_layer_shell();

            // Just works.
            popup.set_layer(gtk_layer_shell::Layer::Top);

            popup.set_width_request(1080);
            popup.set_height_request(1920);

            app.manage(AppState {
                popup,
                workspaces: get_current_workspaces(),
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            open_window,
            greet,
            close_window,
            active_window,
            set_current_workspace
        ])
        .run(tauri::generate_context!())
        .unwrap();
}
