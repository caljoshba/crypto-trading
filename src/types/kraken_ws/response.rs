use serde::{
    Deserialize,
    de,
    Deserializer
};
use std::str::FromStr;
use dataframe::cell::types::datatypes::AnyType;

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

impl PairResult {
    pub fn column_names() -> Vec<&'static str> {
        vec![
            "ask_price",
            "ask_whole_volume",
            "ask_volume",
            "bid_price",
            "bid_whole_volume",
            "bid_volume",
            "closed_price",
            "closed_volume",
            "volume_today",
            "volume_24_hours",
            "price_today",
            "price_24_hours",
            "lowest_today",
            "lowest_24_hours",
            "highest_today",
            "highest_24_hours",
            "opening_today",
            "opening_yesterday"
        ]
    }

    pub fn values_as_vec(&self) -> Vec<AnyType> {
        vec![
            self.a.ask_price.into(),
            self.a.ask_whole_volume.into(),
            self.a.ask_volume.into(),
            self.b.bid_price.into(),
            self.b.bid_whole_volume.into(),
            self.c.closed_price.into(),
            self.c.closed_volume.into(),
            self.v.volume_today.into(),
            self.v.volume_24_hours.into(),
            self.p.price_today.into(),
            self.p.price_24_hours.into(),
            self.l.lowest_today.into(),
            self.l.lowest_24_hours.into(),
            self.h.highest_today.into(),
            self.h.highest_24_hours.into(),
            self.o.opening_today.into(),
            self.o.opening_yesterday.into()
        ]
    }
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