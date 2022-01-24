use serde::{
    Deserialize,
    de,
    Deserializer
};
use std::str::FromStr;

#[derive(Deserialize, Debug)]
/// The result of a specific asset pair
pub struct PairResult {
    /// the most recent ask: [ <price>, <whole lot volume>, <lot volume> ]
    pub a: Ask,
    /// the most recent bid: [ <price>, <whole lot volume>, <lot volume> ]
    pub b: Bid,
    /// the last trade closed: [ <price>, <lot volume> ]
    pub c: Closed,
    /// the volume: [ <today>, <last 24 hours> ]
    pub v: Volume,
    /// the volume weighted average price: [ <today>, <last 24 hours> ]
    pub p: Price,
    /// the number of trades: [ <today>, <last 24 hours> ]
    pub t: Trades,
    /// the lowest value: [ <today>, <last 24 hours> ]
    pub l: Lowest,
    /// the highest value: [ <today>, <last 24 hours> ]
    pub h: Highest,
    /// today's opening price
    pub o: Opening,
}

fn de_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    f64::from_str(&s).map_err(de::Error::custom)
}

#[derive(Deserialize, Debug)]
pub struct Ask {
    #[serde(deserialize_with = "de_from_str")]
    pub ask_price: f64,
    pub ask_whole_volume: u16,
    #[serde(deserialize_with = "de_from_str")]
    pub ask_volume: f64
}

#[derive(Deserialize, Debug)]
pub struct Bid {
    #[serde(deserialize_with = "de_from_str")]
    pub bid_price: f64,
    pub bid_whole_volume: u16,
    #[serde(deserialize_with = "de_from_str")]
    pub bid_volume: f64
}

#[derive(Deserialize, Debug)]
pub struct Closed {
    #[serde(deserialize_with = "de_from_str")]
    pub closed_price: f64,
    #[serde(deserialize_with = "de_from_str")]
    pub closed_volume: f64
}

#[derive(Deserialize, Debug)]
pub struct Volume {
    #[serde(deserialize_with = "de_from_str")]
    pub volume_today: f64,
    #[serde(deserialize_with = "de_from_str")]
    pub volume_24_hours: f64
}

#[derive(Deserialize, Debug)]
pub struct Price {
    #[serde(deserialize_with = "de_from_str")]
    pub price_today: f64,
    #[serde(deserialize_with = "de_from_str")]
    pub price_24_hours: f64
}

#[derive(Deserialize, Debug)]
pub struct Trades {
    pub trades_today: u32,
    pub trades_24_hours: u32
}

#[derive(Deserialize, Debug)]
pub struct Lowest {
    #[serde(deserialize_with = "de_from_str")]
    pub lowest_today: f64,
    #[serde(deserialize_with = "de_from_str")]
    pub lowest_24_hours: f64
}

#[derive(Deserialize, Debug)]
pub struct Highest {
    #[serde(deserialize_with = "de_from_str")]
    pub highest_today: f64,
    #[serde(deserialize_with = "de_from_str")]
    pub highest_24_hours: f64
}

#[derive(Deserialize, Debug)]
pub struct Opening {
    #[serde(deserialize_with = "de_from_str")]
    pub opening_today: f64,
    #[serde(deserialize_with = "de_from_str")]
    pub opening_yesterday: f64
}