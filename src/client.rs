use reqwest::{Url, Client};
use crate::param::*;
use crate::types::{TradesParams};
use crate::builder::{ParamBuilder, Auth};

pub struct AccountClient {
    api_key: String,
    secret_key: String,
    market_client: MarketDataClient
}

impl AccountClient {
    pub fn new<T: Into<String>>(api_key: T, secret_key: T, url: T) -> Self {
        Self {
            api_key: api_key.into(), 
            secret_key: secret_key.into(),
            market_client: MarketDataClient::new(url)
        }
    }

    pub fn account(&self) -> ParamBuilder<'_, '_, TradesParams>{
        let Self { api_key, secret_key, market_client } = self;
        let MarketDataClient { url, client} = market_client;

        let url = url.join("/api/v3/account").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(Auth { api_key, secret_key })
        )
    }
}

pub struct MarketDataClient {
    url: Url,
    client: Client,
}

impl MarketDataClient {
    pub fn new<U: Into<String>>(url: U) -> Self {
        Self {
            url: url.into().parse::<Url>().expect("Invalid Url"),
            client: Client::new()
        }
    }
}