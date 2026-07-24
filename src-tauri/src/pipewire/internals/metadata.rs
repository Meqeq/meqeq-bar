use libspa::utils::dict::DictRef;
use pipewire::{
    channel::Sender,
    metadata::{Metadata, MetadataListener},
    registry::{GlobalObject, Registry},
};

use crate::pipewire::events::{PipewireEvent, PwDefault};

fn get_default(value: Option<&str>) -> PwDefault {
    let value = value.unwrap_or_default();
    let parsed = serde_json::from_str::<PwDefault>(value);

    parsed.unwrap_or_default()
}

pub fn handle_pipewire_metadata(
    global: &GlobalObject<&DictRef>,
    registry: &Registry,
    event_sender: Sender<PipewireEvent>,
) -> Option<(Metadata, MetadataListener)> {
    if let Some(name) = global.props.unwrap().get("metadata.name")
        && name.eq("default")
    {
        let proxy = registry.bind::<Metadata, _>(global).unwrap();

        let listener = proxy
            .add_listener_local()
            .property({
                let sender = event_sender.clone();
                move |_subject, key, _type_, value| {
                    match key.unwrap_or("") {
                        "default.audio.sink" => {
                            sender
                                .send(PipewireEvent::DefaultSink(get_default(value)))
                                .unwrap();
                        }
                        "default.audio.source" => {
                            sender
                                .send(PipewireEvent::DefaultSource(get_default(value)))
                                .unwrap();
                        }
                        _ => {}
                    };
                    0
                }
            })
            .register();

        return Some((proxy, listener));
    }
    None
}
