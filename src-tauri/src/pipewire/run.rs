use std::thread::{self, JoinHandle};

use pipewire::channel::{channel, Receiver, Sender};
use tauri::AppHandle;

// use crate::pipewire::node::ui2pw;

use super::{commands::PwCommand, events::PwEvent, mainloop::pipewire_main_loop};

// enum HandleResult {
//     Node((Arc<Node>, NodeListener)),
//     Metadata((Arc<Metadata>, MetadataListener)),
//     Device((Arc<Device>, DeviceListener)),
//     None,
// }

// fn handle_pipewire_object(
//     global: &GlobalObject<&DictRef>,
//     registry: &Registry,
//     app: Arc<AppHandle>,
// ) -> HandleResult {
//     match global.type_ {
//         ObjectType::Node => {
//             let result = handle_pipewire_node(global, registry, app);
//             HandleResult::Node(result)
//         }

//         ObjectType::Metadata => {
//             return match handle_pipewire_metadata(global, registry, app) {
//                 Some(result) => HandleResult::Metadata(result),
//                 None => HandleResult::None,
//             };
//         }

//         // ObjectType::Device => HandleResult::Device(handle_pipewire_device(global, registry, app)),
//         _ => HandleResult::None,
//     }
// }

// fn pipewire_main_loop(receiver: Receiver<PwMessage>, app: Arc<AppHandle>) {
//     let pw = init();
//     let mainloop = Rc::new(pw.0);
//     let registry = Rc::new(pw.1);

//     let reg = Rc::downgrade(&registry);
//     let ml = Rc::downgrade(&mainloop);

//     let nodes = Rc::new(RefCell::new(HashMap::new()));
//     let metadata = Rc::new(RefCell::new(None::<Arc<Metadata>>));

//     let handle = Arc::clone(&app);

//     let nodes_ref = Rc::clone(&nodes);
//     let metadata_ref = Rc::clone(&metadata);

//     let _listener = registry
//         .add_listener_local()
//         .global(move |global| {
//             match handle_pipewire_object(global, &reg.upgrade().unwrap(), Arc::clone(&handle)) {
//                 HandleResult::Node((node, _listener)) => {
//                     nodes_ref.borrow_mut().insert(global.id, node);

//                     let _ = &ml.upgrade().unwrap().quit();
//                     let _ = &ml.upgrade().unwrap().run();
//                 }
//                 HandleResult::Metadata((m, _listener)) => {
//                     *metadata_ref.borrow_mut() = Some(m);

//                     let _ = &ml.upgrade().unwrap().quit();
//                     let _ = &ml.upgrade().unwrap().run();
//                 }
//                 HandleResult::Device(_) => {
//                     let _ = &ml.upgrade().unwrap().quit();
//                     let _ = &ml.upgrade().unwrap().run();
//                 }

//                 HandleResult::None => {}
//             };
//         })
//         .register();

//     // let nodes_ref = Rc::clone(&nodes);
//     // let metadata_ref = Rc::clone(&metadata);

//     // let _kek = receiver.attach(mainloop.loop_(), move |message| match message {
//     //     PwMessage::SetDefaultSink(sink) => {
//     //         println!("NEW SINK {:?}", sink);

//     //         let b = metadata_ref.borrow();
//     //         let metadata = b.as_ref().unwrap();

//     //         println!("NODES");

//     //         for node in nodes_ref.borrow().iter() {
//     //             println!("{:?}", node);
//     //         }

//     //         metadata.set_property(
//     //             0,
//     //             "default.audio.sink",
//     //             Some("Spa:String:JSON"),
//     //             Some(sink.as_str()),
//     //         )
//     //     }

//     //     PwMessage::SetDefaultSource(source) => {
//     //         println!("NEW SINK {:?}", source);

//     //         let b = metadata_ref.borrow();
//     //         let metadata = b.as_ref().unwrap();

//     //         metadata.set_property(
//     //             0,
//     //             "default.audio.source",
//     //             Some("Spa:String:JSON"),
//     //             Some(source.as_str()),
//     //         )
//     //     }

//     //     PwMessage::SetVolume(id, _volume) => {
//     //         let mut buffer: Vec<u8> = Vec::new();
//     //         let mut builder = Builder::new(&mut buffer);

//     //         let a = vec![0.9f32, 0.9f32];

//     //         let mut kek = [0u8; 8];
//     //         LittleEndian::write_f32_into(&a, &mut kek);

//     //         match builder_add!(
//     //             &mut builder,
//     //             Object(
//     //                 SPA_TYPE_OBJECT_Props,
//     //                 ParamType::Props.as_raw()
//     //             ) {
//     //                 // libspa_sys::SPA_PROP_channelVolumes => Bytes(&kek),
//     //                 libspa_sys::SPA_PROP_mute => Bool(false),

//     //             }
//     //         ) {
//     //             Ok(()) => {}
//     //             Err(err) => {
//     //                 println!("ERRRR: {:?}", err)
//     //             }
//     //         }

//     //         unsafe {
//     //             builder
//     //                 .add_prop(libspa_sys::SPA_PROP_channelVolumes, 0)
//     //                 .unwrap();
//     //             builder.add_array(4, 6, 2, kek.as_ptr().cast()).unwrap();
//     //         }

//     //         let values = PodSerializer::serialize(
//     //             std::io::Cursor::new(Vec::new()),
//     //             &Value::Object(Object {
//     //                 type_: libspa_sys::SPA_TYPE_OBJECT_Props,
//     //                 id: libspa_sys::SPA_PARAM_Props,
//     //                 properties: vec![Property {
//     //                     key: libspa_sys::SPA_PROP_channelVolumes,
//     //                     flags: PropertyFlags::empty(),
//     //                     value: Value::ValueArray(ValueArray::Float(a.clone())),
//     //                 }],
//     //             }),
//     //         )
//     //         .unwrap();

//     //         let nodes = nodes_ref.borrow();

//     //         let node = nodes.get(&id);

//     //         if let Some(node) = node {
//     //             println!("USTAWIANIAE");
//     //             let v = &values.0.into_inner();
//     //             let pod = Pod::from_bytes(v).unwrap();
//     //             node.set_param(ParamType::Props, 0, pod);
//     //         }

//     //         // let c = app.clone();

//     //         // c.run_on_main_thread(move || {
//     //         //     let state = app.state::<Mutex<AppState>>();
//     //         //     let state = state.lock().unwrap();

//     //         //     let node = state.get_pw_node(id);
//     //         //     if let Some(node) = node {
//     //         //         node.set_param(ParamType::Props, 0, Pod::from_bytes(&buffer).unwrap());
//     //         //     }
//     //         // })
//     //         // .unwrap();
//     //     }

//     //     _ => {}
//     // });

//     mainloop.run();
// }

pub fn run_pipewire(app: AppHandle) -> (Sender<PwCommand>, JoinHandle<()>) {
    let (command_tx, command_rx): (Sender<PwCommand>, Receiver<PwCommand>) = channel();

    (
        command_tx,
        thread::spawn(move || pipewire_main_loop(command_rx, app)),
    )
}
