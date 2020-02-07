use reqwest::{Url, Client};
use crate::param::{
    Parameters, 
    Interval,
};
use crate::builder::ParamBuilder;
use crate::types::*;

#[derive(Clone)]
pub struct MarketDataClient {
    pub(super) api_key: String,
    pub(super) url: Url,
    pub(super) client: Client,
}

impl MarketDataClient {
    pub fn connect<T: Into<String>>(api_key: T, url: T) -> crate::error::Result<Self> {
        Ok(Self {
            api_key: api_key.into(),
            url: url.into().parse::<Url>()?,
            client: Client::new()
        })
    }

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