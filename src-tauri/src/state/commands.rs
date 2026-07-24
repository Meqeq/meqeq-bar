use tokio::sync::{mpsc::Receiver, mpsc::channel};

use crate::{
    dbus::commands::DbusCommand, hyprland::commands::HyprlandCommand,
    pipewire::commands::PipewireCommand,
};

pub enum Command {
    Dbus(DbusCommand),
    Hyprland(HyprlandCommand),
    Pipewire(PipewireCommand),
}

impl Command {
    pub fn path(&self) -> String {
        match self {
            Command::Hyprland(x) => format!("Hyprland/{}", x.as_ref()),
            Command::Pipewire(x) => format!("Pipewire/{}", x.as_ref()),
            Command::Dbus(x) => format!("Dbus/{}", x.as_ref()),
        }
    }
}

pub fn pass_commands(
    receiver: &mut Receiver<Command>,
) -> (
    impl Future<Output = ()>,
    Receiver<DbusCommand>,
    Receiver<HyprlandCommand>,
    Receiver<PipewireCommand>,
) {
    let (dbus_tx, dbus_rx) = channel::<DbusCommand>(32);
    let (hyprland_tx, hyprland_rx) = channel::<HyprlandCommand>(32);
    let (pipewire_tx, pipewire_rx) = channel::<PipewireCommand>(32);

    let listener = async move {
        while let Some(command) = receiver.recv().await {
            let path = command.path();
            // println!("COMMAND: {:?}", path);

            match command {
                Command::Hyprland(command) => hyprland_tx
                    .send(command)
                    .await
                    .unwrap_or_else(|e| println!("Error passing command({:?}): {:?}", path, e)),
                Command::Dbus(command) => dbus_tx
                    .send(command)
                    .await
                    .unwrap_or_else(|e| println!("Error passing command({:?}): {:?}", path, e)),
                Command::Pipewire(command) => pipewire_tx
                    .send(command)
                    .await
                    .unwrap_or_else(|e| println!("Error passing command({:?}): {:?}", path, e)),
            };
        }
    };

    (listener, dbus_rx, hyprland_rx, pipewire_rx)
}
