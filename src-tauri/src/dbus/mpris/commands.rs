use strum_macros::AsRefStr;
use tauri::{AppHandle, Manager, command};

use crate::state::{commands::Command, state::AppState};

#[derive(Debug, AsRefStr)]
pub enum PlayerCommand {
    Play(String),
    Pause(String),
    Next(String),
    Prev(String),
    Seek(String, i64),
    Shuffle(String, bool),
}

#[command]
pub async fn player_play(name: String, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Player(PlayerCommand::Play(name)))
        .await;
}

#[command]
pub async fn player_pause(name: String, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Player(PlayerCommand::Pause(name)))
        .await;
}

#[command]
pub async fn player_next(name: String, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Player(PlayerCommand::Next(name)))
        .await;
}

#[command]
pub async fn player_prev(name: String, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Player(PlayerCommand::Prev(name)))
        .await;
}

#[command]
pub async fn player_seek(name: String, position: i64, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Player(PlayerCommand::Seek(name, position)))
        .await;
}

#[command]
pub async fn player_shuffle(name: String, shuffle: bool, app: AppHandle) {
    app.state::<AppState>()
        .send_command(Command::Player(PlayerCommand::Shuffle(name, shuffle)))
        .await;
}
