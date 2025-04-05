use std::process::Stdio;

use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    process::Command,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PipeWireObject {
    id: u32,
    type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Node {
    id: u32,
    type_: String,
    class: String,
    nick: String,
    description: String,
    name: String,
    muted: bool,
    volume: f32,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Metadata {
    id: u32,
    type_: String,
    default_sink: String,
    default_source: String,
}

async fn get_volume(id: u32) -> f32 {
    let output = Command::new("wpctl")
        .arg("get-volume")
        .arg(id.to_string())
        .output()
        .await
        .expect("Nie udało się uruchomić wpctl");

    let stdout = String::from_utf8_lossy(&output.stdout);

    match stdout.strip_prefix("Volume: ") {
        Some(res) => res.trim().parse::<f32>().unwrap(),
        None => 0.0,
    }
}

pub async fn set_volume(id: u32, volume: f32) {
    Command::new("wpctl")
        .arg("set-volume")
        .arg(id.to_string())
        .arg(volume.to_string())
        .output()
        .await
        .expect("Nie udało się uruchomić wpctl");
}

pub async fn set_default(id: u32) {
    Command::new("wpctl")
        .arg("set-default")
        .arg(id.to_string())
        .output()
        .await
        .expect("Nie udało się uruchomić wpctl");
}

pub async fn init_pipewire(app: AppHandle) {
    let mut output = Command::new("zsh")
        .arg("-c")
        .arg("pw-dump -mN | jq -cM --unbuffered '.[] | select(.type == \"PipeWire:Interface:Node\" or .type == \"PipeWire:Interface:Metadata\")' | jq -cM --unbuffered 'if .type == \"PipeWire:Interface:Node\" then { id: .id, type: .type, class: (.info.props.\"media.class\" // \"\"), nick: (.info.props.\"node.nick\" // \"\"), description: (.info.props.\"node.description\" // \"\"), name: .info.props.\"node.name\", muted: (.info.params.Props[0].mute // false), volume: 0 } else if .type == \"PipeWire:Interface:Metadata\" then { id: .id, type: .type, defaultSink: .metadata[]? | select(.key == \"default.audio.sink\").value.name, defaultSource: .metadata[]? | select(.key == \"default.audio.source\").value.name } end end'")
        .stdout(Stdio::piped())
        .spawn()
        .expect("Nie udało się uruchomić wpctl");

    let stdout = output
        .stdout
        .take()
        .expect("Nie udało się przekierować stdout");

    let mut reader = BufReader::new(stdout);

    let mut line = String::new();

    loop {
        reader.read_line(&mut line).await.unwrap();
        line.pop();

        let pw_object: PipeWireObject = serde_json::from_str(line.as_str()).unwrap();

        if pw_object.type_ == "PipeWire:Interface:Node" {
            let mut node: Node = serde_json::from_str(line.as_str()).unwrap();

            node.volume = get_volume(node.id).await;

            app.emit("pipewire_node", serde_json::to_string(&node).unwrap())
                .unwrap();
        } else if pw_object.type_ == "PipeWire:Interface:Metadata" {
            let metadata: Metadata = serde_json::from_str(line.as_str()).unwrap();

            app.emit(
                "pipewire_metadata",
                serde_json::to_string(&metadata).unwrap(),
            )
            .unwrap();
        }

        line.clear();
    }
}
