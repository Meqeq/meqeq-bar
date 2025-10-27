use std::rc::Rc;

use libspa::utils::dict::DictRef;
use pipewire::{
    channel::Sender,
    metadata::{Metadata, MetadataListener},
    registry::{GlobalObject, Registry},
};

use super::events::PwEvent;

pub fn handle_pipewire_metadata(
    global: &GlobalObject<&DictRef>,
    registry: &Registry,
    event_sender: Rc<Sender<PwEvent>>,
) -> Option<(Metadata, MetadataListener)> {
    if let Some(name) = global.props.unwrap().get("metadata.name") {
        if name.eq("default") {
            let proxy = registry.bind::<Metadata, _>(global).unwrap();

            let listener = proxy
                .add_listener_local()
                .property({
                    let sender = Rc::clone(&event_sender);
                    move |_subject, key, _type_, value| {
                        println!("DDDDDDDDDDDD {:?} {:?}", key, value);
                        match key.unwrap_or("") {
                            "default.audio.sink" => {
                                sender
                                    .send(PwEvent::DefaultSink(value.unwrap_or("").to_string()))
                                    .unwrap();
                            }
                            "default.audio.source" => {
                                sender
                                    .send(PwEvent::DefaultSource(value.unwrap_or("").to_string()))
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
    }
    None
}
