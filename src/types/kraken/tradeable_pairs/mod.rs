use serde::{
    Deserialize,
    Serialize
};

use std::collections::HashMap;


#[derive(EnumString, Display, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub enum Pair {
    XXBTZUSD
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
pub struct PairResponse {
    pub error: Vec<String>,
    pub result: HashMap<Pair, PairResult>
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
pub struct PairResult {
    pub altname: String,
    pub wsname: String,
    pub aclass_base: String,
    pub base: String,
    pub aclass_quote: String,
    pub quote: String,
    pub lot: String,
    pub pair_decimals: u8,
    pub lot_deciamls: u8,
    pub lot_multiplier: u8,
    pub leverage_buy: Vec<u8>,
    pub leverage_sell: Vec<u8>,
    pub fees: Vec<(u32, f32)>,
    pub fees_maker: Vec<(u32, f32)>,
    pub fee_volume_curreny: String,
    pub margin_call: u8,
    pub margin_stop: u8,
    pub ordermain: String
}