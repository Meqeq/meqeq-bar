use std::collections::HashMap;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio_stream::StreamExt;
use zbus::{
    Connection, Result, conn::Builder, fdo::DBusProxy, interface, message::Header,
    object_server::SignalEmitter,
};

use crate::dbus::utils::rw_lock_handles;

use super::utils::{ReadHandle, WriteHandle};

enum WatcherEvent {
    RegisterItem(String, String),
    RegisterHost(String, String),
    Unregister(String),
}

struct StatusNotifierWatcher {
    // connection: Connection,
    items: ReadHandle<HashMap<String, String>>,
    host: ReadHandle<HashMap<String, String>>,
    event_tx: Sender<WatcherEvent>,
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
        self.host.with_read(|map| map.len() > 0).await
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

async fn listen_for_unregister(connection: &Connection, event_tx: Sender<WatcherEvent>) {
    let proxy = DBusProxy::new(connection).await.unwrap();

    let mut stream = proxy.receive_name_owner_changed().await.unwrap();

    while let Some(msg) = stream.next().await {
        let args = msg.args().unwrap();

        if args.new_owner().is_none() {
            event_tx
                .send(WatcherEvent::Unregister(args.name().to_string()))
                .await
                .unwrap();
        }
    }
}

async fn handle_events(
    connection: &Connection,
    event_rx: &mut Receiver<WatcherEvent>,
    items: WriteHandle<HashMap<String, String>>,
    host: WriteHandle<HashMap<String, String>>,
) {
    let interface = connection
        .object_server()
        .interface::<_, StatusNotifierWatcher>("/StatusNotifierWatcher")
        .await
        .unwrap();

    while let Some(message) = event_rx.recv().await {
        match message {
            WatcherEvent::RegisterItem(service, path) => {
                let entry = format!("{}{}", service, path);

                items
                    .with_write(|map| {
                        map.insert(service, entry.clone());
                    })
                    .await;

                interface
                    .status_notifier_item_registered(entry)
                    .await
                    .unwrap();
            }
            WatcherEvent::RegisterHost(service, path) => {
                let entry = format!("{}{}", service, path);
                host.with_write(|map| {
                    map.insert(service, entry.clone());
                })
                .await;

                interface
                    .status_notifier_host_registered(entry)
                    .await
                    .unwrap();
            }
            WatcherEvent::Unregister(service) => {
                if let Some(entry) = items.with_write(|map| map.remove(&service)).await {
                    interface
                        .status_notifier_item_unregistered(entry)
                        .await
                        .unwrap();
                } else if let Some(entry) = host.with_write(|map| map.remove(&service)).await {
                    interface
                        .status_notifier_host_unregistered(entry)
                        .await
                        .unwrap();
                }
            }
        };
    }
}

pub async fn run_watcher() -> Result<()> {
    let (event_tx, mut event_rx) = channel(32);
    let (items_read_handle, items_write_handle) = rw_lock_handles(HashMap::new());
    let (host_read_handle, host_write_handle) = rw_lock_handles(HashMap::new());

    let watcher = StatusNotifierWatcher {
        items: items_read_handle,
        host: host_read_handle,
        event_tx: event_tx.clone(),
    };

    let connection = Builder::session()?
        .name("org.kde.StatusNotifierWatcher")?
        .serve_at("/StatusNotifierWatcher", watcher)?
        .build()
        .await
        .unwrap();

    tokio::join!(
        listen_for_unregister(&connection, event_tx),
        handle_events(
            &connection,
            &mut event_rx,
            items_write_handle,
            host_write_handle
        )
    );

    Ok(())
}
