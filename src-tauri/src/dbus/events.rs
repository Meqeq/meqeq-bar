use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;

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
#[serde(rename_all = "camelCase")]
pub struct TrayItemNewProp {
    pub id: String,
    pub prop_name: String,
    pub prop: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MenuEntry {
    pub id: i32,
    pub label: String,
    pub visible: bool,
    pub type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrayItemNewMenu {
    pub id: String,
    pub menu: Vec<MenuEntry>,
}

#[derive(Debug, AsRefStr, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum DbusEvent {
    RegisterTrayItem(TrayItem),
    UnregisterTrayItem(String),

    TrayItemNewIcon(TrayItemNewIcon),
    TrayItemNewProp(TrayItemNewProp),
    TrayItemNewMenu(TrayItemNewMenu),
}
