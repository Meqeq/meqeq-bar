use libspa::{
    param::ParamType,
    pod::{Pod, Property, Value, ValueArray},
    utils::dict::DictRef,
};
use pipewire::{
    channel::Sender,
    device::{Device, DeviceInfoRef, DeviceListener},
    registry::{GlobalObject, Registry},
};

use crate::pipewire::events::{
    PipewireEvent, PwDevice, PwDeviceProfile, PwDeviceRoute, PwDeviceRouteDirection, PwMediaClass,
};

use super::utils::device::{deserialize, pw2ui};

fn parse_media_class(prop: Property) -> Vec<PwMediaClass> {
    let result = vec![];

    if let Value::Struct(classes_struct) = prop.value {
        let skip = match classes_struct.first() {
            Some(Value::Int(_)) => 1,
            _ => 0,
        };

        let result: Vec<PwMediaClass> = classes_struct
            .iter()
            .skip(skip)
            .filter_map(|class| {
                if let Value::Struct(class) = class
                    && let [
                        Value::String(name),
                        _,
                        _,
                        Value::ValueArray(ValueArray::Int(devices)),
                    ] = class.as_slice()
                {
                    return Some(PwMediaClass {
                        name: name.to_string(),
                        devices: devices.clone(),
                    });
                }

                None
            })
            .collect();

        return result;
    }

    result
}

fn extract_info(info: &DeviceInfoRef) -> Option<PwDevice> {
    let mut device = PwDevice::default();

    if let Some(props) = info.props() {
        if let Some(id) = props.get("object.id") {
            device.id = id.parse().unwrap();
        } else {
            return None;
        }

        if let Some(name) = props.get("device.name") {
            device.name = name.to_string();
        }

        if let Some(nick) = props.get("device.nick") {
            device.nick = nick.to_string();
        }

        if let Some(desc) = props.get("device.description") {
            device.description = desc.to_string();
        }

        if let Some(name) = props.get("alsa.card_name") {
            device.card_name = name.to_string();
        }

        if let Some(name) = props.get("alsa.mixer_name") {
            device.mixer_name = name.to_string();
        }

        if let Some(name) = props.get("device.icon-name") {
            device.icon_name = name.to_string();
        }

        if let Some(id) = props.get("client.id") {
            device.client_id = id.parse().unwrap();
        }

        return Some(device);
    }

    None
}

fn extract_route(id: u32, pod: Option<&Pod>) -> Option<PwDeviceRoute> {
    if let Some(param) = deserialize(pod) {
        let mut route = PwDeviceRoute {
            device_id: id,
            ..Default::default()
        };

        for prop in param.properties {
            match prop.key {
                libspa_sys::SPA_PARAM_ROUTE_index => {
                    if let Value::Int(value) = prop.value {
                        route.index = value;
                    }
                }
                libspa_sys::SPA_PARAM_ROUTE_direction => {
                    if let Value::Id(value) = prop.value {
                        route.direction = match value {
                            libspa::utils::Id(0) => PwDeviceRouteDirection::Input,
                            _ => PwDeviceRouteDirection::Output,
                        };
                    }
                }
                libspa_sys::SPA_PARAM_ROUTE_name => {
                    if let Value::String(value) = prop.value {
                        route.name = value;
                    }
                }
                libspa_sys::SPA_PARAM_ROUTE_description => {
                    if let Value::String(value) = prop.value {
                        route.description = value;
                    }
                }
                libspa_sys::SPA_PARAM_ROUTE_priority => {
                    if let Value::Int(value) = prop.value {
                        route.priority = value;
                    }
                }
                libspa_sys::SPA_PARAM_ROUTE_available => {
                    if let Value::Id(libspa::utils::Id(value)) = prop.value {
                        route.available = value != libspa_sys::SPA_PARAM_AVAILABILITY_no;
                    }
                }
                libspa_sys::SPA_PARAM_ROUTE_profiles => {
                    if let Value::ValueArray(ValueArray::Int(value)) = prop.value {
                        route.profiles = value;
                    }
                }
                libspa_sys::SPA_PARAM_ROUTE_devices => {
                    if let Value::ValueArray(ValueArray::Int(value)) = prop.value {
                        route.devices = value;
                    }
                }
                libspa_sys::SPA_PARAM_ROUTE_props => {
                    if let Value::Object(value) = prop.value {
                        for prop in value.properties {
                            match prop.key {
                                libspa_sys::SPA_PROP_channelVolumes => {
                                    if let Value::ValueArray(ValueArray::Float(value)) = prop.value
                                    {
                                        if value.len() == 2 {
                                            route.volume = (pw2ui(value[0]), pw2ui(value[1]));
                                        } else {
                                            let volume = pw2ui(value[0]);
                                            route.volume = (volume, volume);
                                        }
                                    }
                                }
                                libspa_sys::SPA_PROP_mute => {
                                    if let Value::Bool(value) = prop.value {
                                        route.mute = value;
                                    }
                                }
                                _ => {
                                    // println!("LEFTT {:?} {:?}", prop.key, prop.value);
                                }
                            }
                        }
                    }
                }
                _ => {
                    // println!("LEFTT {:?} {:?}", prop.key, prop.value);
                }
            }
        }

        return Some(route);
    }

    None
}

fn extract_profile(id: u32, pod: Option<&Pod>) -> Option<PwDeviceProfile> {
    if let Some(param) = deserialize(pod) {
        let mut profile = PwDeviceProfile {
            device_id: id,
            ..Default::default()
        };

        for prop in param.properties {
            match prop.key {
                libspa_sys::SPA_PARAM_PROFILE_index => {
                    if let Value::Int(value) = prop.value {
                        profile.index = value;
                    }
                }
                libspa_sys::SPA_PARAM_PROFILE_description => {
                    if let Value::String(value) = prop.value {
                        profile.description = value;
                    }
                }
                libspa_sys::SPA_PARAM_PROFILE_available => {
                    if let Value::Id(libspa::utils::Id(value)) = prop.value {
                        profile.available = value != libspa_sys::SPA_PARAM_AVAILABILITY_no;
                    }
                }
                libspa_sys::SPA_PARAM_PROFILE_classes => {
                    profile.classes = parse_media_class(prop);
                }
                libspa_sys::SPA_PARAM_PROFILE_name => {
                    if let Value::String(value) = prop.value {
                        profile.name = value;
                    }
                }
                libspa_sys::SPA_PARAM_PROFILE_priority => {
                    if let Value::Int(value) = prop.value {
                        profile.priority = value;
                    }
                }
                _ => {}
            }
        }

        return Some(profile);
    }

    None
}

pub fn handle_pipewire_device(
    global: &GlobalObject<&DictRef>,
    registry: &Registry,
    event_sender: Sender<PipewireEvent>,
) -> (Device, DeviceListener) {
    let proxy = registry.bind::<Device, _>(global).unwrap();

    let id = global.id;
    let params = [
        ParamType::EnumRoute,
        ParamType::Route,
        ParamType::Profile,
        ParamType::EnumProfile,
    ];

    let listener = proxy
        .add_listener_local()
        .info({
            let sender = event_sender.clone();
            // let device = { Rc::clone(&proxy) };
            move |info| {
                if let Some(node) = extract_info(info) {
                    sender.send(PipewireEvent::Device(node)).unwrap();
                }

                // for param in params {
                //     device.enum_params(0, Some(param), 0, u32::MAX);
                // }
            }
        })
        .param({
            let sender = event_sender.clone();

            move |_seq, param_type, _p3, _p4, p5| {
                match param_type {
                    ParamType::EnumProfile => {
                        if let Some(enum_profile) = extract_profile(id, p5) {
                            sender
                                .send(PipewireEvent::DeviceEnumProfile(enum_profile))
                                .unwrap();
                        }
                    }
                    ParamType::EnumRoute => {
                        if let Some(enum_route) = extract_route(id, p5) {
                            sender
                                .send(PipewireEvent::DeviceEnumRoute(enum_route))
                                .unwrap();
                        }
                    }
                    ParamType::Profile => {
                        if let Some(profile) = extract_profile(id, p5) {
                            sender.send(PipewireEvent::DeviceProfile(profile)).unwrap();
                        }
                    }
                    ParamType::Route => {
                        if let Some(route) = extract_route(id, p5) {
                            sender.send(PipewireEvent::DeviceRoute(route)).unwrap();
                        }
                    }
                    _ => {}
                };
            }
        })
        .register();

    proxy.subscribe_params(&params);

    (proxy, listener)
}
