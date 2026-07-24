use tokio::join;
use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::dbus::events::DbusEvent;
use crate::dbus::host::run_host;
use crate::dbus::utils::TrayItemHandles;
use crate::dbus::watcher::run_watcher;

use super::commands::DbusCommand;

async fn handle_commands(command_rx: &mut Receiver<DbusCommand>, handles: TrayItemHandles) {
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

pub fn run_dbus(
    command_rx: &mut Receiver<DbusCommand>,
) -> (impl Future<Output = ()>, Receiver<DbusEvent>) {
    let (event_tx, event_rx): (Sender<DbusEvent>, Receiver<DbusEvent>) = channel(32);

    let tray_item_handles = TrayItemHandles::new();

    let listener = async {
        let _ = join!(
            run_host(event_tx, tray_item_handles.clone()),
            run_watcher(),
            handle_commands(command_rx, tray_item_handles)
        );
    };

    (listener, event_rx)
}
