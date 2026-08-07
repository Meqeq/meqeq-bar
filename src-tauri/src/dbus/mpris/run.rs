use futures::{StreamExt, future::join_all};
use std::{collections::HashMap, sync::Arc};
use tokio::{
    sync::{
        Mutex,
        mpsc::{Receiver, Sender, channel},
    },
    task::JoinHandle,
};
use zbus::{
    Connection,
    fdo::{DBusProxy, NameLostStream, NameOwnerChangedStream},
    zvariant::Optional,
};

use crate::dbus::mpris::{
    commands::PlayerCommand,
    events::PlayerEvent,
    listener::player_listener,
    spec::{media_player2::MediaPlayer2Proxy, player::PlayerProxy},
};

type HandlesMap<'a> = Arc<Mutex<HashMap<String, (JoinHandle<()>, PlayerProxy<'a>)>>>;

async fn handle_commands(command_rx: &mut Receiver<PlayerCommand>, handle_map: &HandlesMap<'_>) {
    while let Some(command) = command_rx.recv().await {
        let map = handle_map.lock().await;
        match command {
            PlayerCommand::Play(name) => {
                if let Some(proxy) = map.get(&name) {
                    proxy.1.play().await.unwrap();
                }
            }
            PlayerCommand::Pause(name) => {
                if let Some(proxy) = map.get(&name) {
                    proxy.1.pause().await.unwrap();
                }
            }
            PlayerCommand::Next(name) => {
                if let Some(proxy) = map.get(&name) {
                    proxy.1.next().await.unwrap();
                }
            }
            PlayerCommand::Prev(name) => {
                if let Some(proxy) = map.get(&name) {
                    proxy.1.previous().await.unwrap();
                }
            }
            PlayerCommand::Seek(name, position) => {
                if let Some(proxy) = map.get(&name) {
                    proxy.1.seek(position).await.unwrap();
                }
            }
            PlayerCommand::Shuffle(name, shuffle) => {
                if let Some(proxy) = map.get(&name) {
                    proxy.1.set_shuffle(shuffle).await.unwrap();
                }
            }
        }
    }
}

async fn handle_disconnect<'a>(
    connection: &'a Connection,
    name_lost_stream: &mut NameOwnerChangedStream,
    event_tx: Sender<PlayerEvent>,
    handle_map: &'a HandlesMap<'a>,
) {
    while let Some(msg) = name_lost_stream.next().await {
        let args = msg.args().unwrap();

        if !args.name.starts_with("org.mpris.MediaPlayer2") {
            continue;
        }

        if args.new_owner.is_some()
            && let Ok((name, handle, proxy)) =
                player_listener(connection, event_tx.clone(), args.name.to_string()).await
        {
            handle_map.lock().await.insert(name, (handle, proxy));
        } else {
            let name = args.name.to_string();

            if let Some((handle, proxy)) = handle_map.lock().await.remove(&name) {
                println!("handdd");
            }

            event_tx.send(PlayerEvent::Disconnect(name)).await.unwrap();
        }

        println!(
            "NAME {:?} {:?} {:?}",
            args.name, args.old_owner, args.new_owner
        );
    }
}

pub fn run_mpris(
    command_rx: &mut Receiver<PlayerCommand>,
) -> (impl Future<Output = ()>, Receiver<PlayerEvent>) {
    let (event_tx, event_rx) = channel::<PlayerEvent>(32);

    let listener = async move {
        let connection = Connection::session().await.unwrap();
        let proxy = DBusProxy::new(&connection).await.unwrap();

        let names = proxy.list_names().await.unwrap();

        let futures: Vec<_> = names
            .iter()
            .filter(|name| name.starts_with("org.mpris.MediaPlayer2"))
            .map(|name| player_listener(&connection, event_tx.clone(), name.to_string()))
            .collect();

        let kek = join_all(futures).await;

        // let mut iterator = kek.iter();

        // while let Some(Ok((name, handle, proxy))) = iterator.next() {
        //     println!("HHH");
        //     handle_map.lock().await.insert(name, (handle, proxy));
        // }

        let handle_map: HandlesMap = Arc::new(Mutex::new(
            kek.into_iter()
                .filter_map(|maybe_value| {
                    maybe_value.map(|value| (value.0, (value.1, value.2))).ok()
                })
                .collect(),
        ));

        let mut name_lost_stream = proxy.receive_name_owner_changed().await.unwrap();

        tokio::join! {
            handle_commands(command_rx, &handle_map),
            handle_disconnect(&connection, &mut name_lost_stream, event_tx.clone(), &handle_map ),
        };

        // if let Some(name) = names.first() {

        //     let proxy = MediaPlayer2Proxy::builder(&connection)
        //         .destination(name.to_string())
        //         .unwrap()
        //         .build()
        //         .await
        //         .unwrap();

        //     let proxy2 = PlayerProxy::builder(&connection)
        //         .destination(name.to_string())
        //         .unwrap()
        //         .build()
        //         .await
        //         .unwrap();

        //     handle_map.insert(name.to_string(), (handle, proxy));

        //     println!("PROXY {:?}", proxy2);

        //     // proxy2.stop().await.unwrap();

        //     println!("EE: {:?}", proxy2.metadata().await);
        //     println!("PLAYBACK: {:?}", proxy2.playback_status().await);

        println!("NAMES: {:?}", names);
        // names.iter().for_each(|name| {
        //     if name.starts_with("org.mpris.MediaPlayer2") {

        //         println!("NAME: {:?}", name);
        //     }
        // })
    };

    (listener, event_rx)
}
