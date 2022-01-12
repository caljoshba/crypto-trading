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
            PairsResponse,
            PairResult,
            Pairs
        }
    }
};

// https://demo-futures.kraken.com/futures/PI_XBTUSD
// https://docs.kraken.com/rest/

#[tokio::main]
async fn main() -> Result<(), Error> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    println!("email: {}, password: {}, base url: {}", CONFIG.email, CONFIG.password, CONFIG.base_url);

    let pair = Pairs::XXBTZUSD;

    let pair_info: PairsResponse = kraken::get_pair_info(&pair).await?;
    let bit_usd: &PairResult = pair_info.result.get(&pair).unwrap();
    info!("{:?}", bit_usd);

    Ok(())
}
