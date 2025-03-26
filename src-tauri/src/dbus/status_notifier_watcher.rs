use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use trpl::StreamExt;
use zbus::{
    interface,
    fdo::DBusProxy,
    message::Header,
    object_server::SignalEmitter, Connection,
};

pub struct StatusNotifierWatcher {
    tray_items: Arc<Mutex<HashMap<String, String>>>,
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
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
        #[zbus(header)] header: Header<'_>,
        path: String,
    ) {
        let service = header.sender().unwrap().to_string();

        let item = format!("{}{}", &service, &path).to_string();

        emitter
            .status_notifier_item_registered(item.clone())
            .await
            .unwrap();

        let mut items = self.tray_items.lock().unwrap();
        items.insert(service.clone(), item);
    }

    #[zbus(property)]
    fn registered_status_notifier_items(&self) -> Vec<String> {
        let items = self.tray_items.lock().unwrap();

        items.iter().map(|(_, value)| value.clone()).collect()
    }

    #[zbus(signal)]
    async fn status_notifier_item_registered(
        signal_emitter: &SignalEmitter<'_>,
        message: String,
    ) -> zbus::Result<()>;

    #[zbus(signal)]
    async fn status_notifier_item_unregistered(
        signal_emitter: &SignalEmitter<'_>,
        message: String,
    ) -> zbus::Result<()>;

    async fn init(
        &self,
        #[zbus(connection)] connection: &Connection,
        #[zbus(signal_emitter)] emitter: SignalEmitter<'_>,
    ) {
        let proxy = DBusProxy::new(connection).await.unwrap();

        let mut stream = proxy.receive_name_owner_changed().await.unwrap();

        while let Some(msg) = stream.next().await {
            let args = msg.args().unwrap();
            let service = args.name().to_string();

            let mut item = String::new();

            {
                let mut items = self.tray_items.lock().unwrap();
                if items.contains_key(&service) && args.new_owner().is_none() {
                    println!("RRR: {:?}", args);
                    let removed = items.remove(&service).unwrap();

                    item.push_str(&removed);
                }
            }

            if !item.is_empty() {
                emitter
                    .status_notifier_item_unregistered(item)
                    .await
                    .unwrap();
            }
        }
    }
}

impl StatusNotifierWatcher {
    fn new() -> Self {
        Self {
            tray_items: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn serve() {
        let interface = StatusNotifierWatcher::new();

        let connection = zbus::connection::Builder::session()
            .unwrap()
            .name("org.kde.StatusNotifierWatcher")
            .unwrap()
            .serve_at("/StatusNotifierWatcher", interface)
            .unwrap()
            .build()
            .await
            .unwrap();

        let proxy = StatusNotifierWatcherProxy::new(&connection).await.unwrap();

        proxy.init().await.unwrap();
    }
}

