use std::{
    thread::{self},
    time::Duration,
};

use pipewire::channel::{channel, Receiver, Sender};
use tauri::{async_runtime, AppHandle, Manager};
use tokio::time::sleep;

use crate::app_state::AppState;

use super::{commands::PwCommand, mainloop::pipewire_main_loop};

pub struct PipewireState {
    command_tx: Sender<PwCommand>,
}

impl PipewireState {
    pub fn run_command(&self, command: PwCommand) {
        self.command_tx.send(command).unwrap();
    }
}

pub fn init_pipewire(app: &AppHandle) -> PipewireState {
    let (command_tx, command_rx): (Sender<PwCommand>, Receiver<PwCommand>) = channel();

    let handle = app.clone();
    async_runtime::spawn(async move {
        sleep(Duration::from_millis(100)).await;

        let state = handle.state::<AppState>();

        state.wait_for_initialization().await;

        thread::spawn(move || pipewire_main_loop(command_rx, handle));
    });

    PipewireState { command_tx }
}
