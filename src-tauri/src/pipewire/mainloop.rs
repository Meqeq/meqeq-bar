use std::{cell::RefCell, collections::HashMap, rc::Rc};

use libspa::{
    param::ParamType,
    pod::{serialize::PodSerializer, Object, Pod, Property, PropertyFlags, Value, ValueArray},
    utils::dict::DictRef,
};

use pipewire::{
    channel::{channel, Receiver, Sender},
    context::Context,
    core::Core,
    device::{Device, DeviceListener},
    main_loop::MainLoop,
    metadata::{Metadata, MetadataListener},
    node::{Node, NodeListener},
    proxy::ProxyListener,
    registry::{GlobalObject, Registry},
    types::ObjectType,
};
use tauri::{AppHandle, Emitter};

use crate::pipewire::{device::handle_pipewire_device, node::ui2pw};

use super::{
    commands::PwCommand,
    events::PwEvent,
    metadata::handle_pipewire_metadata,
    node::handle_pipewire_node,
    utils::{device_set_profile, device_set_route_properties},
};

fn init() -> (Rc<MainLoop>, Rc<Registry>, Rc<Context>, Rc<Core>) {
    let mainloop = MainLoop::new(None).unwrap();
    let context = Context::new(&mainloop).unwrap();
    let core = context.connect(None).unwrap();
    let registry = core.get_registry().unwrap();

    (
        Rc::new(mainloop),
        Rc::new(registry),
        Rc::new(context),
        Rc::new(core),
    )
}

// #[derive(Debug)]
enum HandleResult {
    Node((Node, NodeListener, ProxyListener)),
    Metadata((Metadata, MetadataListener)),
    Device((Rc<Device>, DeviceListener)),
    None,
}

enum Listener {
    Proxy(ProxyListener),
    Node(NodeListener),
    Metadata(MetadataListener),
    Device(DeviceListener),
}

fn handle_global(
    global: &GlobalObject<&DictRef>,
    registry: Rc<Registry>,
    event_sender: Rc<Sender<PwEvent>>,
) -> HandleResult {
    match global.type_ {
        ObjectType::Node => {
            HandleResult::Node(handle_pipewire_node(global, &registry, event_sender))
        }

        ObjectType::Metadata => {
            return match handle_pipewire_metadata(global, &registry, event_sender) {
                Some(result) => HandleResult::Metadata(result),
                None => HandleResult::None,
            };
        }

        ObjectType::Device => {
            HandleResult::Device(handle_pipewire_device(global, &registry, event_sender))
        }

        _ => HandleResult::None,
    }
}

pub fn pipewire_main_loop(command_receiver: Receiver<PwCommand>, app: AppHandle) {
    let (mainloop, registry, _context, _core) = init();
    let (event_tx, event_rx): (Sender<PwEvent>, Receiver<PwEvent>) = channel();

    let registry_weak = Rc::downgrade(&registry);

    let event_sender = Rc::new(event_tx);

    let nodes = Rc::new(RefCell::new(HashMap::new()));
    let devices = Rc::new(RefCell::new(HashMap::new()));
    let metadata = Rc::new(RefCell::new(None));
    let listeners = Rc::new(RefCell::new(Vec::new()));

    let nodes_ref = Rc::clone(&nodes);
    let devices_ref = Rc::clone(&devices);
    let metadata_ref = Rc::clone(&metadata);
    let listeners_ref = Rc::clone(&listeners);

    let _listener = registry
        .add_listener_local()
        .global(move |global| {
            match handle_global(
                global,
                registry_weak.upgrade().unwrap(),
                Rc::clone(&event_sender),
            ) {
                HandleResult::Node((node, node_listener, proxy_listener)) => {
                    nodes_ref.borrow_mut().insert(global.id, node);
                    listeners_ref
                        .borrow_mut()
                        .push(Listener::Node(node_listener));
                    listeners_ref
                        .borrow_mut()
                        .push(Listener::Proxy(proxy_listener));
                }
                HandleResult::Metadata((metadata, listener)) => {
                    *metadata_ref.borrow_mut() = Some(metadata);
                    listeners_ref
                        .borrow_mut()
                        .push(Listener::Metadata(listener));
                }
                HandleResult::Device((device, listener)) => {
                    devices_ref.borrow_mut().insert(global.id, device);
                    listeners_ref.borrow_mut().push(Listener::Device(listener));
                }

                _ => {}
            }

            // let _ = core.sync(0);
        })
        .register();

    let _kek = event_rx.attach(mainloop.loop_(), move |event| match event {
        PwEvent::Node(node) => {
            app.emit("pw_node", serde_json::to_string(&node).unwrap())
                .unwrap();
        }
        PwEvent::NodeProps(node_props) => {
            app.emit("pw_node_props", serde_json::to_string(&node_props).unwrap())
                .unwrap();
        }
        PwEvent::NodeRemoved(id) => {
            app.emit("pw_node_removed", id.to_string().as_str())
                .unwrap();
        }
        PwEvent::DefaultSink(sink) => {
            app.emit("pw_default_sink", sink.as_str()).unwrap();
        }
        PwEvent::DefaultSource(source) => {
            app.emit("pw_default_source", source.as_str()).unwrap();
        }
        PwEvent::Device(device) => {
            app.emit("pw_device", serde_json::to_string(&device).unwrap())
                .unwrap();
        }
        PwEvent::DeviceEnumProfile(enum_profile) => {
            app.emit(
                "pw_device_enum_profile",
                serde_json::to_string(&enum_profile).unwrap(),
            )
            .unwrap();
        }
        PwEvent::DeviceEnumRoute(enum_route) => {
            app.emit(
                "pw_device_enum_route",
                serde_json::to_string(&enum_route).unwrap(),
            )
            .unwrap();
        }
        PwEvent::DeviceProfile(profile) => {
            app.emit(
                "pw_device_profile",
                serde_json::to_string(&profile).unwrap(),
            )
            .unwrap();
        }
        PwEvent::DeviceRoute(route) => {
            app.emit("pw_device_route", serde_json::to_string(&route).unwrap())
                .unwrap();
        }
    });

    let nodes_ref = Rc::clone(&nodes);
    let devices_ref = Rc::clone(&devices);
    let metadata_ref = Rc::clone(&metadata);

    let _lel = command_receiver.attach(mainloop.loop_(), move |command| match command {
        PwCommand::SetDefaultSink(sink) => {
            if let Some(metadata) = metadata_ref.borrow().as_ref() {
                metadata.set_property(
                    0,
                    "default.audio.sink",
                    Some("Spa:String:JSON"),
                    Some(sink.as_str()),
                );
            }
        }
        PwCommand::SetDefaultSource(source) => {
            if let Some(metadata) = metadata_ref.borrow().as_ref() {
                metadata.set_property(
                    0,
                    "default.audio.source",
                    Some("Spa:String:JSON"),
                    Some(source.as_str()),
                );
            }
        }
        PwCommand::SetNodeVolume(id, volume) => {
            let b = vec![ui2pw(volume[0]), ui2pw(volume[1])];

            let values = PodSerializer::serialize(
                std::io::Cursor::new(Vec::new()),
                &Value::Object(Object {
                    type_: libspa_sys::SPA_TYPE_OBJECT_Props,
                    id: libspa_sys::SPA_PARAM_Props,
                    properties: vec![Property {
                        key: libspa_sys::SPA_PROP_channelVolumes,
                        flags: PropertyFlags::empty(),
                        value: Value::ValueArray(ValueArray::Float(b.clone())),
                    }],
                }),
            )
            .unwrap();

            let nodes = nodes_ref.borrow();

            let node = nodes.get(&id);

            if let Some(node) = node {
                let v = &values.0.into_inner();
                let pod = Pod::from_bytes(v).unwrap();

                node.set_param(ParamType::Props, 0, pod);
            }
        }
        PwCommand::SetNodeMute(id, mute) => {
            let values = PodSerializer::serialize(
                std::io::Cursor::new(Vec::new()),
                &Value::Object(Object {
                    type_: libspa_sys::SPA_TYPE_OBJECT_Props,
                    id: libspa_sys::SPA_PARAM_Props,
                    properties: vec![Property {
                        key: libspa_sys::SPA_PROP_mute,
                        flags: PropertyFlags::empty(),
                        value: Value::Bool(mute),
                    }],
                }),
            )
            .unwrap();

            let nodes = nodes_ref.borrow();

            let node = nodes.get(&id);

            if let Some(node) = node {
                let v = &values.0.into_inner();
                let pod = Pod::from_bytes(v).unwrap();

                node.set_param(ParamType::Props, 0, pod);
            }
        }
        PwCommand::SetDeviceVolume(id, route_index, route_device, volume) => {
            let devices = devices_ref.borrow();
            let device = devices.get(&id);

            if let Some(device) = device {
                let b = vec![ui2pw(volume[0]), ui2pw(volume[1])];

                device_set_route_properties(
                    &device,
                    route_index,
                    route_device,
                    vec![Property {
                        key: libspa_sys::SPA_PROP_channelVolumes,
                        flags: PropertyFlags::empty(),
                        value: Value::ValueArray(ValueArray::Float(b.clone())),
                    }],
                )
            }
        }
        PwCommand::SetDeviceMute(id, route_index, route_device, mute) => {
            let devices = devices_ref.borrow();
            let device = devices.get(&id);

            if let Some(device) = device {
                device_set_route_properties(
                    &device,
                    route_index,
                    route_device,
                    vec![Property {
                        key: libspa_sys::SPA_PROP_mute,
                        flags: PropertyFlags::empty(),
                        value: Value::Bool(mute),
                    }],
                )
            }
        }
        PwCommand::SetDeviceRoute(id, route_index, route_device) => {
            let devices = devices_ref.borrow();
            let device = devices.get(&id);

            if let Some(device) = device {
                device_set_route_properties(&device, route_index, route_device, vec![])
            }
        }
        PwCommand::SetDeviceProfile(id, profile_index) => {
            let devices = devices_ref.borrow();
            let device = devices.get(&id);

            if let Some(device) = device {
                device_set_profile(&device, profile_index)
            }
        }
    });

    mainloop.run();
}
