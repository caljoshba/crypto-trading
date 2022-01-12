use reqwest::{
    Url,
    Error,
    Client,
    header::{
        HeaderMap
    }
};
use crate::{
    types::{
        kraken::{
            tradeable_pairs::{
                PairsResponse,
                Pairs
            }
        }
    },
    config::CONFIG
};
use log::{
    info
};

pub async fn get_pair_info(pair: &Pairs) -> Result<PairsResponse, Error> {
    let url = format!(
        "{base_url}/0/public/AssetPairs?pair={pair}",
        base_url = CONFIG.base_url,
        pair = pair
    );

    info!("{}", url.to_string());

    let url = Url::parse(url.as_str()).unwrap();
    let client = Client::new();

    #[allow(unused_mut)]
    let mut headers  = HeaderMap::new();
    // headers.insert();

    let response = client.get(url).headers(headers).send().await?;
    let pair_response: PairsResponse = response.json().await?;

    Ok(pair_response)
}