use reqwest::{Url, Client};
use crate::param::Parameters;
use crate::builder::ParamBuilder;
use crate::types::*;

#[derive(Clone)]
pub struct UserDataClient {
    api_key: String,
    url: Url,
    client: Client
}

impl UserDataClient {
    pub fn connect<T: Into<String>>(api_key: T, url: T) -> crate::error::Result<Self> {
        Ok(Self {
            api_key: api_key.into(), 
            url: url.into().parse::<Url>()?,
            client: Client::new()
        })
    }

    pub fn start_stream(&self) -> ParamBuilder<'_, '_, StartStreamParams>{
        let Self { api_key, url, client } = self;
        let url = url.join("/api/v3/userDataStream").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.post(url),
            Some(api_key),
            None
        )
    }

    pub fn keep_alive<'a>(&self, listen_key: &'a str) -> ParamBuilder<'a, '_, KeepAliveStreamParams>{
        let Self { api_key,  url, client } = self;
        let url = url.join("/api/v3/userDataStream").unwrap();

        ParamBuilder::new(
            Parameters { listen_key: Some(listen_key), ..Parameters::default() },
            client.put(url),
            Some(api_key),
            None
        )
    }

    pub fn close_stream<'a>(&self, listen_key: &'a str) -> ParamBuilder<'a, '_, CloseStreamParams>{
        let Self { api_key, url, client } = self;
        let url = url.join("/api/v3/userDataStream").unwrap();

        ParamBuilder::new(
            Parameters { listen_key: Some(listen_key), ..Parameters::default() },
            client.delete(url),
            Some(api_key),
            None
        )
    }
}