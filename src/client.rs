use reqwest::{Url, Client};
use crate::param::*;
use crate::builder::ParamBuilder;

struct AccountClient {
    api_key: String,
    api_secret: String,
    market_client: MarketDataClient
}

impl AccountClient {
    pub fn new<T: Into<String>, U: Into<Url>>(api_key: T, api_secret: T, url: U) -> Self {
        Self {
            api_key: api_key.into(), 
            api_secret: api_secret.into(),
            market_client: MarketDataClient::new(url)
        }
    }

    pub fn account<'a>(&self) -> ParamBuilder<'_, PingParams<'a>>{
        let MarketDataClient {ref url, ref client} = self.market_client;

        let url = url.join("/api/v3/account").unwrap();

        ParamBuilder::new(
            PingParams::default(),
            client.get(url),
            Some(Auth::new(&self.api_key, &self.api_secret))
        )
    }
}

struct MarketDataClient {
    url: Url,
    client: Client,
}

impl MarketDataClient {
    pub fn new<U: Into<Url>>(url: U) -> Self {
        Self {
            url: url.into(),
            client: Client::new()
        }
    }
}