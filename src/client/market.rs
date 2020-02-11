use reqwest::{Url, Client};
use crate::param::{
    Parameters, 
    Interval,
};
use crate::builder::ParamBuilder;
use crate::types::*;

/// Client for dealing with market data.
#[derive(Clone)]
pub struct MarketDataClient {
    pub(super) api_key: String,
    pub(super) url: Url,
    pub(super) client: Client,
}

impl MarketDataClient {
    /// Creates new client instance
    /// # Example
    ///
    /// ```no_run
    /// use tokio_binance::{MarketDataClient, BINANCE_US_URL};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = MarketDataClient::connect("<api-key>", BINANCE_US_URL)?;
    ///     Ok(())
    /// }
    /// ```
    pub fn connect<A, U>(api_key: A, url: U) -> crate::error::Result<Self> 
    where
        A: Into<String>,
        U: Into<String>
    {
        Ok(Self {
            api_key: api_key.into(),
            url: url.into().parse::<Url>()?,
            client: Client::new()
        })
    }
    /// Get order book.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{MarketDataClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = MarketDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_order_book("BNBUSDT")
    ///     // optional: default 100; max 5000.
    ///     .with_limit(5)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_order_book<'a>(&self, symbol: &'a str) -> ParamBuilder<'a, '_, OrderBookParams>{
        let Self { ref api_key, url, client } = self;
        let url = url.join("/api/v3/depth").unwrap();

        ParamBuilder::new(
            Parameters { symbol: Some(symbol), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            None
        )
    }
    /// Get recent trades (up to last 500).
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{MarketDataClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = MarketDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_trades("BNBUSDT")
    ///     // optional: default 100; max 5000.
    ///     .with_limit(5)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_trades<'a>(&self, symbol: &'a str) -> ParamBuilder<'a, '_, TradesParams>{
        let Self { ref api_key, url, client } = self;
        let url = url.join("/api/v3/trades").unwrap();

        ParamBuilder::new(
            Parameters { symbol: Some(symbol), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            None
        )
    }
    /// Get older trades.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{MarketDataClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = MarketDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_historical_trades("BNBUSDT")
    ///     // optional: trade id to fetch from; default gets most recent trades.
    ///     .with_from_id(123049)
    ///     // optional: default 100; max 5000.
    ///     .with_limit(5)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_historical_trades<'a>(&self, symbol: &'a str) -> ParamBuilder<'a, '_, HistoricalTradesParams>{
        let Self { ref api_key, url, client } = self;
        let url = url.join("/api/v3/historicalTrades").unwrap();

        ParamBuilder::new(
            Parameters { symbol: Some(symbol), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            None
        )
    }
    /// Get compressed, aggregate trades. 
    /// Trades that fill at the time, from the same order, 
    /// with the same price will have the quantity aggregated.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{MarketDataClient, BINANCE_US_URL};
    /// use chrono::{Utc, Duration};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = MarketDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let end = Utc::now();
    /// let start = end - Duration::minutes(59);
    /// 
    /// let response = client
    ///     .get_aggregate_trades("BNBUSDT")
    ///     // optional: filter by orders greater than or equal to the provided id.
    ///     .with_from_id(1230494)
    ///     // optional: get agg trades from; pass 60 minutes of agg trades is the default.
    ///     .with_start_time(start)
    ///     // optional: get agg trades until; default is now.
    ///     .with_end_time(end)
    ///     // optional: limit the amount of agg trades; default 500; max 1000.
    ///     .with_limit(100)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_aggregate_trades<'a>(&self, symbol: &'a str) -> ParamBuilder<'a, '_, AggTradesParams>{
        let Self { ref api_key, url, client } = self;
        let url = url.join("/api/v3/aggTrades").unwrap();

        ParamBuilder::new(
            Parameters { symbol: Some(symbol), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            None
        )
    }
    /// Kline/candlestick bars for a symbol. Klines are uniquely identified by their open time.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{MarketDataClient, BINANCE_US_URL};
    /// use tokio_binance::Interval;
    /// use chrono::{Utc, Duration};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = MarketDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let end = Utc::now();
    /// let start = end - Duration::minutes(499);
    /// 
    /// let response = client
    ///     .get_candlestick_bars("BNBUSDT", Interval::OneMinute)
    ///     // optional: get klines from; gets all recent klines by default
    ///     .with_start_time(start)
    ///     // optional: get klines until; default is now.
    ///     .with_end_time(end)
    ///     // optional: limit the amount of klines; default 500; max 1000.
    ///     .with_limit(100)
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_candlestick_bars<'a>(&self, symbol: &'a str, interval: Interval) -> ParamBuilder<'a, '_, KlinesParams>{
        let Self { ref api_key, url, client } = self;
        let url = url.join("/api/v3/klines").unwrap();

        ParamBuilder::new(
            Parameters { symbol: Some(symbol), interval: Some(interval), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            None
        )
    }
    /// Current average price for a symbol.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{MarketDataClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = MarketDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_average_price("BNBUSDT")
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_average_price<'a>(&self, symbol: &'a str) -> ParamBuilder<'a, '_, AveragePriceParams>{
        let Self { ref api_key, url, client } = self;
        let url = url.join("/api/v3/avgPrice").unwrap();

        ParamBuilder::new(
            Parameters { symbol: Some(symbol), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            None
        )
    }
    /// 24 hour rolling window price change statistics. 
    /// Careful when accessing this with no symbol.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{MarketDataClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = MarketDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_24hr_ticker_price()
    ///     // optional: filter by symbol; gets all symbols by default.
    ///     .with_symbol("BNBUSDT")
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_24hr_ticker_price<'a>(&self) -> ParamBuilder<'a, '_, TwentyfourHourTickerPriceParams>{
        let Self { ref api_key, url, client } = self;
        let url = url.join("/api/v3/ticker/24hr").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            None
        )
    }
    /// Latest price for a symbol or symbols.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{MarketDataClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = MarketDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_price_ticker()
    ///     // optional: filter by symbol; gets all symbols by default.
    ///     .with_symbol("BNBUSDT")
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_price_ticker<'a>(&self) -> ParamBuilder<'a, '_, TickerPriceParams>{
        let Self { ref api_key, url, client } = self;
        let url = url.join("/api/v3/ticker/price").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            None
        )
    }
    /// Best price/qty on the order book for a symbol or symbols.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{MarketDataClient, BINANCE_US_URL};
    /// use serde_json::Value;
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = MarketDataClient::connect("<api-key>", BINANCE_US_URL)?;
    /// let response = client
    ///     .get_order_book_ticker()
    ///     // optional: filter by symbol; gets all symbols by default.
    ///     .with_symbol("BNBUSDT")
    ///     //
    ///     .json::<Value>()
    ///     .await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_order_book_ticker<'a>(&self) -> ParamBuilder<'a, '_, OrderBookTickerParams>{
        let Self { ref api_key, url, client } = self;
        let url = url.join("/api/v3/ticker/bookTicker").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            None
        )
    }
}