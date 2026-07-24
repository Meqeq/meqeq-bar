use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceInfo {
    pub id: i32,
    pub name: String,
    pub monitor: i128,
    pub monitor_name: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ActiveWindow {
    pub class: String,
    pub title: String,
}

#[derive(Debug, AsRefStr, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum HyprlandEvent {
    ActiveWorkspace(i32),
    ActiveWindow(ActiveWindow),
    Workspaces(Vec<WorkspaceInfo>),
}
