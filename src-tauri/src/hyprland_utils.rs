use std::sync::Mutex;

use hyprland::{
    data::{Workspaces},
    dispatch::Dispatch,
    shared::{Address, HyprData},
};
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

use crate::commands::AppState;

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceInfo {
    pub id: i32,
    pub name: String,
    pub monitor: i128,
    pub monitor_name: String,
    pub last_window: Address,
}

pub fn get_current_workspaces() -> Vec<WorkspaceInfo> {
    // let workspaces = Workspaces::get().unwrap();

    let mut workspaces = Vec::new();

    for workspace in Workspaces::get().unwrap() {
        println!("{:?}", workspace);
        workspaces.push(WorkspaceInfo {
            id: workspace.id,
            name: workspace.name,
            monitor_name: workspace.monitor,
            last_window: workspace.last_window,
            monitor: workspace.monitor_id.unwrap().into(),
        });
    }

    workspaces
}

pub fn set_current_workspace(id: i32, app: AppHandle) {
    match Dispatch::call(hyprland::dispatch::DispatchType::Workspace(
        hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(id),
    )) {
        Ok(res) => res,
        Err(error) => {
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
