use std::fmt;
use std::io::Write;
use std::{collections::HashMap, ops::Deref, sync::Mutex};

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter, Manager};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};
use tokio_stream::StreamExt;
use zbus::{
    interface,
    zvariant::{OwnedValue, Value},
    Connection,
};

use crate::{app_state::AppState, dbus::status_notifier_item::StatusNotifierItemProxy};

use super::{dbus_menu::DbusMenuProxy, status_notifier_watcher::StatusNotifierWatcherProxy};

struct TrayItem<'a> {
    // service: String,
    // handle: JoinHandle<()>,
    menu_proxy: DbusMenuProxy<'a>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrayItemEvent {
    service: String,
    path: String,
    title: String,
    icon: Vec<u8>,
    menu: Vec<MenuEntry>,
    menu_path: String,
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MenuEntry {
    id: i32,
    label: String,
    visible: bool,
    type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct TrayItemMenu {
    service: String,
}

enum KekError {
    NotAStructure,
    WrongFieldCount(usize),
    FieldTypeError(&'static str),
}

impl fmt::Debug for KekError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KekError::NotAStructure => write!(f, "Not a structure"),
            _ => write!(f, "LEL"),
            KekError::WrongFieldCount(count) => {
                write!(f, "Wrong field count: {}, expected 3", count)
            }
            KekError::FieldTypeError(t) => write!(f, "Field type error {}", t),
        }
    }
}

impl TryFrom<&OwnedValue> for MenuEntry {
    type Error = KekError;

    fn try_from(owned: &OwnedValue) -> Result<Self, Self::Error> {
        let value = owned.deref();

        match value {
            Value::Structure(s) => {
                let fields = s.fields();

                if fields.len() != 3 {
                    return Err(KekError::WrongFieldCount(fields.len()));
                }
                let id = match &fields[0] {
                    Value::I32(n) => *n,
                    _ => return Err(KekError::FieldTypeError("i32")),
                };

                let (label, visible, type_) = match &fields[1] {
                    Value::Dict(dict) => {
                        let key = Value::new("label");
                        let label = match dict.get::<Value, Value>(&key) {
                            Ok(Some(Value::Str(label))) => label.as_str().to_string(),
                            _ => String::new(),
                        };

                        let key = Value::new("visible");
                        let visible = match dict.get::<Value, Value>(&key) {
                            Ok(Some(Value::Bool(visible))) => visible,
                            _ => true,
                        };

                        let key = Value::new("type");
                        let _type = match dict.get::<Value, Value>(&key) {
                            Ok(Some(Value::Str(_type))) => _type.as_str().to_string(),
                            _ => String::new(),
                        };

                        (label, visible, _type)
                    }
                    _ => return Err(KekError::NotAStructure),
                };

                Ok(MenuEntry {
                    id,
                    label,
                    visible,
                    type_,
                })
            }
            _ => Err(KekError::NotAStructure),
        }
    }
}

async fn get_menu<'a>(proxy: &DbusMenuProxy<'a>) -> Vec<MenuEntry> {
    let empty: [&str; 0] = [];
    let menu = proxy.get_layout(0, 1, &empty).await.unwrap();

    let entries: Vec<MenuEntry> = menu
        .1
         .2
        .iter()
        .map(|entry| {
            let menu_entry: MenuEntry = entry.try_into().unwrap();

            menu_entry
        })
        .collect();

    entries
}

pub struct StatusNotifierHost {
    connection: Connection,
    app: AppHandle,
}

impl fmt::Debug for StatusNotifierHost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("StatusNotifierHost").finish()
    }
}

impl StatusNotifierHost {
    pub async fn connect(app: AppHandle) -> Self {
        let connection = {
            let state = app.state::<Mutex<AppState>>();
            let state = state.lock().unwrap();

            state.connection.clone()
        };

        StatusNotifierHost { connection, app }
    }

    async fn handle_new_item(&self, item: String) -> DbusMenuProxy {
        let (service, path) = item.split_once("/").unwrap();

        let item_proxy = StatusNotifierItemProxy::new(
            &self.connection,
            service.to_string(),
            format!("/{}", path).to_string(),
        )
        .await
        .unwrap();

        let menu_path = item_proxy.menu().await.unwrap().to_string();
        let menu_proxy =
            DbusMenuProxy::new(&self.connection, service.to_string(), menu_path.clone())
                .await
                .unwrap();

        let to_emit = TrayItemEvent {
            service: service.to_string(),
            path: format!("/{}", path).to_string(),
            title: item_proxy.title().await.unwrap(),
            icon: load_icon(
                item_proxy.icon_name().await.unwrap(),
                item_proxy.icon_theme_path().await.unwrap(),
            )
            .await,
            menu: get_menu(&menu_proxy).await,
            menu_path,
        };

        self.app
            .emit("tray_item_add", serde_json::to_string(&to_emit).unwrap())
            .unwrap();

        menu_proxy
    }

    pub async fn serve(&self) {
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

                    let service_clone = service.clone();

                    let tray_item = TrayItem {
                        menu_proxy: self.handle_new_item(service_clone).await
                    };

                    map.insert(service, tray_item);

                }
                Some(message) = unregister_stream.next() => {
                    let service = message.args().unwrap().message;
                    println!("UNREGISTER: {:?}", message.args().unwrap());

                    let _ = map.remove(&service).unwrap();

                    let (service, path) = service.split_once("/").unwrap();

                    let to_emit = TrayItemEvent {
                        service: service.to_string(),
                        path: path.to_string(),
                        title: String::new(),
                        icon: Vec::new(),
                        menu: Vec::new(),
                        menu_path: String::new(),
                    };

                    self.app
                    .emit("tray_item_remove", serde_json::to_string(&to_emit).unwrap())
                    .unwrap();
                }
            }
        }
    }
}

#[interface(name = "org.kde.StatusNotifierHost")]
impl StatusNotifierHost {}
