use reqwest::{Url, Client};
use crate::param::{
    Parameters, 
};
use crate::builder::ParamBuilder;
use crate::types::*;

/// Client for dealing with general exchange information
#[derive(Clone)]
pub struct GeneralClient {
    pub(super) url: Url,
    pub(super) client: Client,
}

impl GeneralClient {
    /// Creates new client instance
    /// # Example
    ///
    /// ```no_run
    /// use tokio_binance::{GeneralClient, BINANCE_US_URL};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = GeneralClient::connect(BINANCE_US_URL)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn connect<U: Into<String>>(url: U) -> crate::error::Result<Self> {
        Ok(Self {
            url: url.into().parse::<Url>()?,
            client: Client::new()
        })
    }
    /// Test connectivity to the Rest API.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{GeneralClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = GeneralClient::connect(BINANCE_US_URL)?;
    /// let response = client
    ///     .ping()
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// Test connectivity to the Rest API and get the current server time.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{GeneralClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = GeneralClient::connect(BINANCE_US_URL)?;
    /// let response = client
    ///     .get_server_time()
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
    /// Current exchange trading rules and symbol information.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{GeneralClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = GeneralClient::connect(BINANCE_US_URL)?;
    /// let response = client
    ///     .get_exchange_info()
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
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
