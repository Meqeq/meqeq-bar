use strum_macros::AsRefStr;
use tauri::{AppHandle, Manager, command};

use crate::state::{commands::Command, state::AppState};

#[derive(Debug, AsRefStr)]
pub enum HyprlandCommand {
    SetWorkspace(i32),
}

#[command]
pub async fn set_current_workspace(id: i32, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Hyprland(HyprlandCommand::SetWorkspace(id)))
        .await;
}
