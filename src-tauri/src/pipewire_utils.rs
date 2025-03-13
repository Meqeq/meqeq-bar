use std::{
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use pipewire::{
    context::Context,
    main_loop::MainLoop,
};
use serde::{Deserialize, Serialize};
use tauri::{command, AppHandle, Emitter};

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

fn get_volume(id: u32) -> f32 {
    let output = Command::new("wpctl")
        .arg("get-volume")
        .arg(id.to_string())
        .output()
        .expect("Nie udało się uruchomić wpctl");

    let stdout = String::from_utf8_lossy(&output.stdout);
    // Zakładamy, że wynik wygląda mniej więcej tak: "Volume: 0.45"
    match stdout.strip_prefix("Volume: ") {
        Some(res) => res.trim().parse::<f32>().unwrap(),
        None => 0.0,
    }
}

pub fn set_volume(id: u32, volume: f32) {
    let output = Command::new("wpctl")
        .arg("set-volume")
        .arg(id.to_string())
        .arg(volume.to_string())
        .output()
        .expect("Nie udało się uruchomić wpctl");
}


pub fn set_default(id: u32) {
    let output = Command::new("wpctl")
        .arg("set-default")
        .arg(id.to_string())
        .output()
        .expect("Nie udało się uruchomić wpctl");
}


fn get_pipewire(app: AppHandle) {
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
    let reader = BufReader::new(stdout);

    println!("DUPSKO");

    // Odczytujemy dane linia po linii
    for line in reader.lines() {
        match line {
            Ok(data) => {
                // println!("OOOOOOOO: {:?}", data);
                let pw_object: PipeWireObject = serde_json::from_str(data.as_str()).unwrap();

                if pw_object.type_ == "PipeWire:Interface:Node" {
                    let mut node: Node = serde_json::from_str(data.as_str()).unwrap();

                    node.volume = get_volume(node.id);

                    app.emit("pipewire_node", serde_json::to_string(&node).unwrap())
                        .unwrap();

                    println!("Otrzymano: {:?}", node);
                } else if pw_object.type_ == "PipeWire:Interface:Metadata" {
                    let metadata: Metadata = serde_json::from_str(data.as_str()).unwrap();

                    app.emit(
                        "pipewire_metadata",
                        serde_json::to_string(&metadata).unwrap(),
                    )
                    .unwrap();

                    println!("Otrzymano: {:?}", metadata);
                }

                // Możesz dodać tutaj przetwarzanie lub inne operacje na danych.
            }
            Err(e) => {
                eprintln!("Błąd odczytu: {}", e);
                break;
            }
        }
    }
}

#[command]
pub async fn set_up_pipewire(app: AppHandle) -> Result<(), ()> {
    let mainloop = MainLoop::new(None).unwrap();
    let context = Context::new(&mainloop).unwrap();
    let core = context.connect(None).unwrap();
    let registry = core.get_registry().unwrap();

    get_pipewire(app);

    Ok(())
}
