use gtk::{
    gdk::{Display, Monitor},
    prelude::{ContainerExt, GtkWindowExt, MonitorExt, WidgetExt},
    ApplicationWindow,
};
use gtk_layer_shell::LayerShell;
use tauri::WebviewWindow;

#[derive(Debug)]
pub struct MonitorInfo {
    pub monitor: Monitor,
    pub index: i32,
    pub width: i32,
    pub height: i32,
}

pub fn get_monitor_info() -> Vec<MonitorInfo> {
    let display = Display::default().unwrap();
    let monitor_count = display.n_monitors();

    let mut monitors = Vec::with_capacity(monitor_count as usize);

    for i in 0..monitor_count {
        let monitor = display.monitor(i).unwrap();

        let geometry = monitor.geometry();

        monitors.push(MonitorInfo {
            monitor,
            index: i,
            width: geometry.width(),
            height: geometry.height(),
        });
    }
    
    monitors
}

pub fn display_status_bar(app: &tauri::App, info: &MonitorInfo) {
    let window = tauri::WebviewWindowBuilder::new(
        app,
        format!("bar{}", info.index),
        tauri::WebviewUrl::App(info.index.to_string().into()),
    )
    .transparent(true)
    .build()
    .unwrap();

    window.hide().unwrap();

    let gtk_window =
        gtk::ApplicationWindow::new(&window.gtk_window().unwrap().application().unwrap());

    gtk_window.set_app_paintable(true);

    let vbox = window.default_vbox().unwrap();
    window.gtk_window().unwrap().remove(&vbox);
    gtk_window.add(&vbox);

    gtk_window.init_layer_shell();

    gtk_window.set_layer(gtk_layer_shell::Layer::Bottom);
    gtk_window.set_anchor(gtk_layer_shell::Edge::Bottom, true);

    gtk_window.set_exclusive_zone(40);
    gtk_window.set_height_request(40);

    gtk_window.set_width_request(info.width);
    gtk_window.set_monitor(&info.monitor);

    gtk_window.show_all();
}

#[derive(Debug)]
pub struct Popup {
    pub window: ApplicationWindow,
    pub webview_window: WebviewWindow,
}

pub fn create_popup_window(app: &tauri::App, info: &MonitorInfo) -> Popup {
    let webview_window = tauri::WebviewWindowBuilder::new(
        app,
        format!("popup{}", info.index),
        tauri::WebviewUrl::App(format!("popup/{}/default", info.index).into()),
    )
    .transparent(true)
    .build()
    .unwrap();

    webview_window.hide().unwrap();

    let window =
        gtk::ApplicationWindow::new(&webview_window.gtk_window().unwrap().application().unwrap());

    window.set_app_paintable(true);

    let vbox = webview_window.default_vbox().unwrap();

    webview_window.gtk_window().unwrap().remove(&vbox);
    window.add(&vbox);

    window.init_layer_shell();
    window.set_layer(gtk_layer_shell::Layer::Top);

    window.set_width_request(info.width);
    window.set_height_request(info.height);
    window.set_monitor(&info.monitor);

    if info.index == 0 {
        // window.show_all();
        // window.hide();
        // webview_window.open_devtools();
    }

    Popup {
        window,
        webview_window,
    }
}
