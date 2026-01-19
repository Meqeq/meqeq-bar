use std::time::Duration;

use tauri::{AppHandle, Manager, async_runtime};
use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::{
    app_state::AppState,
    battery::{
        events::{BatteryEvent, handle_events},
        watcher::run_watcher,
    },
};

pub fn init_battery(handle: &AppHandle) {
    let (event_tx, mut event_rx): (Sender<BatteryEvent>, Receiver<BatteryEvent>) = channel(32);

    let handle = handle.clone();

    async_runtime::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;

        let state = handle.state::<AppState>();
        state.wait_for_initialization().await;

        let _ = tokio::join!(handle_events(&handle, &mut event_rx), run_watcher(event_tx));
    });
}
