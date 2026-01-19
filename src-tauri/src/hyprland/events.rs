use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc::Receiver;

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

#[derive(Debug)]
pub enum HyprlandEvent {
    ActiveWorkspaceChange(i32),
    ActiveWindowChange(ActiveWindow),
    WorkspacesChange(Vec<WorkspaceInfo>),
}

pub async fn handle_events(app: &AppHandle, event_rx: &mut Receiver<HyprlandEvent>) {
    while let Some(message) = event_rx.recv().await {
        match message {
            HyprlandEvent::ActiveWindowChange(window) => {
                app.emit(
                    "active_window_change",
                    serde_json::to_string(&window).unwrap(),
                )
                .unwrap();
            }
            HyprlandEvent::WorkspacesChange(workspaces) => {
                app.emit("workspaces", serde_json::to_string(&workspaces).unwrap())
                    .unwrap();
            }
            HyprlandEvent::ActiveWorkspaceChange(workspace) => {
                app.emit(
                    "active_workspace_change",
                    serde_json::to_string(&workspace).unwrap(),
                )
                .unwrap();
            }
        }
    }
}
