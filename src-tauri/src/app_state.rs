use gtk::ApplicationWindow;
use serde::{Deserialize, Serialize};
use tauri::App;

use crate::{
    utils::gtk::{get_monitor_info, make_bar},
    utils::hyprland::{get_current_workspaces, WorkspaceInfo},
};

#[derive(Serialize, Deserialize)]
struct WorkspacesInfo {
    workspaces: Vec<WorkspaceInfo>,
    active: i32,
}

#[derive(Debug)]
pub struct AppState {
    pub bars: Vec<ApplicationWindow>,
    pub workspaces: Vec<WorkspaceInfo>,
    initialized: bool,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    pub fn new(app: &App) -> Self {
        let monitor_info = get_monitor_info();

        let bars: Vec<ApplicationWindow> = monitor_info
            .into_iter()
            .map(|monitor| make_bar(app, &monitor))
            .collect();

        let workspaces = get_current_workspaces();

        Self {
            bars,
            workspaces,
            initialized: false,
        }
    }

    pub fn add_workspace(&mut self, workspace: WorkspaceInfo) {
        self.workspaces.push(workspace);
    }

    pub fn remove_workspace(&mut self, id: i32) {
        self.workspaces.retain(|workspace| workspace.id != id);
    }

    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    pub fn initialize(&mut self) {
        self.initialized = true;
    }
}
