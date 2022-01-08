#[macro_use] extern crate lazy_static;
#[macro_use] extern crate strum_macros;

use log4rs;
use reqwest::{
    Error
};

mod config;
mod types;
mod repositories;

use config::CONFIG;

// https://demo-futures.kraken.com/futures/PI_XBTUSD
// https://docs.kraken.com/rest/

#[tokio::main]
async fn main() -> Result<(), Error> {
    log4rs::init_file("config/log4rs.yaml", Default::default()).unwrap();
    println!("email: {}, password: {}, base url: {}", CONFIG.email, CONFIG.password, CONFIG.base_url);

    let pair = types::kraken::Pairs::BITUSD;

    repositories::kraken::get_pair_info(pair).await?;

    Ok(())
}
