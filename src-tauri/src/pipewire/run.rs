use pipewire::channel::{Sender, channel};

use tokio::{join, sync::mpsc, task::spawn_blocking};

use crate::pipewire::events::PipewireEvent;

use super::{commands::PipewireCommand, internals::mainloop::pipewire_main_loop};

async fn pass_commands(rx: &mut mpsc::Receiver<PipewireCommand>, tx: Sender<PipewireCommand>) {
    while let Some(command) = rx.recv().await {
        tx.send(command).expect("Error passing pipewire command");
    }
}

pub fn run_pipewire(
    outside_command_rx: &mut mpsc::Receiver<PipewireCommand>,
) -> (impl Future<Output = ()>, mpsc::Receiver<PipewireEvent>) {
    let (event_tx, event_rx) = mpsc::channel::<PipewireEvent>(32);

    let (command_tx, command_rx) = channel::<PipewireCommand>();

    let listener = async move {
        let _ = join!(
            pass_commands(outside_command_rx, command_tx),
            spawn_blocking(move || {
                pipewire_main_loop(command_rx, event_tx);
            })
        );
    };

    (listener, event_rx)
}
