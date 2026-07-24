use std::{cell::RefCell, collections::HashMap, rc::Rc};

use pipewire::{
    device::{Device, DeviceListener},
    metadata::{Metadata, MetadataListener},
    node::{Node, NodeListener},
    proxy::ProxyListener,
};

// Listeners needs to be stored until main_loop.run is called
#[allow(dead_code)]
pub enum Listener {
    Proxy(ProxyListener),
    Node(NodeListener),
    Metadata(MetadataListener),
    Device(DeviceListener),
}

pub struct ListenerStorage {
    pub nodes: HashMap<u32, Node>,
    pub devices: HashMap<u32, Device>,

    pub metadata: Option<Metadata>,

    pub listeners: Vec<Listener>,
}

impl ListenerStorage {
    pub fn new() -> Rc<RefCell<ListenerStorage>> {
        Rc::new(RefCell::new(ListenerStorage {
            nodes: HashMap::new(),
            devices: HashMap::new(),
            metadata: None,
            listeners: Vec::new(),
        }))
    }
}
