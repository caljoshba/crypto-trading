#![allow(unused_variables, dead_code)]
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
            response::PairResult,
        },
    }
};
use std::sync::Mutex;
use std::rc::{ Rc };
use std::cell::RefCell;
use dataframe::{
    frame::DataFrame,
    cell::types::datatypes::AnyType,
    column::RollingMean,
};
use plotlib::page::Page;
use plotlib::repr::Plot;
use plotlib::view::ContinuousView;
use plotlib::style::{PointMarker, PointStyle};

#[derive(Serialize, Debug, Clone)]
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
    let column_names: Vec<&str> = PairResult::column_names();
    let dataframe = Rc::new(Mutex::new(RefCell::new(DataFrame::new(column_names.clone()))));
    dataframe.lock().unwrap().borrow_mut().create_returns_for_column("bid_price", "bid_price_returns", RollingMean::new(true, Some(10)));
    let dataframe2 = Rc::new(Mutex::new(RefCell::new(DataFrame::new(column_names.clone()))));
    dataframe2.lock().unwrap().borrow_mut().create_returns_for_column("bid_price", "bid_price_returns", RollingMean::new(true, Some(15)));
    let dataframe3 = Rc::new(Mutex::new(RefCell::new(DataFrame::new(column_names))));
    dataframe3.lock().unwrap().borrow_mut().create_returns_for_column("bid_price", "bid_price_returns", RollingMean::new(true, Some(20)));

    let url = Url::parse("wss://ws.kraken.com").unwrap(); // Get the URL
    let (ws_stream, _) = connect_async(url).await.expect("Failed to connect to the websocket"); // Connect to the server
    let (mut write, read) = ws_stream.split();

    let subscription = Subscription::new("ticker");

    let payload = EventMessage::new("subscribe", vec!("XBT/EUR"), subscription);
    let unsubscribe_payload = EventMessage::new("unsubscribe", vec!("XBT/EUR"), subscription);

    write.send(Message::text(serde_json::to_string(&payload).unwrap())).await.unwrap();

    // https://docs.rs/futures/0.1.23/futures/stream/trait.Stream.html#method.fold
    // https://stackoverflow.com/questions/62557219/error-on-future-generator-closure-captured-variable-cannot-escape-fnmut-closu

    let read_future = read.fold(vec![Rc::clone(&dataframe), Rc::clone(&dataframe2), Rc::clone(&dataframe3)], |acc, message| async {
        let data = message.unwrap();
        let value: Option<PairResult> = process_event(data);
        if let Some(pair) = value {
            let row_values: Vec<AnyType> = pair.values_as_vec();
            println!("{:?}", row_values);
            let row_index: usize = acc[0].lock().unwrap().borrow_mut().add_row(row_values.clone());
            acc[1].lock().unwrap().borrow_mut().add_row(row_values.clone());
            acc[2].lock().unwrap().borrow_mut().add_row(row_values.clone());
            analyse_row_added(row_index, Rc::clone(&acc[0]), "./output/scatter_10.svg");
            analyse_row_added(row_index, Rc::clone(&acc[1]), "./output/scatter_15.svg");
            analyse_row_added(row_index, Rc::clone(&acc[2]), "./output/scatter_20.svg");
        }
        acc
    });
    
    // write.send(Message::text(serde_json::to_string(&unsubscribe_payload).unwrap())).await.unwrap();
    read_future.await;
    true
}

fn process_event(message: Message) -> Option<PairResult> {
    // println!("{}", message);
    let response: ResponseTypes = serde_json::from_str(message.to_text().unwrap()).unwrap();

    match response {
        ResponseTypes::StatusResponse(status) => { process_status_event(status) },
        ResponseTypes::SubscriptionStatusResponse(status) => { process_subscription_event(status) },
        ResponseTypes::TickerResponse(ticker_response) => { process_ticker_event(ticker_response) },
        ResponseTypes::Heartbeat(heartbeat) => { process_heartbeat_event(heartbeat) }
    }
}

fn process_status_event(status: StatusResponse) -> Option<PairResult> {
    // println!("{:?}", status);
    None
}

fn process_subscription_event(status: SubscriptionStatusResponse) -> Option<PairResult> {
    // println!("{:?}", status);
    None
}

fn process_ticker_event(response: TickerResponse) -> Option<PairResult> {
    // println!("{:?}", response);
    Some(response.ticker)
}

fn process_heartbeat_event(heartbeat: Heartbeat) -> Option<PairResult> {
    // println!("{:?}", heartbeat);
    None
}

fn analyse_row_added(row_index: usize, dataframe: Rc<Mutex<RefCell<DataFrame>>>, scatter_output: &'static str) {
    let row = Rc::clone(&dataframe.lock().unwrap().borrow().get_rows()[row_index]);
    let row_index: usize = row.borrow().index;
    println!("{}", &row_index);

    if row_index > 99 && row_index % 100 == 0 {
        let (return_values, rate_of_change): (Vec<(f64, f64)>, Vec<(f64, f64)>) = dataframe.lock().unwrap().borrow().get_rolling_means_as_vec_with_unix_datetime_diff::<f64>("bid_price_returns", 5);
        let plot = Plot::new(return_values).point_style(
            PointStyle::new()
                .marker(PointMarker::Cross) // setting the marker to be a square
                .colour("#DD3355"),
        );
        let plot_roc = Plot::new(rate_of_change).point_style(
            PointStyle::new()
                .marker(PointMarker::Circle) // setting the marker to be a circle
                .colour("#49FF33"),
        );
        let view = ContinuousView::new()
            .add(plot)
            .add(plot_roc)
            .y_range(-15., 15.)
            .x_label("seconds")
            .y_label("returns");
        Page::single(&view).save(scatter_output).unwrap();
    }  
}