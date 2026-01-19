use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwNode {
    pub id: u32,
    pub name: String,
    pub nick: String,
    pub class: String,
    pub description: String,
    pub alsa_name: String,
    pub card_name: String,
    pub mixer_name: String,
    pub icon_name: String,
    pub device_id: u32,
    pub client_id: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwNodeProps {
    pub id: u32,
    pub volume: (f32, f32),
    pub muted: bool,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwDevice {
    pub id: u32,
    pub name: String,
    pub nick: String,
    pub description: String,
    pub alsa_name: String,
    pub card_name: String,
    pub mixer_name: String,
    pub icon_name: String,
    pub client_id: u32,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwMediaClass {
    pub name: String,
    pub devices: Vec<i32>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwDeviceProfile {
    pub device_id: u32,
    pub index: i32,
    pub name: String,
    pub description: String,
    pub priority: i32,
    pub available: bool,
    pub classes: Vec<PwMediaClass>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PwDeviceRouteDirection {
    Input,
    Output,
    Unknown,
}

impl Default for PwDeviceRouteDirection {
    fn default() -> Self {
        PwDeviceRouteDirection::Unknown
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PwDeviceRoute {
    pub device_id: u32,
    pub index: i32,
    pub direction: PwDeviceRouteDirection,
    pub name: String,
    pub description: String,
    pub priority: i32,
    pub available: bool,
    pub volume: (f32, f32),
    pub mute: bool,
    pub profiles: Vec<i32>,
    pub devices: Vec<i32>,
}

#[derive(Debug)]
pub enum PwEvent {
    Node(PwNode),
    NodeRemoved(u32),
    NodeProps(PwNodeProps),
    Device(PwDevice),
    DeviceEnumProfile(PwDeviceProfile),
    DeviceEnumRoute(PwDeviceRoute),
    DeviceProfile(PwDeviceProfile),
    DeviceRoute(PwDeviceRoute),
    DefaultSink(String),
    DefaultSource(String),
}

pub fn handle_event(event: PwEvent, app: &AppHandle) {
    match event {
        PwEvent::Node(node) => {
            app.emit("pw_node", serde_json::to_string(&node).unwrap())
                .unwrap();
        }
        PwEvent::NodeProps(node_props) => {
            app.emit("pw_node_props", serde_json::to_string(&node_props).unwrap())
                .unwrap();
        }
        PwEvent::NodeRemoved(id) => {
            app.emit("pw_node_removed", id.to_string().as_str())
                .unwrap();
        }
        PwEvent::DefaultSink(sink) => {
            app.emit("pw_default_sink", sink.as_str()).unwrap();
        }
        PwEvent::DefaultSource(source) => {
            app.emit("pw_default_source", source.as_str()).unwrap();
        }
        PwEvent::Device(device) => {
            app.emit("pw_device", serde_json::to_string(&device).unwrap())
                .unwrap();
        }
        PwEvent::DeviceEnumProfile(enum_profile) => {
            app.emit(
                "pw_device_enum_profile",
                serde_json::to_string(&enum_profile).unwrap(),
            )
            .unwrap();
        }
        PwEvent::DeviceEnumRoute(enum_route) => {
            app.emit(
                "pw_device_enum_route",
                serde_json::to_string(&enum_route).unwrap(),
            )
            .unwrap();
        }
        PwEvent::DeviceProfile(profile) => {
            app.emit(
                "pw_device_profile",
                serde_json::to_string(&profile).unwrap(),
            )
            .unwrap();
        }
        PwEvent::DeviceRoute(route) => {
            app.emit("pw_device_route", serde_json::to_string(&route).unwrap())
                .unwrap();
        }
    }
}
