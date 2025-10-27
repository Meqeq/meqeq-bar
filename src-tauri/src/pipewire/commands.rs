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
    SetDeviceVolume(u32, u32, u32, Vec<f32>),
    SetDeviceMute(u32, u32, u32, bool),
    SetDeviceRoute(u32, u32, u32),
    SetDeviceProfile(u32, u32),
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
        println!("DADADADA {:?}", sink);
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

#[command]
pub async fn set_device_volume(
    id: u32,
    route_index: u32,
    route_device: u32,
    channel_volumes: Vec<f32>,
    app: AppHandle,
) {
    println!("CHANGE_PROPS {:?}  {:?}", id, channel_volumes);

    {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        state.send_pw_message(PwCommand::SetDeviceVolume(
            id,
            route_index,
            route_device,
            channel_volumes,
        ));
    }
}

#[command]
pub async fn set_device_mute(
    id: u32,
    route_index: u32,
    route_device: u32,
    mute: bool,
    app: AppHandle,
) {
    println!("CHANGE_MUTE {:?}  {:?}", id, mute);

    {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        state.send_pw_message(PwCommand::SetDeviceMute(
            id,
            route_index,
            route_device,
            mute,
        ));
    }
}

#[command]
pub async fn set_device_route(id: u32, route_index: u32, route_device: u32, app: AppHandle) {
    {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        state.send_pw_message(PwCommand::SetDeviceRoute(id, route_index, route_device));
    }
}

#[command]
pub async fn set_device_profile(id: u32, profile_index: u32, app: AppHandle) {
    {
        let state = app.state::<Mutex<AppState>>();
        let state = state.lock().unwrap();

        state.send_pw_message(PwCommand::SetDeviceProfile(id, profile_index));
    }
}
