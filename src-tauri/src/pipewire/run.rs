use std::thread::{self, JoinHandle};

use pipewire::channel::{channel, Receiver, Sender};
use tauri::AppHandle;

use super::{commands::PwCommand, mainloop::pipewire_main_loop};

pub fn run_pipewire(app: AppHandle) -> (Sender<PwCommand>, JoinHandle<()>) {
    let (command_tx, command_rx): (Sender<PwCommand>, Receiver<PwCommand>) = channel();

    (
        command_tx,
        thread::spawn(move || pipewire_main_loop(command_rx, app)),
    )
}
