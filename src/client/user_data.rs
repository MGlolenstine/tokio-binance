use reqwest::{Url, Client};
use crate::param::Parameters;
use crate::builder::ParamBuilder;
use crate::types::*;

/// Client for dealing with the user data stream.
#[derive(Clone)]
pub struct UserDataClient {
    api_key: String,
    url: Url,
    client: Client
}

impl UserDataClient {
    /// Creates new client instance
    /// # Example
    ///
    /// ```no_run
    /// use tokio_binance::{UserDataClient, BINANCE_US_URL};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = UserDataClient::connect("<api-key>", BINANCE_US_URL)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn connect<T: Into<String>>(api_key: T, url: T) -> crate::error::Result<Self> {
        Ok(Self {
            api_key: api_key.into(), 
            url: url.into().parse::<Url>()?,
            client: Client::new()
        })
    }
    /// Start a new user data stream. 
    /// The stream will close after 60 minutes unless a keepalive is sent.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{UserDataClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = UserDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .start_stream()
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// Keepalive a user data stream to prevent a time out. 
    /// User data streams will close after 60 minutes. 
    /// It's recommended to send a ping about every 30 minutes.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{UserDataClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = UserDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .keep_alive("<listen-key>")
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// Close out a user data stream.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{UserDataClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = UserDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .close_stream("<listen-key>")
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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