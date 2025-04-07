use std::{process, sync::Mutex};

use gtk::ApplicationWindow;
use serde::{Deserialize, Serialize};
use tauri::App;
use zbus::Connection;

use crate::{
    utils::gtk::{get_monitor_info, make_bar},
    utils::hyprland::WorkspaceInfo,
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
    pub connection: Connection,
    initialized: bool,
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

impl AppState {
    pub async fn new(app: &App) -> Mutex<Self> {
        let connection = Connection::session().await.unwrap();

        connection
            .request_name(format!("org.kde.StatusNotifierHost-{}", process::id()))
            .await
            .unwrap();

        let monitor_info = get_monitor_info();

        let bars: Vec<ApplicationWindow> = monitor_info
            .into_iter()
            .map(|monitor| make_bar(app, &monitor))
            .collect();

        Mutex::new(Self {
            bars,
            workspaces: Vec::new(),
            connection,
            initialized: false,
        })
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

    pub fn initialize(&mut self, mut workspaces: Vec<WorkspaceInfo>) {
        self.initialized = true;
        self.workspaces.append(&mut workspaces);
    }
}
