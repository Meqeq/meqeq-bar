mod commands;
mod gtk_utils;
mod hyprland_utils;
mod pipewire_utils;

use std::sync::Mutex;

use commands::{on_active_window_change, on_workspace_add, on_workspace_remove, AppState};
use gtk::prelude::{ContainerExt, GtkWindowExt, WidgetExt};
use gtk_layer_shell::LayerShell;
use hyprland_utils::{get_current_workspaces, WorkspaceInfo};
use pipewire_utils::set_up_pipewire;

use serde::Deserialize;
use serde::Serialize;
use tauri::{AppHandle, Manager};

#[derive(Serialize, Deserialize)]
struct WorkspacesInfo {
    workspaces: Vec<WorkspaceInfo>,
    active: i32,
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
fn set_current_workspace(id: i32, app: AppHandle) {
    hyprland_utils::set_current_workspace(id, app);
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

            let workspaces = get_current_workspaces();

            app.manage(Mutex::new(AppState { popup, workspaces }));

            // let mainloop = MainLoop::new(None)?;
            // let context = Context::new(&mainloop)?;
            // let core = context.connect(None)?;
            // let registry = core.get_registry()?;

            // // Register a callback to the `global` event on the registry, which notifies of any new global objects
            // // appearing on the remote.
            // // The callback will only get called as long as we keep the returned listener alive.
            // let _listener = registry
            //     .add_listener_local()
            //     .global(|global| println!("New global: {:?}", global.props.unwrap().get("factory.type.name") ))
            //     .register();

            // // Calling the `destroy_global` method on the registry will destroy the object with the specified id on the remote.
            // // We don't have a specific object to destroy now, so this is commented out.
            // // registry.destroy_global(313).into_result()?;

            // mainloop.run();

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            // open_window,
            // close_window,
            set_up_pipewire,
            
            set_current_workspace,
            on_workspace_add,
            on_workspace_remove,
            on_active_window_change,
        ])
        .run(tauri::generate_context!())
        .unwrap();
}
