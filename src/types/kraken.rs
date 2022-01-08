use serde::{
    Deserialize,
    Serialize
};

#[derive(EnumString, Display)]
pub enum Pairs {
    #[strum(serialize="XXBTZUSD")]
    BITUSD
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
pub struct PairsResponse {
    pub error: Vec<String>,
    pub result: Result
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
pub struct Result {
    #[allow(non_snake_case)]
    pub XXBTZUSD: PairResult
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