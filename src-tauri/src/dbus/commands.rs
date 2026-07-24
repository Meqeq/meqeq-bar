use strum_macros::AsRefStr;
use tauri::{AppHandle, Manager, command};

use crate::state::{commands::Command, state::AppState};

#[derive(Debug, AsRefStr)]
pub enum DbusCommand {
    CallMenuEntry(String, i32),
}

#[command]
pub async fn dbus_tray_item_call_menu(item_id: String, entry_id: i32, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Dbus(DbusCommand::CallMenuEntry(item_id, entry_id)))
        .await;
}
