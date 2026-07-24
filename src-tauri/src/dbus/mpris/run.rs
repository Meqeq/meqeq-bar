use tokio::sync::mpsc::{Receiver, Sender, channel};
use zbus::{Connection, fdo::DBusProxy, names::OwnedBusName};

pub fn run_mpris() -> (impl Future<Output = ()>, Receiver<String>) {
    let (event_tx, event_rx) = channel::<String>(32);

    let listener = async {
        let connection = Connection::session().await.unwrap();

        let proxy = DBusProxy::new(&connection).await.unwrap();

        let names = proxy.list_names().await.unwrap();

        names.iter().for_each(|name| {
            if name.starts_with("org.mpris.MediaPlayer2") {
                println!("NAME: {:?}", name);
            }
        })
    };

    (listener, event_rx)
}
