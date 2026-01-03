use std::{
    collections::HashMap,
    fmt,
    ops::Deref,
    sync::{Arc, RwLock},
};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
    sync::{mpsc, oneshot},
};
use zbus::zvariant::{OwnedValue, Value};

use crate::dbus::events::MenuEntry;

#[derive(Clone)]
pub struct ReadHandle<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> ReadHandle<T> {
    pub async fn with_read<R>(&self, f: impl FnOnce(&T) -> R) -> R {
        let guard = self.inner.read().unwrap();
        f(&guard)
    }
}

pub struct WriteHandle<T> {
    inner: Arc<RwLock<T>>,
}

impl<T> WriteHandle<T> {
    pub async fn with_write<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        let mut guard = self.inner.write().unwrap();
        f(&mut guard)
    }
}

pub fn rw_lock_handles<T>(container: T) -> (ReadHandle<T>, WriteHandle<T>) {
    let inner = Arc::new(RwLock::new(container));

    (
        ReadHandle {
            inner: inner.clone(),
        },
        WriteHandle { inner },
    )
}

pub async fn load_icon(icon_name: String, path: String) -> tokio::io::Result<Vec<u8>> {
    let file = File::open(format!("{}/{}.png", path, icon_name)).await?;

    let mut reader = BufReader::new(file);

    let mut buf = Vec::new();
    reader.read_to_end(&mut buf).await?;

    Ok(buf)
}

pub enum MenuParseError {
    NotAStructure,
    WrongFieldCount(usize),
    FieldTypeError(&'static str),
}

impl fmt::Debug for MenuParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MenuParseError::NotAStructure => write!(f, "Not a structure"),
            MenuParseError::WrongFieldCount(count) => {
                write!(f, "Wrong field count: {}, expected 3", count)
            }
            MenuParseError::FieldTypeError(t) => write!(f, "Field type error {}", t),
        }
    }
}

pub fn parse_as_menu_entry(owned: &OwnedValue) -> Result<MenuEntry, MenuParseError> {
    match owned.deref() {
        Value::Structure(s) => {
            let fields = s.fields();

            if fields.len() != 3 {
                return Err(MenuParseError::WrongFieldCount(fields.len()));
            }
            let id = match &fields[0] {
                Value::I32(n) => *n,
                _ => return Err(MenuParseError::FieldTypeError("i32")),
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
                        _ => String::from("entry"),
                    };

                    (label, visible, _type)
                }
                _ => return Err(MenuParseError::NotAStructure),
            };

            Ok(MenuEntry {
                id,
                label,
                visible,
                type_,
            })
        }
        _ => Err(MenuParseError::NotAStructure),
    }
}

#[derive(Debug)]
pub struct TrayItemHandle {
    pub id: String,
    pub menu_call_tx: mpsc::Sender<i32>,
    pub unregister_tx: oneshot::Sender<()>,
}

struct TrayItemHandlesInner {
    handles: HashMap<String, TrayItemHandle>,
    lookup: HashMap<String, String>,
}

#[derive(Clone)]
pub struct TrayItemHandles {
    inner: Arc<RwLock<TrayItemHandlesInner>>,
}

impl TrayItemHandles {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(TrayItemHandlesInner {
                handles: HashMap::new(),
                lookup: HashMap::new(),
            })),
        }
    }

    pub fn insert(&self, key: String, handle: TrayItemHandle) {
        let mut inner = self.inner.write().unwrap();

        inner.lookup.insert(handle.id.clone(), key.clone());
        inner.handles.insert(key, handle);
    }

    pub fn with_read_by_id<R>(
        &self,
        id: &String,
        f: impl FnOnce(Option<&TrayItemHandle>) -> R,
    ) -> R {
        let inner = self.inner.read().unwrap();

        if let Some(service) = inner.lookup.get(id) {
            f(inner.handles.get(service))
        } else {
            f(None)
        }
    }

    // pub fn with_read_by_service<R>(
    //     &self,
    //     service: &String,
    //     f: impl FnOnce(Option<&TrayItemHandle>) -> R,
    // ) -> R {
    //     let inner = self.inner.read().unwrap();
    //     f(inner.handles.get(service))
    // }

    pub fn remove(&self, service: &String) -> Option<TrayItemHandle> {
        let mut inner = self.inner.write().unwrap();

        if let Some(item) = inner.handles.remove(service) {
            inner.lookup.remove(&item.id);

            Some(item)
        } else {
            None
        }
    }
}
