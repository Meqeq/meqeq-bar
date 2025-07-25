use gtk::{
    gdk::{Display, Monitor},
    prelude::{ContainerExt, GtkWindowExt, MonitorExt, WidgetExt},
    ApplicationWindow,
};
use gtk_layer_shell::{Edge, Layer, LayerShell};
use tauri::{App, WebviewUrl, WebviewWindowBuilder};

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

pub fn make_bar(app: &App, monitor: &MonitorInfo) -> ApplicationWindow {
    // println!("WINDOW: {}", monitor.index);
    // println!("{}", format!("/bar/{}", monitor.index));

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

    gtk_window.set_exclusive_zone(50);
    gtk_window.set_height_request(monitor.height);
    gtk_window.set_width_request(monitor.width);
    gtk_window.set_monitor(&monitor.monitor);

    gtk_window.show_all();

    window.hide().unwrap();

    gtk_window
}
