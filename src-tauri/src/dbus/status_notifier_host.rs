use std::{
    collections::{BTreeMap, HashMap},
    ops::Deref,
    process,
    sync::Arc,
};

use serde::{Deserialize, Serialize};
use serde_json::from_value;
use std::error::Error;
use tauri::{AppHandle, Emitter};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
    task::JoinHandle,
};
use tokio_stream::StreamExt;
use zbus::{
    interface,
    zvariant::{self, OwnedValue, Value},
    Connection,
};

use crate::dbus::status_notifier_item::StatusNotifierItemProxy;

use super::{dbus_menu::DbusMenuProxy, status_notifier_watcher::StatusNotifierWatcherProxy};

struct TrayItem {
    // service: String,
    handle: JoinHandle<()>,
}

#[derive(Serialize, Deserialize, Debug)]
struct TrayItemEvent {
    service: String,
    path: String,
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

// impl fmt::Display for ConversionError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ConversionError::NotAStructure => write!(f, "Value is not a structure"),
//             ConversionError::WrongFieldCount(count) => {
//                 write!(f, "Expected 3 fields, got {}", count)
//             }
//             ConversionError::FieldTypeError(field) => {
//                 write!(f, "Field '{}' has unexpected type", field)
//             }
//         }
//     }
// // }

// impl Error for ConversionError {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct MenuEntry {
    position: i32,
    label: String,
    visible: bool,
    type_: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TrayItemMenu {
    service: String,
    entries: Vec<MenuEntry>,
}

#[derive(Debug)]
enum CError {
    NotAStructure,
    WrongFieldCount(usize),
    FieldTypeError(&'static str),
}

impl TryFrom<&OwnedValue> for MenuEntry {
    type Error = CError;

    fn try_from(owned: &OwnedValue) -> Result<Self, Self::Error> {
        let value = owned.deref();

        match value {
            Value::Structure(s) => {
                let fields = s.fields();

                if fields.len() != 3 {
                    return Err(CError::WrongFieldCount(fields.len()));
                }

                let position = match &fields[0] {
                    Value::I32(n) => *n,
                    _ => return Err(CError::FieldTypeError("i32")),
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
                    _ => return Err(CError::NotAStructure),
                };

                Ok(MenuEntry {
                    position,
                    label,
                    visible,
                    type_,
                })
            }
            _ => Err(CError::NotAStructure),
        }
    }
}

async fn get_menu(connection: &Connection, service: String, path: String) -> TrayItemMenu {
    let proxy = DbusMenuProxy::new(connection, service.clone(), path)
        .await
        .unwrap();

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

    TrayItemMenu { service, entries }
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

        let menu = get_menu(
            &self.connection,
            service.to_string(),
            item_proxy.menu().await.unwrap().to_string(),
        )
        .await;

        self.app
            .emit("tray_item_menu", serde_json::to_string(&menu).unwrap())
            .unwrap();

        let to_emit = TrayItemEvent {
            service: service.to_string(),
            path: path.to_string(),
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

                    let (service, path) = service.split_once("/").unwrap();

                    let to_emit = TrayItemEvent {
                        service: service.to_string(),
                        path: path.to_string(),
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
