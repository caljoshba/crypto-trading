use serde::{
    Deserialize,
    Serialize
};

#[derive(EnumString, Display, Debug, Deserialize, Serialize)]
pub enum Pair {
    // #[strum(serialize = "XBT/EUR")]
    #[serde(rename(deserialize = "XBT/EUR"))]
    XBTEUR
}

impl Default for Pair {
    fn default() -> Self { Pair::XBTEUR }
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
/// The result of a specific asset pair
pub struct PairResult {
    /// the most recent ask: [ <price>, <whole lot volume>, <lot volume> ]
    pub a: Vec<(String, u16, String)>,
    /// the most recent bid: [ <price>, <whole lot volume>, <lot volume> ]
    pub b: Vec<(String, u16, String)>,
    /// the last trade closed: [ <price>, <lot volume> ]
    pub c: Vec<(String, String)>,
    /// the volume: [ <today>, <last 24 hours> ]
    pub v: Vec<(String, String)>,
    /// the volume weighted average price: [ <today>, <last 24 hours> ]
    pub p: Vec<(String, String)>,
    /// the number of trades: [ <today>, <last 24 hours> ]
    pub t: Vec<(u32, u32)>,
    /// the lowest value: [ <today>, <last 24 hours> ]
    pub l: Vec<(String, String)>,
    /// the highest value: [ <today>, <last 24 hours> ]
    pub h: Vec<(String, String)>,
    /// today's opening price
    pub o: Vec<(String, String)>,
}
// [
//     340,
//     {
//         "a":["37721.40000",0,"0.37966816"],
//         "b":["37720.30000",0,"0.00020249"],
//         "c":["37714.30000","0.00132555"],
//         "v":["1867.76147417","2724.37715040"],
//         "p":["37165.18685","37270.05834"],
//         "t":[19993,34335],
//         "l":["36500.00000","36500.00000"],
//         "h":["37799.90000","38032.90000"],
//         "o":["37155.00000","38012.70000"]
//     },
//     "ticker",
//     "XBT/EUR"
// ]

#[derive(EnumString, Display, Debug, Deserialize, Serialize)]
pub enum EventStatus {
    // #[strum(serialize = "subscribed")]
    #[serde(rename(deserialize = "subscribed"))]
    SUBSCRIBED,
    // #[strum(serialize = "unsubscribed")]
    #[serde(rename(deserialize = "unsubscribed"))]
    UNSUBSCRIBED,
    // #[strum(serialize = "online")]
    #[serde(rename(deserialize = "online"))]
    ONLINE,
    DEFAULT
}

impl Default for EventStatus {
    fn default() -> Self { EventStatus::DEFAULT }
}

#[derive(EnumString, Display, Debug, Deserialize, Serialize)]
pub enum Event {
    // #[strum(serialize = "systemStatus")]
    #[serde(rename(deserialize = "systemStatus"))]
    SYSTEM,
    // #[strum(serialize = "subscriptionStatus")]
    #[serde(rename(deserialize = "subscriptionStatus"))]
    SUBSCRIPTION,
    DEFAULT
}

impl Default for Event {
    fn default() -> Self { Event::DEFAULT }
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
pub struct Subscription {
    name: String
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
pub struct SubscriptionStatusResponse {
    #[serde(rename(deserialize = "channelID"))]
    pub channel_id: u16,
    #[serde(rename(deserialize = "channelName"))]
    pub channel_name: String,
    pub event: Event,
    pub status: EventStatus,
    pub pair: Pair,
    pub subscription: Subscription
}

#[derive(Deserialize, Serialize, Default, Debug)]
#[serde(default)]
pub struct StatusResponse {
    #[serde(rename(deserialize = "connectionID"))]
    pub connection_id: u64,
    pub event: Event,
    pub status: EventStatus,
    pub version: String
}

pub type TickerResponse = Vec<(u16, PairResult, String, Pair)>;

#[derive(Display, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum ResponseTypes {
    StatusResponse(StatusResponse),
    SubscriptionStatusResponse(SubscriptionStatusResponse),
    TickerResponse(TickerResponse)
}