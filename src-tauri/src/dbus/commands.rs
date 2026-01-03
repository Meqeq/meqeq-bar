use tauri::{command, AppHandle, Manager};
use tokio::sync::mpsc::Receiver;

use crate::app_state::AppState;

pub enum DbusCommand {
    Kek,
}

#[command]
pub async fn dbus_kek(app: AppHandle) {
    app.state::<AppState>()
        .dbus
        .run_command(DbusCommand::Kek)
        .await;
}

pub async fn handle_commands(command_rx: &mut Receiver<DbusCommand>) {
    while let Some(message) = command_rx.recv().await {
        match message {
            DbusCommand::Kek => {
                println!("KEK");
            }
        }
    }
}
