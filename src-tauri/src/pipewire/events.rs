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

#[derive(Debug)]
pub enum PwEvent {
    Node(PwNode),
    NodeRemoved(u32),
    NodeProps(PwNodeProps),
    DefaultSink(String),
    DefaultSource(String),
}
