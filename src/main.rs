#[macro_use] extern crate lazy_static;
#[macro_use] extern crate strum_macros;

use log4rs;
use log::{
    info
};
use reqwest::{
    Error
};

mod config;
mod types;
mod repositories;

use config::CONFIG;
use repositories::{
    kraken
};
use types::{
    kraken::{
        tradeable_pairs::{
            PairResponse,
            PairResult,
            Pair
        }
        ,
        ticker::{
            RequestPair as TickerPair,
            Pair as TickerResponsePair,
            PairResponse as TickerResponse,
            PairResult as TickerResult
        }
    }
};

// https://demo-futures.kraken.com/futures/PI_XBTUSD
// https://docs.kraken.com/rest/

#[tokio::main]
async fn main() -> Result<(), Error> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    println!("email: {}, password: {}, base url: {}", CONFIG.email, CONFIG.password, CONFIG.base_url);

    let pair = Pair::XXBTZUSD;

    let pair_info: PairResponse = kraken::get_pair_info(&pair).await?;
    let bit_usd: &PairResult = pair_info.result.get(&pair).unwrap();
    info!("{:?}", bit_usd);

    let ticker_pair = TickerPair::XBTUSD;
    let ticker_response_pair = TickerResponsePair::XXBTZUSD;

    let ticker_pair_info: TickerResponse = kraken::get_ticker_info(&ticker_pair).await?;
    let ticker_bit_usd: &TickerResult = ticker_pair_info.result.get(&ticker_response_pair).unwrap();
    info!("{:?}", ticker_bit_usd);

    Ok(())
}
