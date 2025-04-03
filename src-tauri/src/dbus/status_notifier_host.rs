use std::{collections::HashMap, process, sync::Arc};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
    task::JoinHandle,
};
use tokio_stream::StreamExt;
use zbus::{interface, Connection};

use crate::dbus::status_notifier_item::StatusNotifierItemProxy;

use super::status_notifier_watcher::StatusNotifierWatcherProxy;

struct TrayItem {
    // service: String,
    handle: JoinHandle<()>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrayItemEvent {
    service: String,
    title: String,
    icon: Vec<u8>,
}

async fn load_icon(icon_name: String, path: String) -> Vec<u8> {
    let file = File::open(format!("{}/{}.png", path, icon_name))
        .await
        .unwrap();

    let mut reader = BufReader::new(file);

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await.unwrap();

    buf
}

pub struct StatusNotifierHost {
    connection: Connection,
    app: AppHandle,
}

impl StatusNotifierHost {
    pub async fn connect(app: AppHandle) -> Arc<Self> {
        let connection = Connection::session().await.unwrap();

        connection
            .request_name(format!("org.kde.StatusNotifierHost-{}", process::id()))
            .await
            .unwrap();

        let kek = Arc::new(StatusNotifierHost {
            connection: connection.clone(),
            app,
        });

        return kek;
    }

    async fn handle_new_item(&self, item: String) {
        let (service, path) = item.split_once("/").unwrap();

        let item_proxy = StatusNotifierItemProxy::new(
            &self.connection,
            service.to_string(),
            format!("/{}", path).to_string(),
        )
        .await
        .unwrap();

        let to_emit = TrayItemEvent {
            service: item,
            title: item_proxy.title().await.unwrap(),
            icon: load_icon(
                item_proxy.icon_name().await.unwrap(),
                item_proxy.icon_theme_path().await.unwrap(),
            )
            .await,
        };

        self.app
            .emit("tray_item_add", serde_json::to_string(&to_emit).unwrap())
            .unwrap();

        let mut stream = item_proxy.receive_new_title().await.unwrap();

        while let Some(msg) = stream.next().await {
            let args = msg.message().body();

            println!("DUPSKOSSS: {:?}", args);
        }
    }

    pub async fn serve(self: Arc<Self>) {
        let proxy = StatusNotifierWatcherProxy::new(&self.connection)
            .await
            .unwrap();

        let mut register_stream = proxy
            .receive_status_notifier_item_registered()
            .await
            .unwrap();

        let mut unregister_stream = proxy
            .receive_status_notifier_item_unregistered()
            .await
            .unwrap();

        let mut map: HashMap<String, TrayItem> = HashMap::new();

        loop {
            tokio::select! {
                Some(message) = register_stream.next() => {
                    let service = message.args().unwrap().message;
                    println!("REGISTER: {:?}", service);

                    let self_clone = Arc::clone(&self);
                    let service_clone = service.clone();

                    let tray_item = TrayItem {
                        // service: service_clone.clone(),
                        handle: tokio::spawn(async move {
                            self_clone.handle_new_item(service_clone).await
                        })
                    };

                    map.insert(service, tray_item);

                }
                Some(message) = unregister_stream.next() => {
                    let service = message.args().unwrap().message;
                    println!("UNREGISTER: {:?}", message.args().unwrap());

                    let unregistered = map.remove(&service).unwrap();

                    let to_emit = TrayItemEvent {
                        service,
                        title: String::new(),
                        icon: Vec::new(),
                    };

                    self.app
                    .emit("tray_item_remove", serde_json::to_string(&to_emit).unwrap())
                    .unwrap();

                    unregistered.handle.abort();
                }
            }
        }
    }
}

#[interface(name = "org.kde.StatusNotifierHost")]
impl StatusNotifierHost {}
