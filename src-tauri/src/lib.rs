// [dependencies]
// gtk = "0.18.1"
// gtk-layer-shell = "0.8.1"

use std::{thread, time};

use gtk::ffi::gtk_css_provider_error_get_type;
use gtk::gdk::Screen;
use gtk::CssProvider;
use gtk::StyleContext;
use gtk::{
    prelude::{ContainerExt, CssProviderExt, GtkWindowExt, StyleContextExt, WidgetExt},
    ApplicationWindow,
};
use gtk_layer_shell::LayerShell;
use tauri::{Manager, Url};

struct AppState {
    popup: ApplicationWindow,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn open_window(app: tauri::AppHandle) {
    let data = app.state::<AppState>();

    let mut webview = app.get_webview_window("label").unwrap();

    data.popup.show_all();

    let ten_millis = time::Duration::from_millis(10000);

    thread::sleep(ten_millis);

    println!("{}", webview.url().unwrap());
    // webview.navigate(tauri::Url("dupa".into()));
    webview.navigate(Url::parse("tauri://localhost/lll").unwrap());

    thread::sleep(ten_millis);

    data.popup.hide();
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
            gtk_window.set_layer(gtk_layer_shell::Layer::Top);

            gtk_window.set_anchor(gtk_layer_shell::Edge::Bottom, true);
            gtk_window.set_exclusive_zone(48);

            gtk_window.set_width_request(1080);
            gtk_window.set_height_request(48);

            gtk_window.show_all();

            let popup_window = tauri::WebviewWindowBuilder::new(
                app,
                "label",
                tauri::WebviewUrl::App("wwww".into()),
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

            popup.set_anchor(gtk_layer_shell::Edge::Right, true);

            popup.set_width_request(200);
            popup.set_height_request(200);

            app.manage(AppState { popup });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open_window, greet])
        .run(tauri::generate_context!())
        .unwrap();
}
