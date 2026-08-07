use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use tauri::{AppHandle, Emitter};
use tokio::{select, sync::mpsc::Receiver};

use crate::{
    dbus::{events::DbusEvent, mpris::events::PlayerEvent},
    hyprland::events::HyprlandEvent,
    pipewire::events::PipewireEvent,
};

#[derive(Debug, AsRefStr, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum Event {
    Dbus(DbusEvent),
    Player(PlayerEvent),
    Hyprland(HyprlandEvent),
    Pipewire(PipewireEvent),
}

impl Event {
    pub fn path(&self) -> String {
        match self {
            Event::Dbus(x) => format!("Dbus/{}", x.as_ref()),
            Event::Player(x) => format!("Player/{}", x.as_ref()),
            Event::Hyprland(x) => format!("Hyprland/{}", x.as_ref()),
            Event::Pipewire(x) => format!("Pipewire/{}", x.as_ref()),
        }
    }
}

async fn emit_event(handle: &AppHandle, event: Event) {
    let path = event.path();

    match serde_json::to_string(&event) {
        Ok(serialized) => {
            println!("EVENT: {:?} {:?}", path, serialized);
            let res = handle.emit(&path, serialized);

            if res.is_err() {
                println!("Error while emitting event({:?}): {:?}", path, res.err());
            }
        }
        Err(e) => {
            println!("Error while serializing event({:?}): {:?}", path, e);
        }
    }
}

pub async fn receive_events(
    handle: &AppHandle,
    dbus_receiver: &mut Receiver<DbusEvent>,
    player_receiver: &mut Receiver<PlayerEvent>,
    hyprland_receiver: &mut Receiver<HyprlandEvent>,
    pipewire_receiver: &mut Receiver<PipewireEvent>,
) {
    loop {
        select! {
            Some(event) = dbus_receiver.recv() => {
                emit_event(handle, Event::Dbus(event)).await;
            },
            Some(event) = player_receiver.recv() => {
                emit_event(handle, Event::Player(event)).await;
            },
            Some(event) = hyprland_receiver.recv() => {
                emit_event(handle, Event::Hyprland(event)).await;
            },
            Some(event) = pipewire_receiver.recv() => {
                emit_event(handle, Event::Pipewire(event)).await;
            }

        }
    }
}
