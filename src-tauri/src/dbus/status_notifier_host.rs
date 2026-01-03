use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tauri::async_runtime;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;
use tokio::time::{Duration, sleep};
use tokio_stream::StreamExt;
use zbus::{Connection, Result, conn::Builder, interface};

use crate::dbus::events::{DbusEvent, TrayItem, TrayItemNewIcon, TrayItemNewProp};
use crate::dbus::status_notifier_item::StatusNotifierItemProxy;
use crate::dbus::utils::load_icon;

use super::status_notifier_watcher::StatusNotifierWatcherProxy;

struct StatusNotifierHost {}

#[interface(name = "org.kde.StatusNotifierHost")]
impl StatusNotifierHost {}

async fn get_proxy<'a>(
    connection: &'a Connection,
    service: &String,
) -> Result<StatusNotifierItemProxy<'a>> {
    let (service, path) = service.split_once("/").unwrap();

    StatusNotifierItemProxy::new(
        &connection,
        service.to_string(),
        format!("/{}", path).to_string(),
    )
    .await
}

async fn emit_icon(id: &String, event_tx: &Sender<DbusEvent>, proxy: &StatusNotifierItemProxy<'_>) {
    let (icon_name, icon_theme_path) = tokio::join!(proxy.icon_name(), proxy.icon_theme_path());

    if let Ok(icon_name) = icon_name
        && let Ok(icon_theme_path) = icon_theme_path
    {
        if let Ok(icon) = load_icon(icon_name, icon_theme_path).await {
            event_tx
                .send(DbusEvent::TrayItemNewIcon(TrayItemNewIcon {
                    id: id.clone(),
                    icon,
                }))
                .await
                .unwrap();
        }
    }
}

async fn emit_title(
    id: &String,
    event_tx: &Sender<DbusEvent>,
    proxy: &StatusNotifierItemProxy<'_>,
) {
    if let Ok(title) = proxy.title().await {
        event_tx
            .send(DbusEvent::TrayItemNewProp(TrayItemNewProp {
                id: id.clone(),
                prop: title,
                prop_name: String::from("title"),
            }))
            .await
            .unwrap();
    }
}

async fn emit_status(
    id: &String,
    event_tx: &Sender<DbusEvent>,
    proxy: &StatusNotifierItemProxy<'_>,
) {
    if let Ok(status) = proxy.status().await {
        event_tx
            .send(DbusEvent::TrayItemNewProp(TrayItemNewProp {
                id: id.clone(),
                prop: status,
                prop_name: String::from("status"),
            }))
            .await
            .unwrap();
    }
}

async fn handle_prop_changes(
    id: &String,
    event_tx: Sender<DbusEvent>,
    proxy: StatusNotifierItemProxy<'_>,
) {
    let mut title_stream = proxy.receive_title_changed().await;
    let mut status_stream = proxy.receive_status_changed().await;
    let mut icon_name_stream = proxy.receive_icon_name_changed().await;
    let mut icon_theme_path_stream = proxy.receive_icon_theme_path_changed().await;

    loop {
        tokio::select! {
            _ = title_stream.next() => emit_title(id, &event_tx, &proxy).await,
            _ = status_stream.next() => emit_status(id, &event_tx, &proxy).await,
            _ = icon_name_stream.next() => emit_icon(id, &event_tx, &proxy).await,
            _ = icon_theme_path_stream.next() => emit_icon(id, &event_tx, &proxy).await,
        }
    }
}

async fn get_registered_item(proxy: &StatusNotifierItemProxy<'_>) -> Result<TrayItem> {
    let (id, title, status) = tokio::join!(proxy.id(), proxy.title(), proxy.status());

    Ok(TrayItem {
        id: id?,
        title: title?,
        status: status?,
    })
}

#[derive(Debug)]
struct TrayItemHandle {
    id: String,
    unregister_tx: oneshot::Sender<()>,
}

async fn register_new_item(
    service: String,
    connection: &Connection,
    event_tx: &Sender<DbusEvent>,
) -> Result<TrayItemHandle> {
    let proxy = get_proxy(connection, &service).await?;

    let tray_item = get_registered_item(&proxy).await?;

    let (unregister_tx, unregister_rx) = oneshot::channel();

    let handle = TrayItemHandle {
        id: tray_item.id.clone(),
        unregister_tx,
    };

    let id = tray_item.id.clone();
    let event_tx = event_tx.clone();
    let connection = connection.clone();

    event_tx
        .send(DbusEvent::RegisterTrayItem(tray_item))
        .await
        .unwrap();

    emit_icon(&id, &event_tx, &proxy).await;

    async_runtime::spawn(async move {
        let proxy = get_proxy(&connection, &service).await.unwrap();

        tokio::select! {
            _ = handle_prop_changes(&id, event_tx,  proxy) => {}
            _ = unregister_rx => {}
        }
    });

    Ok(handle)
}

async fn handle_registered_items(
    connection: &Connection,
    proxy: &StatusNotifierWatcherProxy<'_>,
    event_tx: &Sender<DbusEvent>,
    handles: Arc<RwLock<HashMap<String, TrayItemHandle>>>,
) {
    let mut item_registered_stream = proxy
        .receive_status_notifier_item_registered()
        .await
        .unwrap();

    while let Some(message) = item_registered_stream.next().await {
        let service = message.args().unwrap().service;

        if let Ok(handle) = register_new_item(service.clone(), connection, event_tx).await {
            handles.write().unwrap().insert(service, handle);
        }
    }
}

async fn handle_unregisted_items(
    proxy: &StatusNotifierWatcherProxy<'_>,
    event_tx: &Sender<DbusEvent>,
    handles: Arc<RwLock<HashMap<String, TrayItemHandle>>>,
) {
    let mut item_unregistered_stream = proxy
        .receive_status_notifier_item_unregistered()
        .await
        .unwrap();

    while let Some(message) = item_unregistered_stream.next().await {
        let service = message.args().unwrap().service;

        let unregistered = handles.write().unwrap().remove(&service);

        if let Some(unregistered) = unregistered {
            unregistered.unregister_tx.send(()).unwrap();

            event_tx
                .send(DbusEvent::UnregisterTrayItem(unregistered.id))
                .await
                .unwrap();
        }
    }
}

pub async fn run_host(event_tx: Sender<DbusEvent>) -> Result<()> {
    sleep(Duration::from_millis(100)).await;

    let host = StatusNotifierHost {};

    let connection = Builder::session()?
        .name("org.kde.StatusNotifierHost")?
        .serve_at("/StatusNotifierHost", host)?
        .build()
        .await
        .unwrap();

    let proxy = StatusNotifierWatcherProxy::new(&connection).await.unwrap();

    proxy
        .register_status_notifier_host("/StatusNotifierHost".to_string())
        .await
        .unwrap();

    let handles = Arc::new(RwLock::new(HashMap::new()));

    tokio::join!(
        handle_registered_items(&connection, &proxy, &event_tx, handles.clone()),
        handle_unregisted_items(&proxy, &event_tx, handles)
    );

    Ok(())
}

// use std::fmt;
// use std::{collections::HashMap, ops::Deref, sync::Mutex};

// use serde::{Deserialize, Serialize};
// use tauri::{AppHandle, Emitter, Manager};
// use tokio::{
//     fs::File,
//     io::{AsyncReadExt, BufReader},
// };
// use tokio_stream::StreamExt;
// use zbus::{
//     interface,
//     zvariant::{OwnedValue, Value},
//     Connection,
// };

// use crate::{app_state::AppState, dbus::status_notifier_item::StatusNotifierItemProxy};

// use super::{dbus_menu::DbusMenuProxy, status_notifier_watcher::StatusNotifierWatcherProxy};

// struct TrayItem<'a> {
//     // service: String,
//     // handle: JoinHandle<()>,
//     menu_proxy: DbusMenuProxy<'a>,
// }

// #[derive(Serialize, Deserialize, Debug)]
// struct TrayItemEvent {
//     service: String,
//     path: String,
//     title: String,
//     icon: Vec<u8>,
//     menu: Vec<MenuEntry>,
//     menu_path: String,
// }

// async fn load_icon(icon_name: String, path: String) -> Vec<u8> {
//     let file = File::open(format!("{}/{}.png", path, icon_name))
//         .await
//         .unwrap();

//     let mut reader = BufReader::new(file);

//     let mut buf = Vec::new();
//     reader.read_to_end(&mut buf).await.unwrap();

//     buf
// }

// #[derive(Serialize, Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
// struct MenuEntry {
//     id: i32,
//     label: String,
//     visible: bool,
//     type_: String,
// }

// #[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all = "camelCase")]
// struct TrayItemMenu {
//     service: String,
// }

// enum KekError {
//     NotAStructure,
//     WrongFieldCount(usize),
//     FieldTypeError(&'static str),
// }

// impl fmt::Debug for KekError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             KekError::NotAStructure => write!(f, "Not a structure"),
//             KekError::WrongFieldCount(count) => {
//                 write!(f, "Wrong field count: {}, expected 3", count)
//             }
//             KekError::FieldTypeError(t) => write!(f, "Field type error {}", t),
//         }
//     }
// }

// impl TryFrom<&OwnedValue> for MenuEntry {
//     type Error = KekError;

//     fn try_from(owned: &OwnedValue) -> Result<Self, Self::Error> {
//         let value = owned.deref();

//         match value {
//             Value::Structure(s) => {
//                 let fields = s.fields();

//                 if fields.len() != 3 {
//                     return Err(KekError::WrongFieldCount(fields.len()));
//                 }
//                 let id = match &fields[0] {
//                     Value::I32(n) => *n,
//                     _ => return Err(KekError::FieldTypeError("i32")),
//                 };

//                 let (label, visible, type_) = match &fields[1] {
//                     Value::Dict(dict) => {
//                         let key = Value::new("label");
//                         let label = match dict.get::<Value, Value>(&key) {
//                             Ok(Some(Value::Str(label))) => label.as_str().to_string(),
//                             _ => String::new(),
//                         };

//                         let key = Value::new("visible");
//                         let visible = match dict.get::<Value, Value>(&key) {
//                             Ok(Some(Value::Bool(visible))) => visible,
//                             _ => true,
//                         };

//                         let key = Value::new("type");
//                         let _type = match dict.get::<Value, Value>(&key) {
//                             Ok(Some(Value::Str(_type))) => _type.as_str().to_string(),
//                             _ => String::new(),
//                         };

//                         (label, visible, _type)
//                     }
//                     _ => return Err(KekError::NotAStructure),
//                 };

//                 Ok(MenuEntry {
//                     id,
//                     label,
//                     visible,
//                     type_,
//                 })
//             }
//             _ => Err(KekError::NotAStructure),
//         }
//     }
// }

// async fn get_menu<'a>(proxy: &DbusMenuProxy<'a>) -> Vec<MenuEntry> {
//     let empty: [&str; 0] = [];
//     let menu = proxy.get_layout(0, 1, &empty).await.unwrap();

//     let entries: Vec<MenuEntry> = menu
//         .1
//          .2
//         .iter()
//         .map(|entry| {
//             let menu_entry: MenuEntry = entry.try_into().unwrap();

//             menu_entry
//         })
//         .collect();

//     entries
// }

// pub struct StatusNotifierHost {
//     connection: Connection,
//     app: AppHandle,
// }

// impl fmt::Debug for StatusNotifierHost {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("StatusNotifierHost").finish()
//     }
// }

// impl StatusNotifierHost {
//     pub async fn connect(app: AppHandle) {
//         // let connection = {
//         //     let state = app.state::<Mutex<AppState>>();
//         //     let state = state.lock().unwrap();

//         //     state.connection.clone()
//         // };

//         // StatusNotifierHost { connection, app }
//     }

//     async fn handle_new_item(&self, item: String) -> DbusMenuProxy {
//         let (service, path) = item.split_once("/").unwrap();

//         let item_proxy = StatusNotifierItemProxy::new(
//             &self.connection,
//             service.to_string(),
//             format!("/{}", path).to_string(),
//         )
//         .await
//         .unwrap();

//         println!("NEW ITEM KEK");

//         let menu_path = item_proxy.menu().await.unwrap().to_string();
//         let menu_proxy =
//             DbusMenuProxy::new(&self.connection, service.to_string(), menu_path.clone())
//                 .await
//                 .unwrap();

//         let to_emit = TrayItemEvent {
//             service: service.to_string(),
//             path: format!("/{}", path).to_string(),
//             title: item_proxy.title().await.unwrap(),
//             icon: load_icon(
//                 item_proxy.icon_name().await.unwrap(),
//                 item_proxy.icon_theme_path().await.unwrap(),
//             )
//             .await,
//             menu: get_menu(&menu_proxy).await,
//             menu_path,
//         };

//         self.app
//             .emit("tray_item_add", serde_json::to_string(&to_emit).unwrap())
//             .unwrap();

//         menu_proxy
//     }

//     pub async fn serve(&self) {
//         let proxy = StatusNotifierWatcherProxy::new(&self.connection)
//             .await
//             .unwrap();

//         let mut register_stream = proxy
//             .receive_status_notifier_item_registered()
//             .await
//             .unwrap();

//         let mut unregister_stream = proxy
//             .receive_status_notifier_item_unregistered()
//             .await
//             .unwrap();

//         let mut map: HashMap<String, TrayItem> = HashMap::new();

//         loop {
//             tokio::select! {
//                 Some(message) = register_stream.next() => {
//                     let service = message.args().unwrap().message;
//                     println!("REGISTER: {:?}", service);

//                     let service_clone = service.clone();

//                     let tray_item = TrayItem {
//                         menu_proxy: self.handle_new_item(service_clone).await
//                     };

//                     map.insert(service, tray_item);

//                 }
//                 Some(message) = unregister_stream.next() => {
//                     let service = message.args().unwrap().message;
//                     println!("UNREGISTER: {:?}", message.args().unwrap());

//                     let _ = map.remove(&service).unwrap();

//                     let (service, path) = service.split_once("/").unwrap();

//                     let to_emit = TrayItemEvent {
//                         service: service.to_string(),
//                         path: path.to_string(),
//                         title: String::new(),
//                         icon: Vec::new(),
//                         menu: Vec::new(),
//                         menu_path: String::new(),
//                     };

//                     self.app
//                     .emit("tray_item_remove", serde_json::to_string(&to_emit).unwrap())
//                     .unwrap();
//                 }
//             }
//         }
//     }
// }

// #[interface(name = "org.kde.StatusNotifierHost")]
// impl StatusNotifierHost {}
