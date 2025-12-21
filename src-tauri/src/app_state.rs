use tauri::async_runtime;
use tokio::sync::{broadcast, mpsc};

use crate::{hyprland::init::HyprlandState, pipewire::run::PipewireState, utils::gtk::Bar};

pub struct AppState {
    pub bars: Vec<Bar>,
    pub hyprland: HyprlandState,
    pub pipewire: PipewireState,
    init_count_tx: mpsc::Sender<bool>,
    init_tx: broadcast::Sender<bool>,
}

impl AppState {
    pub fn new(bars: Vec<Bar>, hyprland: HyprlandState, pipewire: PipewireState) -> AppState {
        let to_initialize = bars.len();

        let (init_tx, _) = broadcast::channel(32);
        let (init_count_tx, mut init_count_rx) = mpsc::channel(to_initialize);

        let init_tx_clone = init_tx.clone();
        async_runtime::spawn(async move {
            let mut left_to_initialize = to_initialize;
            while let Some(_) = init_count_rx.recv().await {
                left_to_initialize -= 1;

                if left_to_initialize == 0 {
                    break;
                }
            }

            init_tx_clone.send(true).unwrap();
        });

        AppState {
            bars,
            hyprland,
            pipewire,
            init_count_tx,
            init_tx,
        }
    }

    pub async fn initialize(&self) {
        self.init_count_tx.send(true).await.unwrap();
    }

    pub async fn wait_for_initialization(&self) {
        let mut init_rx = self.init_tx.subscribe();

        init_rx.recv().await.unwrap();
    }
}

unsafe impl Send for AppState {}
unsafe impl Sync for AppState {}

// use std::{process, sync::Mutex};

// use gtk::ApplicationWindow;
// use pipewire::channel::Sender;
// use serde::{Deserialize, Serialize};
// use tauri::App;
// use zbus::Connection;

// use crate::{pipewire::commands::PwCommand, utils::hyprland::WorkspaceInfo};

// #[derive(Serialize, Deserialize)]
// struct WorkspacesInfo {
//     workspaces: Vec<WorkspaceInfo>,
//     active: i32,
// }

// pub struct AppState {
//     // pub bars: Vec<ApplicationWindow>,
//     pub workspaces: Vec<WorkspaceInfo>,
//     pub connection: Connection,

//     pw_sender: Option<Sender<PwCommand>>,

//     initialized: bool,
// }

// unsafe impl Send for AppState {}
// unsafe impl Sync for AppState {}

// impl AppState {
//     pub async fn new(app: &App) -> Mutex<Self> {
//         let connection = Connection::session().await.unwrap();

//         connection
//             .request_name(format!("org.kde.StatusNotifierHost-{}", process::id()))
//             .await
//             .unwrap();

//         // let monitor_info = get_monitor_info();

//         // let bars: Vec<ApplicationWindow> = monitor_info
//         //     .into_iter()
//         //     .map(|monitor| make_bar(app, &monitor))
//         //     .collect();

//         Mutex::new(Self {
//             // bars,
//             workspaces: Vec::new(),
//             connection,
//             initialized: false,
//             pw_sender: None,
//         })
//     }

//     pub fn add_workspace(&mut self, workspace: WorkspaceInfo) {
//         self.workspaces.push(workspace);
//     }

//     pub fn remove_workspace(&mut self, id: i32) {
//         self.workspaces.retain(|workspace| workspace.id != id);
//     }

//     pub fn is_initialized(&self) -> bool {
//         self.initialized
//     }

//     pub fn initialize(&mut self, mut workspaces: Vec<WorkspaceInfo>) {
//         self.initialized = true;
//         self.workspaces.append(&mut workspaces);
//     }

//     pub fn set_pw_sender(&mut self, sender: Sender<PwCommand>) {
//         self.pw_sender = Some(sender);
//     }

//     pub fn send_pw_message(&self, message: PwCommand) {
//         self.pw_sender.as_ref().unwrap().send(message).unwrap();
//     }
// }
