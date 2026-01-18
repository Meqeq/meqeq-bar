use std::{any::Any, time::Duration};

use battery::Battery;
use tokio::sync::mpsc::Sender;

use crate::battery::events::BatteryEvent;

pub async fn run_watcher(event_tx: Sender<BatteryEvent>) {
    let manager = battery::Manager::new().unwrap();

    for (idx, maybe_battery) in manager.batteries().unwrap().enumerate() {
        let battery = maybe_battery.unwrap();
        println!("Battery #{}:", idx);
        println!("Vendor: {:?}", battery.vendor());
        println!("Model: {:?}", battery.model());
        println!("State: {:?}", battery.state());
        println!("Time to full charge: {:?}", battery.time_to_full());
        println!("Time to full charge: {:?}", battery.energy_rate());
        println!("Time to full charge: {:?}", battery.state_of_charge());
        println!("");
    }

    async move {
        let batteries: Vec<Battery> = manager
            .batteries()
            .unwrap()
            .filter_map(|maybe_battery| maybe_battery.ok())
            .collect();

        let prev_state = String::new();

        loop {
            tokio::time::sleep(Duration::from_millis(2000)).await;

            batteries.iter().enumerate().for_each(|(id, battery)| {
                println!("DDDBBBB {:?}", battery);
            });
        }
    }
    .await;
}
