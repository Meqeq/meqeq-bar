use tokio::sync::mpsc::Sender;

use hyprland::{
    data::{Workspace, Workspaces},
    event_listener::{AsyncEventListener, WindowEventData},
    shared::{HyprData, HyprDataActive},
};

use super::events::{ActiveWindow, HyprlandEvent, WorkspaceInfo};

async fn on_active_window_changed(data: Option<WindowEventData>, tx: Sender<HyprlandEvent>) {
    let active_window = match data {
        Some(active_window) => ActiveWindow {
            class: active_window.class,
            title: active_window.title,
        },
        None => ActiveWindow {
            class: "".to_string(),
            title: "".to_string(),
        },
    };

    tx.send(HyprlandEvent::ActiveWindowChange(active_window))
        .await
        .unwrap();

    let active_workspace = Workspace::get_active_async().await.unwrap().id;

    tx.send(HyprlandEvent::ActiveWorkspaceChange(active_workspace))
        .await
        .unwrap();
}

async fn emit_current_workspaces(tx: Sender<HyprlandEvent>) {
    let data = Workspaces::get_async().await.unwrap();

    let workspaces: Vec<WorkspaceInfo> = data
        .iter()
        .map(|workspace| WorkspaceInfo {
            id: workspace.id,
            name: workspace.name.clone(),
            monitor: workspace.monitor_id.unwrap_or(0),
            monitor_name: workspace.monitor.clone(),
        })
        .collect();

    tx.send(HyprlandEvent::WorkspacesChange(workspaces))
        .await
        .unwrap();
}

pub async fn start_listener(event_tx: Sender<HyprlandEvent>) {
    let mut listener = AsyncEventListener::new();

    let tx = event_tx.clone();
    listener.add_active_window_changed_handler(move |data| {
        Box::pin(on_active_window_changed(data, tx.clone()))
    });

    let tx = event_tx.clone();
    listener.add_workspace_added_handler(move |_| Box::pin(emit_current_workspaces(tx.clone())));

    let tx = event_tx.clone();
    listener.add_workspace_deleted_handler(move |_| Box::pin(emit_current_workspaces(tx.clone())));

    emit_current_workspaces(event_tx.clone()).await;

    listener.start_listener_async().await.unwrap();
}
