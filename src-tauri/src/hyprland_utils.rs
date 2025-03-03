use hyprland::{data::Workspaces, dispatch::Dispatch, shared::HyprData};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkspaceInfo {
    id: i32,
    name: String,
    monitor: i128,
}

pub fn get_current_workspaces() -> Vec<WorkspaceInfo> {
    // let workspaces = Workspaces::get().unwrap();

    let mut workspaces = Vec::new();

    for workspace in Workspaces::get().unwrap() {
        workspaces.push(WorkspaceInfo {
            id: workspace.id,
            name: workspace.name,
            monitor: workspace.monitor_id.unwrap().into(),
        });
    }

    workspaces
}

pub fn set_current_workspace(id: i32) {
    match Dispatch::call(hyprland::dispatch::DispatchType::Workspace(
        hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(id),
    )) {
        Ok(res) => res,
        Err(error) => println!("Error: {}", error),
    }
}
