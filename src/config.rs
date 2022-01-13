use std::env;
use std::collections::HashMap;
use crate::types::{
    kraken::{
        ticker::{
            Pair as TickerPair
        },
        tradeable_pairs::{
            Pair as TradePair
        }
    }
};

lazy_static! {
    pub static ref CONFIG: Config = Config::new();
}


#[derive(PartialEq, Eq, Hash)]
pub enum UrlKeys {
    TradePair,
    TickerPair
}

// const EMAIL: &str = "lwu2j7m3@futures-demo.com";
// const PASSWORD: &str = "dv6xxvsg724eck1jajnd";
// const BASE_URL: &str = "https://demo-futures.kraken.com";
pub struct Config {
    pub email: String,
    pub password: String,
    pub base_url: String,
    pub urls: HashMap<UrlKeys, &'static str>
}

impl Config {
    fn new() -> Self {
        Self {
            email: env::var("EMAIL").unwrap_or("lwu2j7m3@futures-demo.com".to_string()).parse().unwrap(),
            password: env::var("PASSWORD").unwrap_or("dv6xxvsg724eck1jajnd".to_string()).parse().unwrap(),
            base_url: env::var("BASE_URL").unwrap_or("https://api.kraken.com".to_string()).parse().unwrap(),
            urls: HashMap::from([
                    (UrlKeys::TradePair, "0/public/AssetPairs"),
                    (UrlKeys::TickerPair, "0/public/Ticker")
            ])
        }
    }
}