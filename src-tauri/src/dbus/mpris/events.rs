use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use strum_macros::AsRefStr;
use zbus::proxy::PropertyChanged;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MediaPlayer {
    pub name: String,
    pub identity: String,
}

#[derive(Debug, AsRefStr, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlaybackStatus {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, AsRefStr, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlayerAbility {
    GoNext,
    GoPrevious,
    Play,
    Pause,
    Seek,
    Control,
}

impl PlaybackStatus {
    pub async fn from_property(property: PropertyChanged<'_, String>) -> PlaybackStatus {
        match property.get().await {
            Ok(status) => {
                if status == "Playing" {
                    PlaybackStatus::Playing
                } else if status == "Paused" {
                    PlaybackStatus::Paused
                } else {
                    PlaybackStatus::Stopped
                }
            }
            Err(err) => {
                println!("Error reading playback status: {:?}", err);
                PlaybackStatus::Stopped
            }
        }
    }
}

#[derive(Debug, AsRefStr, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum LoopStatus {
    None,
    Track,
    Playlist,
}

impl LoopStatus {
    pub async fn from_property(property: PropertyChanged<'_, String>) -> LoopStatus {
        match property.get().await {
            Ok(status) => {
                if status == "Track" {
                    LoopStatus::Track
                } else if status == "Playlist" {
                    LoopStatus::Playlist
                } else {
                    LoopStatus::None
                }
            }
            Err(err) => {
                println!("Error reading loop status: {:?}", err);
                LoopStatus::None
            }
        }
    }
}

#[derive(Debug, AsRefStr, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", untagged)]
pub enum PlayerEvent {
    New(MediaPlayer),
    Disconnect(String),
    PlaybackStatus(String, PlaybackStatus),
    Metadata(String, HashMap<String, zbus::zvariant::OwnedValue>),
    LoopStatus(String, LoopStatus),
    Ability(String, PlayerAbility, bool),
    Shuffle(String, bool),
    CanGoNext(String, bool),
    CanGoPrevious(String, bool),
    Position(String, i64),
    CanSeek(String, bool),
    CanControl(String, bool),
}
