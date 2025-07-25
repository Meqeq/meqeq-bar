use std::sync::Mutex;

use tauri::{command, AppHandle, Manager};

use crate::app_state::AppState;

#[derive(Debug)]
pub enum PwCommand {
    // Terminate,
    SetDefaultSink(String),
    SetDefaultSource(String),
    SetVolume(u32, Vec<f32>),
    // SetMute(bool),
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
pub async fn set_node_props(id: u32, channel_volumes: Vec<f32>, mute: bool, app: AppHandle) {
    println!("CHANGE_PROPS {:?} {:?} {:?}", id, channel_volumes, mute);

    {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        state.send_pw_message(PwCommand::SetVolume(id, channel_volumes));
    }
}
