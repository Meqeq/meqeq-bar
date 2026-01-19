use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::sync::mpsc::Receiver;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Battery {
    id: usize,
    percentage: f64,
    state: String,
}

pub enum BatteryEvent {
    BatteryInfo(Battery),
    BatteryNewState(battery::State),
}

pub async fn handle_events(app: &AppHandle, event_rx: &mut Receiver<BatteryEvent>) {
    while let Some(message) = event_rx.recv().await {
        match message {
            BatteryEvent::BatteryNewState(state) => {
                app.emit("battery_new_state", state.to_string()).unwrap();
            }
            BatteryEvent::BatteryInfo(info) => {
                app.emit("battery_info", serde_json::to_string(&info).unwrap())
                    .unwrap();
            }
        }
    }
}
