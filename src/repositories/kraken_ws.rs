use tokio_tungstenite::{
    connect_async,
    tungstenite::Message
};
use reqwest::Url;
use futures_util::{
    StreamExt,
    SinkExt
};
use serde::{
    Deserialize,
    Serialize
};

#[derive(Deserialize, Serialize, Debug)]
struct EventMessage<'t> {
    event: &'t str,
    pair: Vec<&'t str>,
    subscription: Subscription<'t>
}

#[derive(Deserialize, Serialize, Debug)]
struct Subscription<'t> {
    name: &'t str
}

impl<'t> EventMessage<'t> {
    fn new(event: &'t str, pair: Vec<&'t str>, subscription: Subscription<'t>) -> Self {
        Self {
            event,
            pair,
            subscription
        }
    }
}

impl<'t> Subscription<'t> {
    fn new(name: &'t str) -> Self {
        Self {
            name
        }
    }
}

pub async fn open_connection() -> bool {
    let url = Url::parse("wss://ws.kraken.com").unwrap(); // Get the URL
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to the websocket"); // Connect to the server
    let (mut write, read) = ws_stream.split();

    let pairs_to_subscribe = vec!("XBT/EUR");
    let subscription = Subscription::new("ticker");

    let payload = EventMessage::new("subscribe", pairs_to_subscribe, subscription);

    write.send(Message::text(serde_json::to_string(&payload).unwrap())).await.unwrap();

    let read_future = read.for_each(|message| async {
        let data = message.unwrap();
        println!("{}", data);
    });

    read_future.await;
    true
}