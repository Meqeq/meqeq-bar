use serde::{Deserialize, Serialize};

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
