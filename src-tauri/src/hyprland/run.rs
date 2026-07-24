use hyprland::dispatch::{Dispatch, DispatchType, WorkspaceIdentifierWithSpecial};
use tokio::{
    join,
    sync::mpsc::{Receiver, channel},
};

use crate::hyprland::{commands::HyprlandCommand, events::HyprlandEvent};

use super::listener::start_listener;

async fn handle_commands(command_rx: &mut Receiver<HyprlandCommand>) {
    while let Some(command) = command_rx.recv().await {
        match command {
            HyprlandCommand::SetWorkspace(id) => {
                Dispatch::call(DispatchType::Workspace(WorkspaceIdentifierWithSpecial::Id(
                    id,
                )))
                .unwrap();
            }
        }
    }
}

pub fn run_hyprland(
    command_rx: &mut Receiver<HyprlandCommand>,
) -> (impl Future<Output = ()>, Receiver<HyprlandEvent>) {
    let (event_tx, event_rx) = channel::<HyprlandEvent>(32);

    let listener = async {
        join!(start_listener(event_tx), handle_commands(command_rx));
    };

    (listener, event_rx)
}
