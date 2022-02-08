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
        // dataframe::{
        //     // column::Column,
        //     row::Row,
        //     cell::{
        //         Cell,
        //         // SelectRow
        //     },
        // },
        kraken_ws::{
            ResponseTypes,
            StatusResponse,
            SubscriptionStatusResponse,
            TickerResponse,
            Heartbeat,
            // Pair,
            response::PairResult,
        },
        // trades::Trades
    }
};
// use std::sync::Mutex;
use std::rc::{ Rc };
use std::cell::RefCell;
use dataframe::{
    frame::DataFrame,
    cell::types::datatypes::AnyType,
};

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
    // let trades_state: Mutex<Trades> = Mutex::new(Trades::new(Pair::XBTEUR));
    // let row = Rc::new(Row::new(1));
    // let column: Column<f64> = Column::new();
    let column_names: Vec<&str> = PairResult::column_names();
    let dataframe = Rc::new(RefCell::new(DataFrame::new(column_names)));

    let url = Url::parse("wss://ws.kraken.com").unwrap(); // Get the URL
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to the websocket"); // Connect to the server
    let (mut write, read) = ws_stream.split();

    let subscription = Subscription::new("ticker");

    let payload = EventMessage::new("subscribe", vec!("XBT/EUR"), subscription);
    let unsubscribe_payload = EventMessage::new("unsubscribe", vec!("XBT/EUR"), subscription);

    write.send(Message::text(serde_json::to_string(&payload).unwrap())).await.unwrap();

    // https://docs.rs/futures/0.1.23/futures/stream/trait.Stream.html#method.fold
    // https://stackoverflow.com/questions/62557219/error-on-future-generator-closure-captured-variable-cannot-escape-fnmut-closu
    // let read_future = read.for_each(|message| async {
    //     let data = message.unwrap();
    //     let value: Option<PairResult> = process_event(data);
    //     let mut dframe = Rc::clone(&dataframe);
    //     // trades_state.lock().unwrap().append(&value);
    //     // create_cells(value, &column);
    //     // create_cells(value, &row);
    //     if let Some(pair) = value {
    //         let row_values: Vec<AnyType> = pair.values_as_vec();
    //         dframe.add_row(row_values);
    //         // let ask_whole_volume: Rc<Cell<u16, u16>> = Rc::new(Cell::new(Rc::clone(&row), pair.a.ask_whole_volume));
    //         // row.add_cell(&ask_whole_volume);
    //         // let trades_today: Rc<Cell<u16, u32>> = Rc::new(Cell::new(Rc::clone(&row), pair.t.trades_today));
    //         // row.add_cell(&ask_whole_volume);
    //         // row.add_cell(&ask_price);
    //     }
        
    //     // println!("{:?}", &trades_state);
    // });

    let read_future = read.fold(Rc::clone(&dataframe), |acc, message| async {
        let data = message.unwrap();
        let value: Option<PairResult> = process_event(data);
        // trades_state.lock().unwrap().append(&value);
        // create_cells(value, &column);
        // create_cells(value, &row);
        if let Some(pair) = value {
            let row_values: Vec<AnyType> = pair.values_as_vec();
            acc.borrow_mut().add_row(row_values);
            // let ask_whole_volume: Rc<Cell<u16, u16>> = Rc::new(Cell::new(Rc::clone(&row), pair.a.ask_whole_volume));
            // row.add_cell(&ask_whole_volume);
            // let trades_today: Rc<Cell<u16, u32>> = Rc::new(Cell::new(Rc::clone(&row), pair.t.trades_today));
            // row.add_cell(&ask_whole_volume);
            // row.add_cell(&ask_price);
        }
        println!("{:?}", acc);
        acc
    });
    
    write.send(Message::text(serde_json::to_string(&unsubscribe_payload).unwrap())).await.unwrap();
    read_future.await;
    // read_future.;
    true
}

// fn create_cells(data: Option<PairResult>, column: &Column<f64>) {
//     if let Some(pair) = data {
//         // let ask_price: Rc<Cell<f64>> = Rc::new(Cell::<f64>::new(Rc::clone(row), pair.a.ask_price));
//         // row.add_cell(&ask_price);
//     } else {

//     }
// }

// fn create_cells(data: Option<PairResult>, row: &Rc<Row<Cell<u16, u16>>>) {
//     if let Some(pair) = data {
//         let ask_whole_volume: Rc<Cell<u16, u16>> = Rc::new(Cell::new(Rc::clone(row), pair.a.ask_whole_volume));
//         row.add_cell(&ask_whole_volume);
//         let trades_today: Rc<Cell<u16, u32>> = Rc::new(Cell::new(Rc::clone(row), pair.t.trades_today));
//         row.add_cell(&ask_whole_volume);
//         // row.add_cell(&ask_price);
//     } else {

//     }
// }

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