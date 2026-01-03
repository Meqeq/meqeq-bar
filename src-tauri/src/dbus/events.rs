use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc::Receiver;

#[derive(Serialize, Deserialize, Debug)]
pub struct TrayItem {
    pub id: String,
    pub title: String,
    pub status: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrayItemNewIcon {
    pub id: String,
    pub icon: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrayItemNewProp {
    pub id: String,
    pub prop_name: String,
    pub prop: String,
}

pub enum DbusEvent {
    RegisterTrayItem(TrayItem),
    UnregisterTrayItem(String),

    TrayItemNewIcon(TrayItemNewIcon),
    TrayItemNewProp(TrayItemNewProp),
}

pub async fn handle_events(app: &AppHandle, event_rx: &mut Receiver<DbusEvent>) {
    while let Some(message) = event_rx.recv().await {
        match message {
            DbusEvent::RegisterTrayItem(item) => {
                app.emit(
                    "dbus_register_tray_item",
                    serde_json::to_string(&item).unwrap(),
                )
                .unwrap();
            }
            DbusEvent::UnregisterTrayItem(item) => {
                app.emit("dbus_unregister_tray_item", item).unwrap();
            }
            DbusEvent::TrayItemNewIcon(icon) => {
                app.emit(
                    "dbus_tray_item_new_icon",
                    serde_json::to_string(&icon).unwrap(),
                )
                .unwrap();
            }
            DbusEvent::TrayItemNewProp(prop) => {
                app.emit(
                    "dbus_tray_item_new_prop",
                    serde_json::to_string(&prop).unwrap(),
                )
                .unwrap();
            }
        }
    }
}
