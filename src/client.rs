use reqwest::{Url, Client};
use crate::param::Parameters;
use crate::builder::{ParamBuilder, Auth};
use crate::types::*;

pub struct AccountClient {
    api_key: String,
    secret_key: String,
    url: Url,
    client: Client
}

impl AccountClient {
    pub fn new<T: Into<String>>(api_key: T, secret_key: T, url: T) -> Self {
        Self {
            api_key: api_key.into(), 
            secret_key: secret_key.into(),
            url: url.into().parse::<Url>().expect("Invalid Url"),
            client: Client::new()
        }
    }

    pub fn account(&self) -> ParamBuilder<'_, '_, TradesParams>{
        let Self { api_key, secret_key, url, client } = self;

        let url = url.join("/api/v3/account").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(Auth { api_key, secret_key })
        )
    }

    pub fn general_client(&self) -> GeneralClient {
        GeneralClient {url: self.url.clone(), client: self.client.clone() }
    }

    pub fn market_client(&self) -> MarketDataClient {
        MarketDataClient {url: self.url.clone(), client: self.client.clone() }
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


pub struct GeneralClient {
    url: Url,
    client: Client,
}

impl GeneralClient {
    pub fn new<U: Into<String>>(url: U) -> Self {
        Self {
            url: url.into().parse::<Url>().expect("Invalid Url"),
            client: Client::new()
        }
    }

    pub fn test_connection(&self) -> ParamBuilder<'_, '_, PingParams>{
        let Self { url, client } = self;
        let url = url.join("/api/v3/ping").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            None
        )
    }

    pub fn get_time(&self) -> ParamBuilder<'_, '_, TimeParams>{
        let Self { url, client } = self;
        let url = url.join("/api/v3/time").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            None
        )
    }

    pub fn get_exchange_info(&self) -> ParamBuilder<'_, '_, ExchangeInfoParams>{
        let Self { url, client } = self;
        let url = url.join("/api/v3/exchangeInfo").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            None
        )
    }
}