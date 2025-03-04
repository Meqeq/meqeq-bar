use std::sync::Mutex;

use gtk::ApplicationWindow;
use hyprland::{
    data::{Client, Workspace},
    event_listener::EventListener,
    shared::{HyprDataActive, HyprDataActiveOptional},
};
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Emitter, Manager};

use crate::hyprland_utils::WorkspaceInfo;

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveWindow {
    pub class: String,
    pub title: String,
}

#[derive(Debug)]
pub struct AppState {
    pub popup: ApplicationWindow,
    pub workspaces: Vec<WorkspaceInfo>,
}

impl AppState {
    fn add_workspace(&mut self, workspace: WorkspaceInfo) {
        self.workspaces.push(workspace);
    }

    fn remove_workspace(&mut self, id: i32) {
        self.workspaces.retain(|workspace| workspace.id != id);
    }
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

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
    let active_window = Client::get_active().unwrap().unwrap();
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
