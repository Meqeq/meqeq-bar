use libspa::{
    param::ParamType,
    pod::{serialize::PodSerializer, Object, Pod, Property, PropertyFlags, Value},
};
use pipewire::device::Device;

pub fn device_set_route_properties(
    device: &Device,
    route_index: u32,
    route_device: u32,
    properties: Vec<Property>,
) {
    let mut route_properties = Vec::new();

    route_properties.push(Property {
        key: libspa_sys::SPA_PARAM_ROUTE_index,
        flags: PropertyFlags::empty(),
        value: Value::Int(route_index.try_into().unwrap()),
    });

    route_properties.push(Property {
        key: libspa_sys::SPA_PARAM_ROUTE_device,
        flags: PropertyFlags::empty(),
        value: Value::Int(route_device.try_into().unwrap()),
    });

    if !properties.is_empty() {
        route_properties.push(Property {
            key: libspa_sys::SPA_PARAM_ROUTE_props,
            flags: PropertyFlags::empty(),
            value: Value::Object(Object {
                type_: libspa_sys::SPA_TYPE_OBJECT_Props,
                id: libspa_sys::SPA_PARAM_Route,
                properties,
            }),
        });
    }

    route_properties.push(Property {
        key: libspa_sys::SPA_PARAM_ROUTE_save,
        flags: PropertyFlags::empty(),
        value: Value::Bool(true),
    });

    let values = PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &Value::Object(Object {
            type_: libspa_sys::SPA_TYPE_OBJECT_ParamRoute,
            id: libspa_sys::SPA_PARAM_Route,
            properties: route_properties,
        }),
    );

    if let Ok((values, _)) = values {
        if let Some(pod) = Pod::from_bytes(&values.into_inner()) {
            device.set_param(ParamType::Route, 0, pod);
        }
    }
}

pub fn device_set_profile(device: &Device, profile_index: u32) {
    let properties = vec![
        Property {
            key: libspa_sys::SPA_PARAM_PROFILE_index,
            flags: PropertyFlags::empty(),
            value: Value::Int(profile_index.try_into().unwrap()),
        },
        Property {
            key: libspa_sys::SPA_PARAM_PROFILE_save,
            flags: PropertyFlags::empty(),
            value: Value::Bool(true),
        },
    ];

    let values = PodSerializer::serialize(
        std::io::Cursor::new(Vec::new()),
        &Value::Object(Object {
            type_: libspa_sys::SPA_TYPE_OBJECT_ParamProfile,
            id: libspa_sys::SPA_PARAM_Profile,
            properties,
        }),
    );

    if let Ok((values, _)) = values {
        if let Some(pod) = Pod::from_bytes(&values.into_inner()) {
            device.set_param(ParamType::Profile, 0, pod);
        }
    }
}
