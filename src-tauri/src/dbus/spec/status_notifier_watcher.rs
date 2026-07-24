use std::collections::HashMap;
use tokio::sync::mpsc::Sender;

use zbus::{interface, message::Header, object_server::SignalEmitter};

use crate::dbus::utils::ReadHandle;

pub enum WatcherEvent {
    RegisterItem(String, String),
    RegisterHost(String, String),
    Unregister(String),
}

pub struct StatusNotifierWatcher {
    items: ReadHandle<HashMap<String, String>>,
    host: ReadHandle<HashMap<String, String>>,
    event_tx: Sender<WatcherEvent>,
}

impl StatusNotifierWatcher {
    pub fn new(
        items: ReadHandle<HashMap<String, String>>,
        host: ReadHandle<HashMap<String, String>>,
        event_tx: Sender<WatcherEvent>,
    ) -> StatusNotifierWatcher {
        StatusNotifierWatcher {
            items,
            host,
            event_tx,
        }
    }
}

#[interface(
    name = "org.kde.StatusNotifierWatcher",
    proxy(
        default_path = "/StatusNotifierWatcher",
        default_service = "org.kde.StatusNotifierWatcher",
    )
)]
impl StatusNotifierWatcher {
    async fn register_status_notifier_item(
        &self,
        #[zbus(header)] header: Header<'_>,
        path: String,
    ) {
        if let Some(service) = header.sender() {
            self.event_tx
                .send(WatcherEvent::RegisterItem(service.to_string(), path))
                .await
                .unwrap();
        }
    }

    async fn register_status_notifier_host(
        &self,
        #[zbus(header)] header: Header<'_>,
        path: String,
    ) {
        if let Some(service) = header.sender() {
            self.event_tx
                .send(WatcherEvent::RegisterHost(service.to_string(), path))
                .await
                .unwrap();
        }
    }

    #[zbus(property)]
    async fn registered_status_notifier_items(&self) -> Vec<String> {
        self.items
            .with_read(|map| map.values().cloned().collect())
            .await
    }

    #[zbus(property)]
    async fn is_status_notifier_host_registered(&self) -> bool {
        self.host.with_read(|map| !map.is_empty()).await
    }

    #[zbus(signal)]
    async fn status_notifier_item_registered(
        signal_emitter: &SignalEmitter<'_>,
        service: String,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_item_unregistered(
        signal_emitter: &SignalEmitter<'_>,
        service: String,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_host_registered(
        signal_emitter: &SignalEmitter<'_>,
        service: String,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_host_unregistered(
        signal_emitter: &SignalEmitter<'_>,
        service: String,
    ) -> zbus::Result<()>;
}
