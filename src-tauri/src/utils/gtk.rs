use gtk::{
    gdk::{self, Display},
    prelude::{ContainerExt, GtkWindowExt, MonitorExt, WidgetExt},
    WindowType,
};
use gtk_layer_shell::{Edge, Layer, LayerShell};

use tauri::{
    App, LogicalPosition, LogicalSize, Manager, Webview, WebviewBuilder, WebviewUrl, Window,
    WindowBuilder,
};

#[derive(Debug)]
struct Monitor {
    pub width: i32,
    pub height: i32,
    pub index: usize,
    pub gtk_handle: gdk::Monitor,
}

#[derive(Debug)]
struct Monitors {
    pub primary: usize,
    pub list: Vec<Monitor>,
}

#[derive(Debug)]
pub struct Bar {
    window: Window,
    webview: Webview,
    pub gtk_window: gtk::Window,
}

fn get_monitors() -> Monitors {
    let display = Display::default().expect("Could not get default display");
    let count: usize = display
        .n_monitors()
        .try_into()
        .expect("Incorrect number of monitors received");

    let mut list = Vec::with_capacity(count);

    let mut primary = 0usize;

    for index in 0..count {
        let gtk_handle = display
            .monitor(index as i32)
            .expect(format!("Could not get monitor {}", index).as_str());
        let geometry = gtk_handle.geometry();

        if gtk_handle.is_primary() {
            primary = index.try_into().unwrap();
        }

        list.push(Monitor {
            index,
            gtk_handle,
            width: geometry.width(),
            height: geometry.height(),
        });
    }

    Monitors { primary, list }
}

fn create_window(app: &App, monitor: &Monitor, index: usize) -> (gtk::Window, Window) {
    let window = WindowBuilder::new(app, format!("bar{}", index))
        .visible(false)
        .build()
        .unwrap();

    let gtk_window = gtk::Window::new(WindowType::Toplevel);

    let gtk_box = window.default_vbox().unwrap();
    window.gtk_window().unwrap().remove(&gtk_box);

    gtk_window.init_layer_shell();

    gtk_window.set_decorated(false);
    gtk_window.set_exclusive_zone(40);
    gtk_window.set_app_paintable(true);
    gtk_window.set_layer(Layer::Bottom);
    gtk_window.set_anchor(Edge::Bottom, true);
    gtk_window.set_monitor(&monitor.gtk_handle);
    gtk_window.set_size_request(monitor.width, monitor.height);

    gtk_window.add(&gtk_box);

    gtk_window.show_all();

    (gtk_window, window)
}

fn create_webview(window: Window, index: usize, related_webview: Option<&Webview>) -> Webview {
    let builder = WebviewBuilder::new(
        format!("bar{}", index),
        WebviewUrl::App(format!("/bar/{}", index).into()),
    )
    .transparent(true);

    let (pos, size) = (LogicalPosition::new(0, 0), LogicalSize::new(1, 1));

    match related_webview {
        #[cfg(target_os = "linux")]
        Some(related) => {
            let window2 = window.clone();

            related
                .with_webview(move |v| {
                    window2
                        .add_child(builder.with_related_view(v.inner()), pos, size)
                        .unwrap();
                })
                .unwrap();
        }
        _ => {
            window.add_child(builder, pos, size).unwrap();
        }
    };

    window
        .get_webview(format!("bar{}", index).as_str())
        .unwrap()
}

fn create_bar(app: &App, monitor: &Monitor, index: usize, related_bar: Option<&Bar>) -> Bar {
    let (gtk_window, window) = create_window(app, monitor, index);
    let webview = create_webview(
        window.clone(),
        index,
        related_bar.map(|related| &related.webview),
    );

    Bar {
        window,
        webview,
        gtk_window,
    }
}

pub fn create_bars(app: &App) -> Vec<Bar> {
    let monitors = get_monitors();

    let primary = create_bar(
        app,
        &monitors.list[monitors.primary],
        monitors.primary,
        None,
    );

    let mut bars: Vec<Bar> = monitors
        .list
        .iter()
        .filter(|monitor| monitor.index != monitors.primary)
        .map(|monitor| create_bar(app, &monitor, monitor.index, Some(&primary)))
        .collect();

    bars.insert(monitors.primary, primary);

    bars
}
