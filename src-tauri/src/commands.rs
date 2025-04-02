use gtk::{prelude::WidgetExt, ApplicationWindow};
use gtk_layer_shell::{Layer, LayerShell};
use hyprland::{
    data::{Client, Workspace},
    event_listener::EventListener,
    shared::{HyprDataActive, HyprDataActiveOptional},
};
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, sync::Mutex};
use tauri::{command, AppHandle, Emitter, Manager};
use tokio::join;

use crate::{
    dbus::{
        status_notifier_host::StatusNotifierHost, status_notifier_watcher::StatusNotifierWatcher,
    },
    gtk_utils::Popup,
    hyprland_utils::WorkspaceInfo,
    pipewire_utils::{self, set_up_pipewire},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveWindow {
    pub class: String,
    pub title: String,
}

#[derive(Debug)]
pub struct AppState {
    pub bars: Vec<ApplicationWindow>,
    pub workspaces: Vec<WorkspaceInfo>,
    initialized: bool,
}

impl AppState {
    pub fn new(bars: Vec<ApplicationWindow>, workspaces: Vec<WorkspaceInfo>) -> Self {
        Self {
            bars,
            workspaces,
            initialized: false,
        }
    }

    fn add_workspace(&mut self, workspace: WorkspaceInfo) {
        self.workspaces.push(workspace);
    }

    fn remove_workspace(&mut self, id: i32) {
        self.workspaces.retain(|workspace| workspace.id != id);
    }

    fn is_initialized(&self) -> bool {
        self.initialized
    }

    fn initialize(&mut self) {
        self.initialized = true;
    }
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

#[command]
pub async fn initialize(app: AppHandle) {
    // invoke("on_workspace_add").then(() => {});
    //  invoke("on_workspace_remove").then(() => {});
    //  invoke("on_active_window_change");
    //  invoke("set_up_pipewire");
    //  invoke("dbus");
    {
        let state = app.state::<Mutex<AppState>>();
        let mut state = state.lock().unwrap();

        if state.is_initialized() {
            return;
        }

        state.initialize();
    }

    let _ = join!(
        tokio::spawn(on_active_window_change(app.clone())),
        tokio::spawn(on_workspace_remove(app.clone())),
        tokio::spawn(on_active_window_change(app.clone())),
        tokio::spawn(set_up_pipewire(app.clone())),
        tokio::spawn(dbus(app.clone())),
    );
}

#[command]
pub async fn on_workspace_add(app: AppHandle) {
    let mut listener = EventListener::new();

    listener.add_workspace_added_handler(move |data| {
        println!("{:?}", data);
        let workspace = Workspace::get_active().unwrap();

        let state = app.state::<Mutex<AppState>>();
        let mut state = state.lock().unwrap();

        state.add_workspace(WorkspaceInfo {
            id: workspace.id,
            name: workspace.name,
            monitor_name: workspace.monitor,
            last_window: workspace.last_window,
            monitor: workspace.monitor_id.unwrap().into(),
        });

        app.emit(
            "workspaces",
            serde_json::to_string(&state.workspaces).unwrap(),
        )
        .unwrap();
    });
    listener.start_listener().unwrap();
}

#[command]
pub async fn on_workspace_remove(app: AppHandle) {
    let mut listener = EventListener::new();

    listener.add_workspace_deleted_handler(move |data| {
        let state = app.state::<Mutex<AppState>>();
        let mut state = state.lock().unwrap();

        state.remove_workspace(data.id);

        app.emit(
            "workspaces",
            serde_json::to_string(&state.workspaces).unwrap(),
        )
        .unwrap();
    });

    listener.start_listener().unwrap();
}

#[command]
pub async fn on_active_window_change(app: AppHandle) {
    match Client::get_active() {
        Ok(window) => match window {
            Some(active_window) => {
                println!("AAA: {:?}", active_window);

                app.emit(
                    "active_window_change",
                    serde_json::to_string(&ActiveWindow {
                        class: active_window.class,
                        title: active_window.title,
                    })
                    .unwrap(),
                )
                .unwrap();

                app.emit("active_workspace_change", active_window.workspace.id)
                    .unwrap();

                {
                    let state = app.state::<Mutex<AppState>>();
                    let state = state.lock().unwrap();

                    app.emit(
                        "workspaces",
                        serde_json::to_string(&state.workspaces).unwrap(),
                    )
                    .unwrap();
                }
            }
            None => {}
        },
        Err(_) => {}
    }

    let mut listener = EventListener::new();

    listener.add_active_window_changed_handler(move |data| {
        let event_data = data.unwrap();

        let active_window = serde_json::to_string(&ActiveWindow {
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

    listener.start_listener().unwrap();
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivePopup {
    pub name: String,
    pub monitor: u32,
}

#[command]
pub async fn set_layer(app: AppHandle, bar: usize, layer: String) {
    let app_clone = app.clone();
    app.run_on_main_thread(move || {
        let state = app_clone.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();
        let window = state.bars.get(bar).unwrap();

        if layer == "top" {
            window.set_layer(Layer::Top);
        } else {
            window.set_layer(Layer::Bottom);
        }
    })
    .unwrap();
}

#[command]
pub async fn set_volume(id: u32, volume: f32) {
    pipewire_utils::set_volume(id, volume);
}

#[command]
pub async fn set_default(id: u32) {
    pipewire_utils::set_default(id);
}

#[command]
pub async fn dbus(app: AppHandle) {
    let notifier_host = StatusNotifierHost::connect(app).await;

    trpl::join(StatusNotifierWatcher::serve(), notifier_host.serve()).await;
}
