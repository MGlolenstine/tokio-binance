use reqwest::{Url, Client};
use crate::param::{
    Parameters, 
    OrderType, 
    Side, 
    TimeInForce, 
    Interval,
    ID
};
use crate::builder::{ParamBuilder};
use crate::types::*;

#[derive(Clone)]
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

    pub fn place_limit_order<'a>(
        &self, symbol: &'a str, 
        side: Side, 
        price: f64, 
        quantity: f64, 
        execute: bool
    ) -> ParamBuilder<'a, '_, LimitOrderParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = if execute {
            url.join("/api/v3/order").unwrap()
        } else {
            url.join("/api/v3/order/test").unwrap()
        };

        ParamBuilder::new(
            Parameters { 
                symbol: Some(symbol),
                side: Some(side),
                order_type: Some(OrderType::Limit),
                price: Some(price),
                quantity: Some(quantity),
                time_in_force: Some(TimeInForce::Gtc),
                ..Parameters::default() 
            },
            client.post(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn place_market_order<'a>(
        &self, symbol: &'a str, 
        side: Side, 
        quantity: f64, 
        execute: bool
    ) -> ParamBuilder<'a, '_, MarketOrderParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = if execute {
            url.join("/api/v3/order").unwrap()
        } else {
            url.join("/api/v3/order/test").unwrap()
        };

        ParamBuilder::new(
            Parameters { 
                symbol: Some(symbol),
                side: Some(side),
                order_type: Some(OrderType::Market),
                quantity: Some(quantity),
                ..Parameters::default() 
            },
            client.post(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_order_status<'a>(&self, symbol: &'a str, id: ID<'a>) -> ParamBuilder<'a, '_, OrderStatusParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/order").unwrap();

        let order_id = if let ID::OrderId(id) = id {
            Some(id)
        } else {
            None
        };

        let orig_client_order_id = if let ID::OriClientOrderId(id) = id {
            Some(id)
        } else {
            None
        };

        ParamBuilder::new(
            Parameters { 
                symbol: Some(symbol),
                order_id,
                orig_client_order_id,
                ..Parameters::default() 
            },
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_account(&self) -> ParamBuilder<'_, '_, AccountParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/account").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn market_client(&self) -> MarketDataClient {
        MarketDataClient { api_key: self.api_key.clone(), url: self.url.clone(), client: self.client.clone() }
    }

    pub fn general_client(&self) -> GeneralClient {
        GeneralClient { url: self.url.clone(), client: self.client.clone() }
    }

}

#[derive(Clone)]
pub struct MarketDataClient {
    api_key: String,
    url: Url,
    client: Client,
}

impl MarketDataClient {
    pub fn new<T: Into<String>>(api_key: T, url: T) -> Self {
        Self {
            api_key: api_key.into(),
            url: url.into().parse::<Url>().expect("Invalid Url"),
            client: Client::new()
        }
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

    pub fn general_client(&self) -> GeneralClient {
        GeneralClient { url: self.url.clone(), client: self.client.clone() }
    }

    pub fn account_client<T: Into<String>>(&self, secret_key: T) -> AccountClient {
        AccountClient { 
            api_key: self.api_key.clone(), 
            secret_key: secret_key.into(), 
            url: self.url.clone(), 
            client: self.client.clone() 
        }
    }
}

#[derive(Clone)]
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
            None,
            None
        )
    }

    pub fn get_time(&self) -> ParamBuilder<'_, '_, TimeParams>{
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

    pub fn market_client<T: Into<String>>(&self, api_key: T) -> MarketDataClient {
        MarketDataClient { 
            api_key: api_key.into(),
            url: self.url.clone(), 
            client: self.client.clone() 
        }
    }

    pub fn account_client<T: Into<String>>(&self, api_key: T, secret_key: T) -> AccountClient {
        AccountClient { 
            api_key: api_key.into(), 
            secret_key: secret_key.into(), 
            url: self.url.clone(), 
            client: self.client.clone() 
        }
    }
}