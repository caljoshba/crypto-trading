// use reqwest::{
//     Url,
//     Error,
//     Client,
//     header::{
//         HeaderMap
//     }
// };
// use crate::{
//     types::{
//         kraken::{
//             tradeable_pairs::{
//                 Pair as TradePair,
//                 PairResponse as TradeResponse
//             },
//             ticker::{
//                 RequestPair as TickerPair,
//                 PairResponse as TickerResponse
//             }
//         }
//     },
//     config::{
//         CONFIG,
//         UrlKeys
//     }
// };
// use log::{
//     info
// };
// use std::{
//     time::Instant
// };

// pub async fn get_pair_info(pair: &TradePair) -> Result<TradeResponse, Error> {
//     let endpoint = CONFIG.urls.get(&UrlKeys::TradePair).unwrap();
//     let url = format!(
//         "{base_url}/{endpoint}?pair={pair}",
//         base_url = CONFIG.base_url,
//         endpoint = endpoint,
//         pair = pair
//     );

//     info!("{}", url.to_string());

//     let url = Url::parse(url.as_str()).unwrap();
//     let client = Client::new();

//     #[allow(unused_mut)]
//     let mut headers  = HeaderMap::new();
//     // headers.insert();
//     let now = Instant::now();

//     let response = client.get(url).headers(headers).send().await?;
//     let elapsed = now.elapsed();
//     info!("time elapsed for trade request: {}", elapsed.as_millis());
//     let pair_response: TradeResponse = response.json().await?;

//     Ok(pair_response)
// }

// pub async fn get_ticker_info(pair: &TickerPair) -> Result<TickerResponse, Error> {
//     let endpoint = CONFIG.urls.get(&UrlKeys::TickerPair).unwrap();
//     let url = format!(
//         "{base_url}/{endpoint}?pair={pair}",
//         base_url = CONFIG.base_url,
//         endpoint = endpoint,
//         pair = pair
//     );

//     info!("{}", url.to_string());

//     let url = Url::parse(url.as_str()).unwrap();
//     let client = Client::new();
//     let now = Instant::now();

//     let response = client.get(url).send().await?;
//     let elapsed = now.elapsed();
//     info!("time elapsed for ticker request: {}", elapsed.as_millis());
//     let ticker_response: TickerResponse = response.json().await?;

//     Ok(ticker_response)
// }