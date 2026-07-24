use gtk_layer_shell::{KeyboardMode, Layer, LayerShell};

use tokio::sync::{Barrier, mpsc::Sender};

use crate::{state::commands::Command, utils::gtk::Bar};

pub struct AppState {
    bars: Vec<Bar>,
    init_barrier: Barrier,
    command_tx: Sender<Command>,
}

impl AppState {
    pub fn new(bars: Vec<Bar>, command_tx: Sender<Command>) -> AppState {
        let init_barrier = Barrier::new(bars.len() + 1);

        AppState {
            bars,
            init_barrier,
            command_tx,
        }
    }

    pub async fn mark_initialized_bar(&self) {
        self.init_barrier.wait().await;
    }

    pub async fn wait_for_initialize(&self) {
        self.init_barrier.wait().await;
    }

    pub async fn send_command(&self, command: Command) {
        match self.command_tx.send(command).await {
            Ok(_) => {}
            Err(e) => {
                println!("Error while sending command {:?}", e)
            }
        }
    }

    pub fn set_layer_for_bar(&self, bar: usize, layer: String) {
        let bar = self.bars.get(bar).expect("Wrong bar index");

        let layer = if layer == "top" {
            Layer::Top
        } else {
            Layer::Bottom
        };

        let keyboard_mode = match layer {
            Layer::Top => KeyboardMode::OnDemand,
            Layer::Bottom => KeyboardMode::None,
            _ => KeyboardMode::None,
        };

        bar.gtk_window.set_layer(layer);
        bar.gtk_window.set_keyboard_mode(keyboard_mode);
    }
}

// unsafe impl Send for AppState {}
// unsafe impl Sync for AppState {}
