use std::{collections::HashMap, process, sync::Arc};

use tauri::AppHandle;
use tokio::task;
use trpl::{JoinHandle, StreamExt};
use zbus::{fdo::DBusProxy, interface, message::Header, object_server::SignalEmitter, Connection};

use crate::dbus::status_notifier_item::StatusNotifierItemProxy;

use super::status_notifier_watcher::StatusNotifierWatcherProxy;

struct TrayItem {
    // path: String,
    // service: String,
    handle: JoinHandle<()>,
}

async fn kek(proxy: StatusNotifierItemProxy<'_>) {
    let (title, icon_name) = tokio::join!(proxy.title(), proxy.icon_name());

    println!("www: {:?} {:?}", title.unwrap(), icon_name.unwrap());
}

pub struct StatusNotifierHost {}

impl StatusNotifierHost {
    async fn handle_new_item(&self, item: String, connection: Connection) {
        let (service, path) = item.split_once("/").unwrap();

        let item_proxy = StatusNotifierItemProxy::new(
            &connection,
            service.to_string(),
            format!("/{}", path).to_string(),
        )
        .await
        .unwrap();

        let mut stream = item_proxy.receive_new_title().await.unwrap();

        while let Some(msg) = stream.next().await {
            let args = msg.message().body();

            println!("DUPSKOSSS: {:?}", args);
        }
    }

    pub async fn serve(self: Arc<Self>) {
        let connection = Connection::session().await.unwrap();

        connection
            .request_name(format!("org.kde.StatusNotifierHost-{}", process::id()))
            .await
            .unwrap();

        let proxy = StatusNotifierWatcherProxy::new(&connection).await.unwrap();

        let mut register_stream = proxy
            .receive_status_notifier_item_registered()
            .await
            .unwrap();

        let mut unregister_stream = proxy
            .receive_status_notifier_item_unregistered()
            .await
            .unwrap();

        let map: HashMap<String, TrayItem> = HashMap::new();

        loop {
            tokio::select! {
                Some(message) = register_stream.next() => {
                    let service = message.args().unwrap().message;
                    println!("REGISTER: {:?}", service);

                    let self_clone = Arc::clone(&self);
                    let connection_clone = connection.clone();

                    let tray_item = TrayItem {
                        handle: tokio::spawn(async move {
                         self_clone.handle_new_item(service, connection_clone).await
                        })
                    };

                }
                Some(message) = unregister_stream.next() => {
                    println!("UNREGISTER: {:?}", message.args().unwrap());
                }
            }
        }
    }
}

#[interface(name = "org.kde.StatusNotifierHost")]
impl StatusNotifierHost {}

// let mut stream = proxy
//     .receive_status_notifier_item_registered()
//     .await
//     .unwrap();

// while let Some(msg) = stream.next().await {
//     let args = msg.args().unwrap();

//     self.handle_new_item().await;

//     // let aa = task::spawn(self.handle_new_item());

//     // let (service, path) = args.message.split_once("/").unwrap();

//     // let item_proxy = StatusNotifierItemProxy::new(
//     //     &connection,
//     //     service.to_string(),
//     //     format!("/{}", path).to_string(),
//     // )
//     // .await
//     // .unwrap();
//     // // let kek = item_proxy.icon_pixmap().await.unwrap();

//     // let aa = task::spawn(kek(item_proxy));
// }

// let proxy = StatusNotifierWatcherProxy::new(&connection).await.unwrap();

// proxy.init().await.unwrap();

// let connection = zbus::connection::Builder::session()
//   .unwrap()
//   .name()
//   .unwrap()
//   .serve_at("/StatusNotifierHost", self)
//   .unwrap()
//   .build()
//   .await
//   .unwrap();

// async fn handle_new_item(&self, connection: Connection, item: String) {
//         let (service, path) = item.split_once("/").unwrap();

//         let item_proxy = StatusNotifierItemProxy::new(
//             &connection,
//             service.to_string(),
//             format!("/{}", path).to_string(),
//         )
//         .await
//         .unwrap();
//     }
