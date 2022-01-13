#[macro_use] extern crate lazy_static;
#[macro_use] extern crate strum_macros;

// use std::{
//     time::Instant
// };
use log4rs;
// use log::{
//     info
// };
use reqwest::{
    Error
};

mod config;
mod types;
mod repositories;

// use config::CONFIG;
// use repositories::{
//     kraken
// };
// use types::{
//     kraken::{
//         tradeable_pairs::{
//             PairResponse,
//             PairResult,
//             Pair
//         }
//         ,
//         ticker::{
//             RequestPair as TickerPair,
//             Pair as TickerResponsePair,
//             PairResponse as TickerResponse,
//             PairResult as TickerResult
//         }
//     }
// };

use repositories::{
    kraken_ws
};

// https://demo-futures.kraken.com/futures/PI_XBTUSD
// https://docs.kraken.com/rest/

#[tokio::main]
async fn main() -> Result<(), Error> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    // println!("email: {}, password: {}, base url: {}", CONFIG.email, CONFIG.password, CONFIG.base_url);

    // let trade_task = tokio::spawn(async move {
    //     let now = Instant::now();
    //     let pair = Pair::XXBTZUSD;

    //     let pair_info: PairResponse = kraken::get_pair_info(&pair).await.unwrap();
    //     let bit_usd: &PairResult = pair_info.result.get(&pair).unwrap();
    //     info!("{:?}", bit_usd);
    //     let elapsed = now.elapsed();
    //     info!("time elapsed for trade: {}", elapsed.as_millis());
    // });

    // let ticker_task = tokio::spawn(async move {
    //     let now = Instant::now();
    //     let ticker_pair = TickerPair::XBTUSD;
    //     let ticker_response_pair = TickerResponsePair::XXBTZUSD;

    //     let ticker_pair_info: TickerResponse = kraken::get_ticker_info(&ticker_pair).await.unwrap();
    //     let ticker_bit_usd: &TickerResult = ticker_pair_info.result.get(&ticker_response_pair).unwrap();
    //     info!("{:?}", ticker_bit_usd);
    //     let elapsed = now.elapsed();
    //     info!("time elapsed for ticker: {}", elapsed.as_millis());
    // });

    // let (trade_task, ticker_task) = tokio::join!(trade_task, ticker_task);
    // info!("{:?}", &trade_task.unwrap());
    // info!("{:?}", &ticker_task.unwrap());
    kraken_ws::open_connection().await;
    
    Ok(())
}
