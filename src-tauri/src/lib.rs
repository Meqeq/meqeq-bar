// [dependencies]
// gtk = "0.18.1"
// gtk-layer-shell = "0.8.1"

use std::{thread, time};

use gtk::ffi::gtk_css_provider_error_get_type;
use gtk::gdk::Screen;
use gtk::glib::Propagation;
use gtk::CssProvider;
use gtk::EventBox;
use gtk::StyleContext;
use gtk::{
    prelude::{ContainerExt, CssProviderExt, GtkWindowExt, StyleContextExt, WidgetExt},
    ApplicationWindow,
};
use gtk_layer_shell::LayerShell;
use hyprland::event_listener::WindowFloatEventData;
use serde::Deserialize;
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager};
struct AppState {
    popup: ApplicationWindow,
}

use hyprland::event_listener::EventListener;

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

#[derive(Serialize, Deserialize)]
struct Kek {
    class: String,
    title: String,
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
fn active_window(app: AppHandle) {
    // let mut listener = EventListener::new();

    // listener.add_active_window_changed_handler(|data| {
    //     let event_data = data.unwrap();
    //     let stringified = serde_json::to_string(&Kek {
    //         class: event_data.class,
    //         title: event_data.title,
    //     })
    //     .unwrap();

    //     app.emit("active_window_changed", "stringified").unwrap();
    // });

    // listener.start_listener();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let main_window = app.get_webview_window("main").unwrap();
            main_window.hide().unwrap();

            let gtk_window = gtk::ApplicationWindow::new(
                &main_window.gtk_window().unwrap().application().unwrap(),
            );

            // To prevent the window from being black initially.
            gtk_window.set_app_paintable(true);

            let vbox = main_window.default_vbox().unwrap();
            main_window.gtk_window().unwrap().remove(&vbox);
            gtk_window.add(&vbox);

            // Doesn't throw errors.
            gtk_window.init_layer_shell();

            // Just works.
            gtk_window.set_layer(gtk_layer_shell::Layer::Bottom);

            gtk_window.set_anchor(gtk_layer_shell::Edge::Bottom, true);
            gtk_window.set_exclusive_zone(48);

            gtk_window.set_width_request(1080);
            gtk_window.set_height_request(48);

            gtk_window.show_all();

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

            app.manage(AppState { popup });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_window, greet, close_window])
        .run(tauri::generate_context!())
        .unwrap();
}
