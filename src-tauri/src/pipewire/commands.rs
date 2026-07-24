use strum_macros::AsRefStr;
use tauri::{AppHandle, Manager, command};

use crate::state::{commands::Command, state::AppState};

#[derive(Debug, AsRefStr)]
pub enum PipewireCommand {
    SetDefaultSink(String),
    SetDefaultSource(String),
    SetNodeVolume(u32, Vec<f32>),
    MuteNode(u32, bool),
    SetDeviceVolume(u32, u32, u32, Vec<f32>),
    SetDeviceMute(u32, u32, u32, bool),
    SetDeviceRoute(u32, u32, u32),
    SetDeviceProfile(u32, u32),
}

#[command]
pub async fn set_default_source(source: String, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Pipewire(PipewireCommand::SetDefaultSource(source)))
        .await;
}

#[command]
pub async fn set_default_sink(sink: String, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Pipewire(PipewireCommand::SetDefaultSink(sink)))
        .await;
}

#[command]
pub async fn set_node_volume(id: u32, channel_volumes: Vec<f32>, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Pipewire(PipewireCommand::SetNodeVolume(
            id,
            channel_volumes,
        )))
        .await;
}

#[command]
pub async fn set_node_mute(id: u32, mute: bool, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Pipewire(PipewireCommand::MuteNode(id, mute)))
        .await;
}

#[command]
pub async fn set_device_volume(
    id: u32,
    route_index: u32,
    route_device: u32,
    channel_volumes: Vec<f32>,
    app: AppHandle,
) {
    app.state::<AppState>()
        .send_command(Command::Pipewire(PipewireCommand::SetDeviceVolume(
            id,
            route_index,
            route_device,
            channel_volumes,
        )))
        .await;
}

#[command]
pub async fn set_device_mute(
    id: u32,
    route_index: u32,
    route_device: u32,
    mute: bool,
    app: AppHandle,
) {
    app.state::<AppState>()
        .send_command(Command::Pipewire(PipewireCommand::SetDeviceMute(
            id,
            route_index,
            route_device,
            mute,
        )))
        .await;
}

#[command]
pub async fn set_device_route(id: u32, route_index: u32, route_device: u32, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Pipewire(PipewireCommand::SetDeviceRoute(
            id,
            route_index,
            route_device,
        )))
        .await;
}

#[command]
pub async fn set_device_profile(id: u32, profile_index: u32, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Pipewire(PipewireCommand::SetDeviceProfile(
            id,
            profile_index,
        )))
        .await;
}
