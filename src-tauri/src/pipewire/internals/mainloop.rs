use libspa::{
    param::ParamType,
    pod::{Object, Pod, Property, PropertyFlags, Value, ValueArray, serialize::PodSerializer},
    utils::dict::DictRef,
};

use pipewire::{
    Error,
    channel::{Receiver, Sender, channel},
    context::ContextRc,
    main_loop::MainLoopRc,
    registry::{GlobalObject, Registry, RegistryRc},
    types::ObjectType,
};

use tokio::sync::mpsc;

use crate::pipewire::{commands::PipewireCommand, events::PipewireEvent};

use super::{
    device::handle_pipewire_device,
    metadata::handle_pipewire_metadata,
    node::handle_pipewire_node,
    utils::{
        device::{device_set_profile, device_set_route_properties, ui2pw},
        listener_storage::{Listener, ListenerStorage},
    },
};

fn init() -> Result<(MainLoopRc, RegistryRc), Error> {
    let mainloop = MainLoopRc::new(None)?;
    let context = ContextRc::new(&mainloop, None)?;

    let core = context.connect_rc(None)?;
    let registry = core.get_registry_rc()?;

    Ok((mainloop, registry))
}

fn handle_global_listener(
    registry: &Registry,
    event_tx: Sender<PipewireEvent>,
    global: &GlobalObject<&DictRef>,
    listener_storage: &mut ListenerStorage,
) {
    match global.type_ {
        ObjectType::Node => {
            let (node, node_listener, proxy_listener) =
                handle_pipewire_node(global, registry, event_tx);

            listener_storage.nodes.insert(global.id, node);
            listener_storage.listeners.extend([
                Listener::Node(node_listener),
                Listener::Proxy(proxy_listener),
            ]);
        }
        ObjectType::Device => {
            let (device, device_listener) = handle_pipewire_device(global, registry, event_tx);

            listener_storage.devices.insert(global.id, device);
            listener_storage
                .listeners
                .push(Listener::Device(device_listener));
        }
        ObjectType::Metadata => {
            if let Some((metadata, listener)) = handle_pipewire_metadata(global, registry, event_tx)
            {
                listener_storage.metadata = Some(metadata);
                listener_storage
                    .listeners
                    .push(Listener::Metadata(listener));
            }
        }
        _ => {}
    };
}

fn handle_command(command: PipewireCommand, listener_storage: &ListenerStorage) {
    match command {
        PipewireCommand::SetDefaultSink(sink) => {
            if let Some(metadata) = &listener_storage.metadata {
                metadata.set_property(
                    0,
                    "default.audio.sink",
                    Some("Spa:String:JSON"),
                    Some(sink.as_str()),
                );
            }
        }
        PipewireCommand::SetDefaultSource(source) => {
            if let Some(metadata) = &listener_storage.metadata {
                metadata.set_property(
                    0,
                    "default.audio.source",
                    Some("Spa:String:JSON"),
                    Some(source.as_str()),
                );
            }
        }
        PipewireCommand::SetNodeVolume(id, volume) => {
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

            let node = listener_storage.nodes.get(&id);

            if let Some(node) = node {
                let v = &values.0.into_inner();
                let pod = Pod::from_bytes(v).unwrap();

                node.set_param(ParamType::Props, 0, pod);
            }
        }
        PipewireCommand::MuteNode(id, mute) => {
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

            let node = listener_storage.nodes.get(&id);

            if let Some(node) = node {
                let v = &values.0.into_inner();
                let pod = Pod::from_bytes(v).unwrap();

                node.set_param(ParamType::Props, 0, pod);
            }
        }
        PipewireCommand::SetDeviceVolume(id, route_index, route_device, volume) => {
            let device = listener_storage.devices.get(&id);

            if let Some(device) = device {
                let b = vec![ui2pw(volume[0]), ui2pw(volume[1])];

                device_set_route_properties(
                    device,
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
        PipewireCommand::SetDeviceMute(id, route_index, route_device, mute) => {
            let device = listener_storage.devices.get(&id);

            if let Some(device) = device {
                device_set_route_properties(
                    device,
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
        PipewireCommand::SetDeviceRoute(id, route_index, route_device) => {
            let device = listener_storage.devices.get(&id);

            if let Some(device) = device {
                device_set_route_properties(device, route_index, route_device, vec![])
            }
        }
        PipewireCommand::SetDeviceProfile(id, profile_index) => {
            let device = listener_storage.devices.get(&id);

            if let Some(device) = device {
                device_set_profile(device, profile_index)
            }
        }
    }
}

pub fn pipewire_main_loop(
    command_receiver: Receiver<PipewireCommand>,
    output_event_tx: mpsc::Sender<PipewireEvent>,
) {
    let (mainloop, registry) = init().unwrap_or_else(|e| {
        panic!("Error initializing Pipewire: {:?}", e);
    });

    let (event_tx, event_rx) = channel::<PipewireEvent>();
    let listener_storage = ListenerStorage::new();

    let _global_listener = registry
        .add_listener_local()
        .global({
            let registry = registry.clone();
            let listener_storage = listener_storage.clone();

            move |global| {
                handle_global_listener(
                    &registry,
                    event_tx.clone(),
                    global,
                    &mut listener_storage.borrow_mut(),
                );
            }
        })
        .register();

    let _event_passing = event_rx.attach(mainloop.loop_(), move |event| {
        let event_tx = output_event_tx.clone();
        tokio::spawn(async move {
            event_tx.send(event).await.expect("Error passing event");
        });
    });

    let _command_handling = command_receiver.attach(mainloop.loop_(), {
        move |command| {
            handle_command(command, &listener_storage.borrow());
        }
    });

    mainloop.run();
}
