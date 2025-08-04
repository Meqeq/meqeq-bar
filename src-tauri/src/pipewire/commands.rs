use std::sync::Mutex;

use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Manager};

use crate::app_state::AppState;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwSetDefault {
    id: u32,
}

#[derive(Debug)]
pub enum PwCommand {
    // Terminate,
    SetDefaultSink(String),
    SetDefaultSource(String),
    SetNodeVolume(u32, Vec<f32>),
    SetNodeMute(u32, bool),
}

#[command]
pub async fn set_default_source(source: String, app: AppHandle) {
    {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        state.send_pw_message(PwCommand::SetDefaultSource(source));
    }
}

#[command]
pub async fn set_default_sink(sink: String, app: AppHandle) {
    {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        state.send_pw_message(PwCommand::SetDefaultSink(sink));
    }
}

#[command]
pub async fn set_node_volume(id: u32, channel_volumes: Vec<f32>, app: AppHandle) {
    println!("CHANGE_PROPS {:?}  {:?}", id, channel_volumes);

    {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        state.send_pw_message(PwCommand::SetNodeVolume(id, channel_volumes));
    }
}

#[command]
pub async fn set_node_mute(id: u32, mute: bool, app: AppHandle) {
    println!("CHANGE_MUTE {:?}  {:?}", id, mute);

    {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        state.send_pw_message(PwCommand::SetNodeMute(id, mute));
    }
}
