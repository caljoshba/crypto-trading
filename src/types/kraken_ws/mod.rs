pub mod response;

use serde::{
    Deserialize,
    Serialize
};

#[derive(EnumString, Display, Debug, Deserialize, Serialize)]
pub enum Pair {
    #[serde(rename(deserialize = "XBT/EUR"))]
    XBTEUR
}

impl Default for Pair {
    fn default() -> Self { Pair::XBTEUR }
}

#[derive(EnumString, Display, Debug, Deserialize)]
pub enum EventStatus {
    #[serde(rename(deserialize = "subscribed"))]
    SUBSCRIBED,
    #[serde(rename(deserialize = "unsubscribed"))]
    UNSUBSCRIBED,
    #[serde(rename(deserialize = "online"))]
    ONLINE,
    DEFAULT
}

impl Default for EventStatus {
    fn default() -> Self { EventStatus::DEFAULT }
}

#[derive(EnumString, Display, Debug, Deserialize)]
pub enum Event {
    #[serde(rename(deserialize = "systemStatus"))]
    SYSTEM,
    #[serde(rename(deserialize = "subscriptionStatus"))]
    SUBSCRIPTION,
    #[serde(rename(deserialize = "heartbeat"))]
    HEARTBEAT,
    DEFAULT
}

impl Default for Event {
    fn default() -> Self { Event::DEFAULT }
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Subscription {
    name: String
}

#[derive(Deserialize, Debug)]
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

#[derive(Deserialize, Debug)]
pub struct StatusResponse {
    #[serde(rename(deserialize = "connectionID"))]
    pub connection_id: u64,
    pub event: Event,
    pub status: EventStatus,
    pub version: String
}

#[derive(Deserialize, Debug)]
#[serde(expecting = "expecting [<channel_id>, <ticker>, <subscription_name>, <pair>] array")]
pub struct TickerResponse {
    pub channel_id: u16,
    pub ticker: response::PairResult,
    pub subscription_name: String,
    pub pair: Pair,
}

#[derive(Deserialize, Default, Debug)]
#[serde(default)]
pub struct Heartbeat {
    event: Event
}

#[derive(Display, Debug, Deserialize)]
#[serde(untagged)]
pub enum ResponseTypes {
    StatusResponse(StatusResponse),
    SubscriptionStatusResponse(SubscriptionStatusResponse),
    TickerResponse(TickerResponse),
    Heartbeat(Heartbeat)
}