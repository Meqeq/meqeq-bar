use hyprland::dispatch::Dispatch;
use tauri::{command, AppHandle, Manager};
use tokio::sync::mpsc::Receiver;

use crate::app_state::AppState;

#[derive(Debug)]
pub enum HyprlandCommand {
    SetWorkspace(i32),
}

#[command]
pub async fn set_current_workspace(id: i32, app: AppHandle) {
    app.state::<AppState>()
        .hyprland
        .run_command(HyprlandCommand::SetWorkspace(id))
        .await;
}

pub async fn handle_commands(command_rx: &mut Receiver<HyprlandCommand>) {
    while let Some(message) = command_rx.recv().await {
        match message {
            HyprlandCommand::SetWorkspace(id) => {
                Dispatch::call(hyprland::dispatch::DispatchType::Workspace(
                    hyprland::dispatch::WorkspaceIdentifierWithSpecial::Id(id),
                ))
                .unwrap();
            }
        }
    }
}
