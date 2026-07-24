use crate::dbus::spec::dbus_menu::DbusMenuProxy;
use crate::dbus::spec::status_notifier_host::StatusNotifierHost;
use crate::dbus::spec::status_notifier_item::StatusNotifierItemProxy;
use crate::dbus::spec::status_notifier_watcher::StatusNotifierWatcherProxy;

use tauri::async_runtime;
use tokio::sync::mpsc::{Receiver, Sender, channel};
use tokio::sync::oneshot;
use tokio::time::{Duration, sleep};
use tokio_stream::StreamExt;
use zbus::zvariant::Value;
use zbus::{Connection, conn::Builder};

use crate::dbus::events::{
    DbusEvent, MenuEntry, TrayItem, TrayItemNewIcon, TrayItemNewMenu, TrayItemNewProp,
};

use crate::dbus::utils::{
    TrayItemHandle, TrayItemHandles, argb32_to_png, load_icon, parse_as_menu_entry,
};

fn get_status_notifier_item_proxy_data(service: &str) -> Option<(String, String)> {
    service.split_once("/").map_or_else(
        || {
            service.rsplit_once(":").map(|(destination, _)| {
                (destination.to_string(), "/StatusNotifierItem".to_string())
            })
        },
        |(destination, path)| Some((destination.to_string(), format!("/{}", path))),
    )
}

async fn get_proxy<'a>(
    connection: &'a Connection,
    service: &str,
) -> zbus::Result<StatusNotifierItemProxy<'a>> {
    let (destination, path) = get_status_notifier_item_proxy_data(service)
        .expect("StatusNotiferItem service param should be correct");

    StatusNotifierItemProxy::new(connection, destination, path).await
}

async fn get_menu_proxy<'a>(
    connection: &'a Connection,
    service: &str,
    path: &String,
) -> zbus::Result<DbusMenuProxy<'a>> {
    let (destination, _) = get_status_notifier_item_proxy_data(service)
        .expect("StatusNotiferItem service param should be correct");

    DbusMenuProxy::new(connection, destination, path.to_string()).await
}

async fn emit_icon(id: &str, event_tx: &Sender<DbusEvent>, proxy: &StatusNotifierItemProxy<'_>) {
    let (icon_name, icon_theme_path) = tokio::join!(proxy.icon_name(), proxy.icon_theme_path());

    if let Ok(icon_name) = icon_name
        && let Ok(icon_theme_path) = icon_theme_path
        && let Ok(icon) = load_icon(icon_name, icon_theme_path).await
    {
        return event_tx
            .send(DbusEvent::TrayItemNewIcon(TrayItemNewIcon {
                id: id.to_string(),
                icon,
            }))
            .await
            .unwrap();
    }

    let icon_pixmap = proxy.icon_pixmap().await;

    if let Ok(icon_pixmap) = icon_pixmap {
        let icon = argb32_to_png(
            icon_pixmap[0].2.as_slice(),
            icon_pixmap[0].0,
            icon_pixmap[0].1,
        )
        .unwrap();

        return event_tx
            .send(DbusEvent::TrayItemNewIcon(TrayItemNewIcon {
                id: id.to_string(),
                icon,
            }))
            .await
            .unwrap();
    }
}

async fn emit_title(id: &str, event_tx: &Sender<DbusEvent>, proxy: &StatusNotifierItemProxy<'_>) {
    if let Ok(title) = proxy.title().await {
        event_tx
            .send(DbusEvent::TrayItemNewProp(TrayItemNewProp {
                id: id.to_owned(),
                prop: title,
                prop_name: String::from("title"),
            }))
            .await
            .unwrap();
    }
}

async fn emit_status(id: &str, event_tx: &Sender<DbusEvent>, proxy: &StatusNotifierItemProxy<'_>) {
    if let Ok(status) = proxy.status().await {
        event_tx
            .send(DbusEvent::TrayItemNewProp(TrayItemNewProp {
                id: id.to_owned(),
                prop: status,
                prop_name: "status".to_owned(),
            }))
            .await
            .unwrap();
    }
}

async fn handle_prop_changes(
    id: &str,
    event_tx: Sender<DbusEvent>,
    proxy: StatusNotifierItemProxy<'_>,
) {
    let mut title_stream = proxy.receive_title_changed().await;
    let mut status_stream = proxy.receive_status_changed().await;
    let mut icon_name_stream = proxy.receive_icon_name_changed().await;
    let mut icon_theme_path_stream = proxy.receive_icon_theme_path_changed().await;
    let mut icon_stream = proxy.receive_new_icon().await.unwrap();

    loop {
        tokio::select! {
            _ = title_stream.next() => emit_title(id, &event_tx, &proxy).await,
            _ = status_stream.next() => emit_status(id, &event_tx, &proxy).await,
            _ = icon_name_stream.next() => emit_icon(id, &event_tx, &proxy).await,
            _ = icon_theme_path_stream.next() => emit_icon(id, &event_tx, &proxy).await,
            _ = icon_stream.next() => emit_icon(id, &event_tx, &proxy).await,
        }
    }
}

async fn emit_menu(id: &str, event_tx: &Sender<DbusEvent>, proxy: &DbusMenuProxy<'_>) {
    let menu = get_menu(proxy).await;

    if let Ok(menu) = menu {
        event_tx
            .send(DbusEvent::TrayItemNewMenu(TrayItemNewMenu {
                id: id.to_owned(),
                menu,
            }))
            .await
            .unwrap();
    }
}

async fn handle_menu_changes(id: &str, event_tx: &Sender<DbusEvent>, proxy: &DbusMenuProxy<'_>) {
    let mut stream = proxy.receive_layout_updated().await.unwrap();

    while stream.next().await.is_some() {
        emit_menu(id, event_tx, proxy).await;
    }
}

async fn handle_menu_calls(menu_call_rx: &mut Receiver<i32>, proxy: &DbusMenuProxy<'_>) {
    while let Some(id) = menu_call_rx.recv().await {
        let _ = proxy.event(id, "clicked", &Value::I32(0), 0).await;
    }
}

async fn handle_menu(
    id: &str,
    service: &str,
    menu_path: &String,
    connection: &Connection,
    event_tx: Sender<DbusEvent>,
    menu_call_rx: &mut Receiver<i32>,
) {
    let proxy = get_menu_proxy(connection, service, menu_path)
        .await
        .unwrap();

    emit_menu(id, &event_tx, &proxy).await;

    tokio::join!(
        handle_menu_changes(id, &event_tx, &proxy),
        handle_menu_calls(menu_call_rx, &proxy)
    );
}

async fn get_menu<'a>(proxy: &DbusMenuProxy<'a>) -> zbus::Result<Vec<MenuEntry>> {
    let empty: [&str; 0] = [];
    let menu = proxy.get_layout(0, 1, &empty).await?;

    let entries: Vec<MenuEntry> = menu
        .1
        .2
        .iter()
        .filter_map(|entry| parse_as_menu_entry(entry).ok())
        .collect();

    Ok(entries)
}

async fn get_registered_item(proxy: &StatusNotifierItemProxy<'_>) -> zbus::Result<TrayItem> {
    let (id, title, status) = tokio::join!(proxy.id(), proxy.title(), proxy.status());

    Ok(TrayItem {
        id: id?,
        title: title?,
        status: status?,
    })
}

async fn register_new_item(
    service: String,
    connection: &Connection,
    event_tx: &Sender<DbusEvent>,
) -> zbus::Result<TrayItemHandle> {
    let proxy = get_proxy(connection, &service).await?;

    let tray_item = get_registered_item(&proxy).await?;

    let (unregister_tx, unregister_rx) = oneshot::channel();
    let (menu_call_tx, mut menu_call_rx) = channel(32);

    let handle = TrayItemHandle {
        id: tray_item.id.clone(),
        unregister_tx,
        menu_call_tx,
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

        let menu_path = proxy.menu().await.unwrap().to_string();

        tokio::select! {
            _ = handle_prop_changes(&id, event_tx.clone(), proxy) => {}
            _ = handle_menu(&id, &service, &menu_path, &connection, event_tx, &mut menu_call_rx) => {}
            _ = unregister_rx => {}
        }
    });

    Ok(handle)
}

async fn handle_registered_items(
    connection: &Connection,
    proxy: &StatusNotifierWatcherProxy<'_>,
    event_tx: &Sender<DbusEvent>,
    handles: TrayItemHandles,
) {
    let mut item_registered_stream = proxy
        .receive_status_notifier_item_registered()
        .await
        .unwrap();

    while let Some(message) = item_registered_stream.next().await {
        let service = message.args().unwrap().service;

        if let Ok(handle) = register_new_item(service.clone(), connection, event_tx).await {
            handles.insert(service.clone(), handle);
        }
    }
}

async fn handle_unregisted_items(
    proxy: &StatusNotifierWatcherProxy<'_>,
    event_tx: &Sender<DbusEvent>,
    handles: TrayItemHandles,
) {
    let mut item_unregistered_stream = proxy
        .receive_status_notifier_item_unregistered()
        .await
        .unwrap();

    while let Some(message) = item_unregistered_stream.next().await {
        let service = message.args().unwrap().service;

        let unregistered = handles.remove(&service);

        if let Some(unregistered) = unregistered {
            unregistered.unregister_tx.send(()).unwrap();

            event_tx
                .send(DbusEvent::UnregisterTrayItem(unregistered.id))
                .await
                .unwrap();
        }
    }
}

pub async fn run_host(event_tx: Sender<DbusEvent>, handles: TrayItemHandles) -> zbus::Result<()> {
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

    tokio::join!(
        handle_registered_items(&connection, &proxy, &event_tx, handles.clone()),
        handle_unregisted_items(&proxy, &event_tx, handles)
    );

    Ok(())
}
