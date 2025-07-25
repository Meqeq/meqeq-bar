use std::rc::Rc;

use libspa::utils::dict::DictRef;
use pipewire::{
    channel::{channel, Receiver, Sender},
    context::Context,
    core::Core,
    main_loop::MainLoop,
    metadata::{Metadata, MetadataListener},
    node::{Node, NodeListener},
    proxy::ProxyListener,
    registry::{GlobalObject, Registry},
    types::ObjectType,
};
use tauri::{AppHandle, Emitter};

use super::{
    commands::PwCommand, events::PwEvent, metadata::handle_pipewire_metadata,
    node::handle_pipewire_node,
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
    None,
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

        // ObjectType::Device => HandleResult::Device(handle_pipewire_device(global, registry, app)),
        _ => HandleResult::None,
    }
}

pub fn pipewire_main_loop(command_receiver: Receiver<PwCommand>, app: AppHandle) {
    let (mainloop, registry, context, core) = init();
    let (event_tx, event_rx): (Sender<PwEvent>, Receiver<PwEvent>) = channel();

    let registry_weak = Rc::downgrade(&registry);
    let mainloop_weak = Rc::downgrade(&mainloop);

    let event_sender = Rc::new(event_tx);

    let _listener = registry
        .add_listener_local()
        .global(move |global| {
            match handle_global(
                global,
                registry_weak.upgrade().unwrap(),
                Rc::clone(&event_sender),
            ) {
                HandleResult::Node(_) => {
                    let ml = mainloop_weak.upgrade().unwrap();
                    ml.quit();
                    ml.run();
                }
                HandleResult::Metadata(_) => {
                    let ml = mainloop_weak.upgrade().unwrap();
                    ml.quit();
                    ml.run();
                }

                _ => {}
            }
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
        _ => {}
    });

    mainloop.run();
}
