use std::sync::Mutex;

use hyprland::{
    data::{Client, Workspace, Workspaces},
    dispatch::Dispatch,
    event_listener::{AsyncEventListener, WindowEventData, WorkspaceEventData},
    shared::{HyprData, HyprDataActive, HyprDataActiveOptional},
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};

use crate::app_state::AppState;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub id: i32,
    pub name: String,
    pub monitor: i128,
    pub monitor_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActiveWindow {
    pub class: String,
    pub title: String,
}

pub async fn get_current_workspaces() -> Vec<WorkspaceInfo> {
    let mut workspaces = Vec::new();

    for workspace in Workspaces::get_async().await.unwrap() {
        // println!("{:?}", workspace);
        workspaces.push(WorkspaceInfo {
            id: workspace.id,
            name: workspace.name,
            monitor_name: workspace.monitor,
            monitor: workspace.monitor_id,
        });
    }

    workspaces
}

pub async fn get_active_window() -> ActiveWindow {
    let window = Client::get_active_async().await.unwrap();

    match window {
        Some(active_window) => {
            return ActiveWindow {
                class: active_window.class,
                title: active_window.title,
            }
        }
        None => {
            return ActiveWindow {
                class: "".to_string(),
                title: "".to_string(),
            }
        }
    }
}

pub fn set_current_workspace(id: i32, app: AppHandle) {
    match Dispatch::call(hyprland::dispatch::DispatchType::Workspace(
        hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(id),
    )) {
        Ok(res) => res,
        Err(_) => {
            let state = app.state::<Mutex<AppState>>();
            let state = state.lock().unwrap();

            let workspace = &state.workspaces[id as usize - 1];

            Dispatch::call(hyprland::dispatch::DispatchType::FocusMonitor(
                hyprland::dispatch::MonitorIdentifier::Name(workspace.monitor_name.as_str()),
            ))
            .unwrap();
        }
    }
}

async fn on_workspace_add(app: AppHandle) {
    let workspace = Workspace::get_active_async().await.unwrap();

    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    state.add_workspace(WorkspaceInfo {
        id: workspace.id,
        name: workspace.name,
        monitor_name: workspace.monitor,
        monitor: workspace.monitor_id,
    });

    app.emit(
        "workspaces",
        serde_json::to_string(&state.workspaces).unwrap(),
    )
    .unwrap();
}

async fn on_workspace_remove(data: WorkspaceEventData, app: AppHandle) {
    let state = app.state::<Mutex<AppState>>();
    let mut state = state.lock().unwrap();

    state.remove_workspace(data.id);

    app.emit(
        "workspaces",
        serde_json::to_string(&state.workspaces).unwrap(),
    )
    .unwrap();
}

async fn on_active_window_change(data: WindowEventData, app: AppHandle) {
    let active_window = serde_json::to_string(&ActiveWindow {
        class: data.class,
        title: data.title,
    })
    .unwrap();

    app.emit("active_window_change", active_window).unwrap();
    app.emit(
        "active_workspace_change",
        Workspace::get_active_async().await.unwrap().id,
    )
    .unwrap();
}

pub async fn init_hyprland(app: AppHandle) {
    let mut listener = AsyncEventListener::new();

    let handle = app.clone();
    listener.add_workspace_added_handler(move |_| Box::pin(on_workspace_add(handle.clone())));

    let handle = app.clone();
    listener.add_workspace_deleted_handler(move |data| {
        Box::pin(on_workspace_remove(data, handle.clone()))
    });

    listener.add_active_window_changed_handler(move |data| {
        Box::pin(on_active_window_change(data.unwrap(), app.clone()))
    });

    let _ = listener.start_listener_async().await.unwrap();
}
