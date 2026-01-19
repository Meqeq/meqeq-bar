use tauri::{AppHandle, Manager, async_runtime};
use tokio::sync::mpsc::{Receiver, Sender, channel};

use tokio::time::{Duration, sleep};

use crate::app_state::AppState;
use crate::dbus::commands::handle_commands;
use crate::dbus::events::{DbusEvent, handle_events};
use crate::dbus::status_notifier_host::run_host;
use crate::dbus::status_notifier_watcher::run_watcher;
use crate::dbus::utils::TrayItemHandles;

use super::commands::DbusCommand;

pub struct DbusState {
    command_tx: Sender<DbusCommand>,
}

impl DbusState {
    pub async fn run_command(&self, command: DbusCommand) {
        self.command_tx.send(command).await.unwrap();
    }
}

pub fn init_dbus(handle: &AppHandle) -> DbusState {
    let (event_tx, mut event_rx): (Sender<DbusEvent>, Receiver<DbusEvent>) = channel(32);
    let (command_tx, mut command_rx): (Sender<DbusCommand>, Receiver<DbusCommand>) = channel(32);

    let handle = handle.clone();

    let tray_item_handles = TrayItemHandles::new();

    async_runtime::spawn(async move {
        sleep(Duration::from_millis(100)).await;

        let state = handle.state::<AppState>();
        state.wait_for_initialization().await;

        let _ = tokio::join!(
            run_host(event_tx, tray_item_handles.clone()),
            run_watcher(),
            handle_events(&handle, &mut event_rx),
            handle_commands(&mut command_rx, tray_item_handles)
        );
    });

    DbusState { command_tx }
}
