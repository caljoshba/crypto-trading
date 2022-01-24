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
    Serialize
};
use crate::{
    types::{
        kraken_ws::{
            ResponseTypes,
            StatusResponse,
            SubscriptionStatusResponse,
            TickerResponse,
            Heartbeat,
            Pair,
            response::PairResult,
        },
        trades::Trades
    }
};
use std::sync::Mutex;

#[derive(Serialize, Debug)]
struct EventMessage<'t> {
    event: &'t str,
    pair: Vec<&'t str>,
    subscription: Subscription<'t>
}

#[derive(Serialize, Debug, Clone, Copy)]
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
    let trades_state: Mutex<Trades> = Mutex::new(Trades::new(Pair::XBTEUR));
    let url = Url::parse("wss://ws.kraken.com").unwrap(); // Get the URL
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to the websocket"); // Connect to the server
    let (mut write, read) = ws_stream.split();

    let subscription = Subscription::new("ticker");

    let payload = EventMessage::new("subscribe", vec!("XBT/EUR"), subscription);
    let unsubscribe_payload = EventMessage::new("unsubscribe", vec!("XBT/EUR"), subscription);

    write.send(Message::text(serde_json::to_string(&payload).unwrap())).await.unwrap();

    let read_future = read.for_each(|message| async {
        let data = message.unwrap();
        let value: Option<PairResult> = process_event(data);
        trades_state.lock().unwrap().append(value);
        println!("{:?}", &trades_state);
    });
    
    write.send(Message::text(serde_json::to_string(&unsubscribe_payload).unwrap())).await.unwrap();

    read_future.await;
    true
}

fn process_event(message: Message) -> Option<PairResult> {
    println!("{}", message);
    let response: ResponseTypes = serde_json::from_str(message.to_text().unwrap()).unwrap();

    match response {
        ResponseTypes::StatusResponse(status) => { process_status_event(status) },
        ResponseTypes::SubscriptionStatusResponse(status) => { process_subscription_event(status) },
        ResponseTypes::TickerResponse(ticker_response) => { process_ticker_event(ticker_response) },
        ResponseTypes::Heartbeat(heartbeat) => { process_heartbeat_event(heartbeat) }
    }
}

fn process_status_event(status: StatusResponse) -> Option<PairResult> {
    println!("{:?}", status);
    None
}

fn process_subscription_event(status: SubscriptionStatusResponse) -> Option<PairResult> {
    println!("{:?}", status);
    None
}

fn process_ticker_event(response: TickerResponse) -> Option<PairResult> {
    println!("{:?}", response);
    Some(response.ticker)
}

fn process_heartbeat_event(heartbeat: Heartbeat) -> Option<PairResult> {
    println!("{:?}", heartbeat);
    None
}