use tokio::time::{self, Duration};
use tokio::{select, spawn, sync::mpsc::Sender, task::JoinHandle};
use tokio_stream::StreamExt;
use zbus::{Connection, Result};

use crate::dbus::mpris::events::PlayerAbility;
use crate::dbus::mpris::{
    events::{LoopStatus, MediaPlayer, PlaybackStatus, PlayerEvent},
    spec::{media_player2::MediaPlayer2Proxy, player::PlayerProxy},
};

pub async fn player_listener<'a>(
    connection: &'a Connection,
    sender: Sender<PlayerEvent>,
    name: String,
) -> Result<(String, JoinHandle<()>, PlayerProxy<'a>)> {
    let proxy = MediaPlayer2Proxy::builder(connection)
        .destination(name.clone())?
        .build()
        .await?;

    let player_proxy = PlayerProxy::builder(connection)
        .destination(name.clone())?
        .build()
        .await?;

    let _ = sender
        .send(PlayerEvent::New(MediaPlayer {
            name: name.clone(),
            identity: proxy.identity().await?,
        }))
        .await;

    let mut play_status_stream = player_proxy.receive_playback_status_changed().await;
    let mut metadata_stream = player_proxy.receive_metadata_changed().await;

    let mut loop_status_stream = player_proxy.receive_loop_status_changed().await;

    let mut shuffle_stream = player_proxy.receive_shuffle_changed().await;

    let mut can_go_next_stream = player_proxy.receive_can_go_next_changed().await;

    let mut can_go_previous_stream = player_proxy.receive_can_go_previous_changed().await;

    let mut can_seek_stream = player_proxy.receive_can_seek_changed().await;

    let mut can_control_stream = player_proxy.receive_can_control_changed().await;

    let mut seeked_stream = player_proxy.receive_seeked().await.unwrap();

    let mut interval = time::interval(Duration::from_millis(1000));

    let name2 = name.clone();

    let proxy2 = player_proxy.clone();

    let join_handle = spawn(async move {
        loop {
            select! {
                Some(property) = play_status_stream.next() => {
                    let status = PlaybackStatus::from_property(property).await;
                    let _ = sender.send( PlayerEvent::PlaybackStatus(name2.clone(), status)).await;
                },

                Some(property) = loop_status_stream.next() => {
                    let status = LoopStatus::from_property(property).await;
                    let _ = sender.send( PlayerEvent::LoopStatus(name2.clone(), status)).await;
                },

                Some(property) = metadata_stream.next() => {
                    let e = property.get().await.unwrap();
                    let _ = sender.send( PlayerEvent::Metadata(name2.clone(), e)).await;
                    let _ = sender.send( PlayerEvent::Position(name2.clone(), 0)).await;
                }

                Some(property) = shuffle_stream.next() => {
                    let _ = sender.send( PlayerEvent::Shuffle(name2.clone(), property.get().await.unwrap())).await;
                }

                Some(property) = can_go_next_stream.next() => {
                    let _ = sender.send( PlayerEvent::Ability(name2.clone(), PlayerAbility::GoNext, property.get().await.unwrap())).await;
                      }

                Some(property) = can_go_previous_stream.next() => {
                    let _ = sender.send( PlayerEvent::Ability(name2.clone(), PlayerAbility::GoPrevious, property.get().await.unwrap())).await;
                }

                Some(property) = can_seek_stream.next() => {
                         let _ = sender.send( PlayerEvent::Ability(name2.clone(), PlayerAbility::Seek, property.get().await.unwrap())).await;
                     }

                Some(property) = can_control_stream.next() => {
                         let _ = sender.send( PlayerEvent::Ability(name2.clone(), PlayerAbility::Control, property.get().await.unwrap())).await;
                }

                _ = seeked_stream.next() => {
                    println!("SEEEKED");
                    let _ = sender.send( PlayerEvent::Position(name2.clone(), proxy2.position().await.unwrap())).await;
                }

                _ = interval.tick() => {
                    let _ = sender.send( PlayerEvent::Position(name2.clone(), proxy2.position().await.unwrap())).await;
                }
            }
        }
    });

    Ok((name, join_handle, player_proxy))
}
