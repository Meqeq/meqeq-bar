use std::sync::{Arc, RwLock};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};

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
