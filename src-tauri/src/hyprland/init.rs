use std::time::Duration;

use tokio::{
    join,
    sync::mpsc::{channel, Receiver, Sender},
    time::sleep,
};

use tauri::{async_runtime, AppHandle, Manager};

use crate::{
    app_state::AppState,
    hyprland::{
        commands::{handle_commands, HyprlandCommand},
        events::HyprlandEvent,
    },
};

use super::{events::handle_events, listener::start_listener};

pub struct HyprlandState {
    command_tx: Sender<HyprlandCommand>,
}

impl HyprlandState {
    pub async fn run_command(&self, command: HyprlandCommand) {
        self.command_tx.send(command).await.unwrap();
    }
}

pub fn init_hyprland(handle: &AppHandle) -> HyprlandState {
    let (event_tx, mut event_rx): (Sender<HyprlandEvent>, Receiver<HyprlandEvent>) = channel(32);
    let (command_tx, mut command_rx): (Sender<HyprlandCommand>, Receiver<HyprlandCommand>) =
        channel(32);

    let handle = handle.clone();
    async_runtime::spawn(async move {
        sleep(Duration::from_millis(100)).await;

        let state = handle.state::<AppState>();

        state.wait_for_initialization().await;

        join!(
            handle_commands(&mut command_rx),
            handle_events(&handle, &mut event_rx),
            start_listener(event_tx)
        );
    });

    async_runtime::spawn(async move {});

    HyprlandState { command_tx }
}
