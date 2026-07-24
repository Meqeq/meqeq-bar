use tauri::async_runtime;
use tokio::sync::{broadcast, mpsc};

use crate::utils::gtk::Bar;

pub struct AppState {
    pub bars: Vec<Bar>,
    // pub hyprland: HyprlandState,
    // pub pipewire: PipewireState,
    // pub dbus: DbusState,
    init_count_tx: mpsc::Sender<bool>,
    init_tx: broadcast::Sender<bool>,
}

impl AppState {
    pub fn new(
        bars: Vec<Bar>,
        // hyprland: HyprlandState,
        // pipewire: PipewireState,
        // dbus: DbusState,
    ) -> AppState {
        let to_initialize = bars.len();

        let (init_tx, _) = broadcast::channel(32);
        let (init_count_tx, mut init_count_rx) = mpsc::channel(to_initialize);

        let init_tx_clone = init_tx.clone();
        async_runtime::spawn(async move {
            let mut left_to_initialize = to_initialize;
            while init_count_rx.recv().await.is_some() {
                left_to_initialize -= 1;

                if left_to_initialize == 0 {
                    break;
                }
            }

            init_tx_clone.send(true).unwrap();
        });

        AppState {
            bars,
            // hyprland,
            // pipewire,
            // dbus,
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
