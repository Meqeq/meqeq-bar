use tauri::{AppHandle, Manager, command};
use tokio::sync::mpsc::Receiver;

use crate::{app_state::AppState, dbus::utils::TrayItemHandles};

pub enum DbusCommand {
    CallMenuEntry(String, i32),
}

#[command]
pub async fn dbus_tray_item_call_menu(item_id: String, entry_id: i32, app: AppHandle) {
    app.state::<AppState>()
        .dbus
        .run_command(DbusCommand::CallMenuEntry(item_id, entry_id))
        .await;
}

pub async fn handle_commands(command_rx: &mut Receiver<DbusCommand>, handles: TrayItemHandles) {
    while let Some(message) = command_rx.recv().await {
        match message {
            DbusCommand::CallMenuEntry(item_id, entry_id) => {
                let tx = handles.with_read_by_id(&item_id, |handle| {
                    handle.map(|handle| handle.menu_call_tx.clone())
                });

                if let Some(tx) = tx {
                    tx.send(entry_id).await.unwrap();
                }
            }
        }
    }
}
