use serde::{
    Deserialize,
    Serialize
};

use std::collections::HashMap;

#[derive(EnumString, Display, Debug, Deserialize, Serialize)]
pub enum RequestPair {
    XBTUSD
}

#[derive(EnumString, Display, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Pair {
    XXBTZUSD
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
/// Defines the response from the Kraken API for ticker (value of an asset pair)
pub struct PairResponse {
    /// the error response, [] if no error
    pub error: Vec<String>,
    /// the result, a dictionary of the asset pair as the key and the result as the PairResult object
    pub result: HashMap<Pair, PairResult>
}

// #[derive(Deserialize, Serialize, Default, Debug)]
// #[serde(default)]
// /// The result of a specific asset pair
// pub struct PairResult {
//     /// the most recent ask: [ <price>, <whole lot volume>, <lot volume> ]
//     pub a: Vec<(String, String, String)>,
//     /// the most recent bid: [ <price>, <whole lot volume>, <lot volume> ]
//     pub b: Vec<(String, String, String)>,
//     /// the last trade closed: [ <price>, <lot volume> ]
//     pub c: Vec<(String, String)>,
//     /// the volume: [ <today>, <last 24 hours> ]
//     pub v: Vec<(String, String)>,
//     /// the volume weighted average price: [ <today>, <last 24 hours> ]
//     pub p: Vec<(String, String)>,
//     /// the number of trades: [ <today>, <last 24 hours> ]
//     pub t: Vec<(u32, u32)>,
//     /// the lowest value: [ <today>, <last 24 hours> ]
//     pub l: Vec<(String, String)>,
//     /// the highest value: [ <today>, <last 24 hours> ]
//     pub h: Vec<(String, String)>,
//     /// today's opening price
//     pub o: String,
// }

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
/// The result of a specific asset pair
pub struct PairResult {
    /// the most recent ask: [ <price>, <whole lot volume>, <lot volume> ]
    pub a: Vec<String>,
    /// the most recent bid: [ <price>, <whole lot volume>, <lot volume> ]
    pub b: Vec<String>,
    /// the last trade closed: [ <price>, <lot volume> ]
    pub c: Vec<String>,
    /// the volume: [ <today>, <last 24 hours> ]
    pub v: Vec<String>,
    /// the volume weighted average price: [ <today>, <last 24 hours> ]
    pub p: Vec<String>,
    /// the number of trades: [ <today>, <last 24 hours> ]
    pub t: Vec<u32>,
    /// the lowest value: [ <today>, <last 24 hours> ]
    pub l: Vec<String>,
    /// the highest value: [ <today>, <last 24 hours> ]
    pub h: Vec<String>,
    /// today's opening price
    pub o: String,
}