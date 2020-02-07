use reqwest::{Url, Client};
use crate::param::{
    Parameters, 
};
use crate::builder::ParamBuilder;
use crate::types::*;

#[derive(Clone)]
pub struct GeneralClient {
    pub(super) url: Url,
    pub(super) client: Client,
}

impl GeneralClient {
    pub fn connect<U: Into<String>>(url: U) -> crate::error::Result<Self> {
        Ok(Self {
            url: url.into().parse::<Url>()?,
            client: Client::new()
        })
    }

    pub fn ping(&self) -> ParamBuilder<'_, '_, PingParams>{
        let Self { url, client } = self;
        let url = url.join("/api/v3/ping").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            None,
            None
        )
    }

    pub fn get_server_time(&self) -> ParamBuilder<'_, '_, TimeParams>{
        let Self { url, client } = self;
        let url = url.join("/api/v3/time").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            None,
            None
        )
    }

    pub fn get_exchange_info(&self) -> ParamBuilder<'_, '_, ExchangeInfoParams>{
        let Self { url, client } = self;
        let url = url.join("/api/v3/exchangeInfo").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            None,
            None
        )
    }
}
