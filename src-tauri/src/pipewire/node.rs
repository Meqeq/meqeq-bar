use libspa::{
    param::ParamType,
    pod::{Pod, Value, ValueArray},
    utils::dict::DictRef,
};
use pipewire::{
    channel::Sender,
    node::{Node, NodeInfoRef, NodeListener},
    proxy::{ProxyListener, ProxyT},
    registry::{GlobalObject, Registry},
};

use super::{
    events::{PwEvent, PwNode, PwNodeProps},
    utils::deserialize,
};

pub fn pw2ui(volume: f32) -> f32 {
    volume.clamp(0.0, 1.0).powf(1.0 / 3.0)
}

pub fn ui2pw(value: f32) -> f32 {
    value.clamp(0.0, 1.0).powf(3.0)
}

fn extract_info(info: &NodeInfoRef) -> Option<PwNode> {
    let mut node = PwNode::default();

    if let Some(props) = info.props() {
        if let Some(id) = props.get("object.id") {
            node.id = id.parse().unwrap();
        } else {
            return None;
        }

        if let Some(name) = props.get("node.name") {
            node.name = name.to_string();
        }

        if let Some(nick) = props.get("node.nick") {
            node.nick = nick.to_string();
        }

        if let Some(class) = props.get("media.class") {
            node.class = class.to_string();
        }

        if let Some(desc) = props.get("node.description") {
            node.description = desc.to_string();
        }

        if let Some(name) = props.get("alsa.card_name") {
            node.card_name = name.to_string();
        }

        if let Some(name) = props.get("alsa.mixer_name") {
            node.mixer_name = name.to_string();
        }

        if let Some(name) = props.get("alsa.name") {
            node.alsa_name = name.to_string();
        }

        if let Some(name) = props.get("device.icon-name") {
            node.icon_name = name.to_string();
        }

        if let Some(id) = props.get("device.id") {
            node.device_id = id.parse().unwrap();
        }

        if let Some(id) = props.get("client.id") {
            node.client_id = id.parse().unwrap();
        }

        return Some(node);
    }

    None
}

fn extract_props(id: u32, pod: Option<&Pod>) -> Option<PwNodeProps> {
    if let Some(param) = deserialize(pod) {
        let mut props = PwNodeProps::default();

        for prop in param.properties {
            match prop.key {
                libspa_sys::SPA_PROP_channelVolumes => {
                    if let Value::ValueArray(ValueArray::Float(value)) = &prop.value {
                        if value.len() >= 2 {
                            props.id = id;
                            props.volume.0 = pw2ui(value[0]);
                            props.volume.1 = pw2ui(value[1]);
                        }
                    }
                }
                libspa_sys::SPA_PROP_mute => {
                    if let Value::Bool(value) = prop.value {
                        props.muted = value;
                    }
                }
                _ => {}
            }
        }

        if props.id > 0 {
            return Some(props);
        }
    }

    None
}

pub fn handle_pipewire_node(
    global: &GlobalObject<&DictRef>,
    registry: &Registry,
    event_sender: Sender<PwEvent>,
) -> (Node, NodeListener, ProxyListener) {
    let proxy = registry.bind::<Node, _>(global).unwrap();

    let id = global.id;

    let listener = proxy
        .add_listener_local()
        .info({
            let sender = event_sender.clone();
            move |info| {
                if let Some(node) = extract_info(info) {
                    sender.send(PwEvent::Node(node)).unwrap();
                }
            }
        })
        .param({
            let sender = event_sender.clone();
            move |_, param_type, _, _, p5| {
                match param_type {
                    ParamType::Props => {
                        if let Some(node_props) = extract_props(id, p5) {
                            sender.send(PwEvent::NodeProps(node_props)).unwrap();
                        }
                    }
                    _ => {}
                };
            }
        })
        .register();

    let listener2 = proxy
        .upcast_ref()
        .add_listener_local()
        .removed({
            let sender = event_sender.clone();
            move || {
                sender.send(PwEvent::NodeRemoved(id)).unwrap();
            }
        })
        .register();

    proxy.subscribe_params(&[ParamType::Props]);

    (proxy, listener, listener2)
}
