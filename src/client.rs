use reqwest::{Url, Client};
use crate::param::{
    Parameters, 
    OrderType, 
    Side, 
    TimeInForce, 
    Interval,
    ID
};
use crate::builder::ParamBuilder;
use crate::types::*;

/// https://api.binance.us
pub const BINANCE_US_URL: &'static str = "https://api.binance.us";

#[derive(Clone)]
pub struct AccountClient {
    api_key: String,
    secret_key: String,
    url: Url,
    client: Client
}

impl AccountClient {
    pub fn connect<T: Into<String>>(api_key: T, secret_key: T, url: T) -> crate::error::Result<Self> {
        Ok(Self {
            api_key: api_key.into(), 
            secret_key: secret_key.into(),
            url: url.into().parse::<Url>()?,
            client: Client::new()
        })
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

    pub fn get_order<'a>(&self, symbol: &'a str, id: ID<'a>) -> ParamBuilder<'a, '_, OrderStatusParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/order").unwrap();

        let order_id = if let ID::OrderId(id) = id {
            Some(id)
        } else {
            None
        };

        let orig_client_order_id = if let ID::ClientOId(id) = id {
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

    pub fn cancel_order<'a>(&self, symbol: &'a str, id: ID<'a>) -> ParamBuilder<'a, '_, CancelOrderParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/order").unwrap();

        let order_id = if let ID::OrderId(id) = id {
            Some(id)
        } else {
            None
        };

        let orig_client_order_id = if let ID::ClientOId(id) = id {
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
            client.delete(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_open_orders(&self) -> ParamBuilder<'_, '_, OpenOrderParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/openOrders").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_all_orders<'a>(&self, symbol: &'a str) -> ParamBuilder<'a, '_, AllOrdersParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/allOrders").unwrap();

        ParamBuilder::new(
            Parameters { symbol: Some(symbol), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn place_oco_order<'a>(
        &self, symbol: &'a str, 
        side: Side, 
        price: f64,
        stop_price: f64,
        quantity: f64,
    ) -> ParamBuilder<'a, '_, OcoParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/order/oco").unwrap();

        ParamBuilder::new(
            Parameters { 
                symbol: Some(symbol),
                side: Some(side),
                price: Some(price),
                stop_price: Some(stop_price),
                quantity: Some(quantity),
                ..Parameters::default() 
            },
            client.post(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn cancel_oco<'a>(&self, symbol: &'a str, id: ID<'a>) -> ParamBuilder<'a, '_, CancelOcoParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/orderList").unwrap();

        let order_list_id = if let ID::OrderId(id) = id {
            Some(id)
        } else {
            None
        };

        let list_client_order_id = if let ID::ClientOId(id) = id {
            Some(id)
        } else {
            None
        };

        ParamBuilder::new(
            Parameters { 
                symbol: Some(symbol),
                order_list_id,
                list_client_order_id,
                ..Parameters::default() 
            },
            client.delete(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_oco<'a>(&self, id: ID<'a>) -> ParamBuilder<'a, '_, OcoStatusParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/orderList").unwrap();

        let order_list_id = if let ID::OrderId(id) = id {
            Some(id)
        } else {
            None
        };

        let orig_client_order_id = if let ID::ClientOId(id) = id {
            Some(id)
        } else {
            None
        };

        ParamBuilder::new(
            Parameters { 
                order_list_id,
                orig_client_order_id,
                ..Parameters::default() 
            },
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_all_oco_orders(&self) -> ParamBuilder<'_, '_, AllOcoParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/allOrderList").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_open_oco_orders(&self) -> ParamBuilder<'_, '_, OpenOcoParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/openOrderList").unwrap();

        ParamBuilder::new(
            Parameters::default(),
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

    pub fn get_account_trades<'a>(&self, symbol: &'a str) -> ParamBuilder<'a, '_, AccountTradesParams>{
        let Self { ref api_key, ref secret_key, url, client } = self;

        let url = url.join("/api/v3/myTrades").unwrap();

        ParamBuilder::new(
            Parameters { symbol: Some(symbol), ..Parameters::default() },
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

#[derive(Clone)]
pub struct WithdrawClient {
    api_key: String,
    secret_key: String,
    url: Url,
    client: Client
}

impl WithdrawClient {
    pub fn connect<T: Into<String>>(api_key: T, secret_key: T, url: T) -> crate::error::Result<Self> {
        Ok(Self {
            api_key: api_key.into(), 
            secret_key: secret_key.into(),
            url: url.into().parse::<Url>()?,
            client: Client::new()
        })
    }

    pub fn withdraw<'a>(&self, asset: &'a str, address: &'a str, amount: f64) -> ParamBuilder<'a, '_, WithdrawParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/withdraw.html").unwrap();

        ParamBuilder::new(
            Parameters { 
                asset: Some(asset), 
                address: Some(address),
                amount: Some(amount),
                ..Parameters::default() 
            },
            client.post(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn deposit_history(&self) -> ParamBuilder<'_, '_, DepositHistoryParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/depositHistory.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn withdraw_history(&self) -> ParamBuilder<'_, '_, WithdrawHistoryParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/withdrawHistory.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_deposit_address<'a>(&self, asset: &'a str) -> ParamBuilder<'a, '_, DepositAddressParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/depositAddress.html").unwrap();

        ParamBuilder::new(
            Parameters { asset: Some(asset), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_account_status(&self) -> ParamBuilder<'_, '_, AccountStatusParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/accountStatus.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_system_status(&self) -> ParamBuilder<'_, '_, SystemStatusParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/systemStatus.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }
    pub fn get_api_status(&self) -> ParamBuilder<'_, '_, ApiStatusParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/apiTradingStatus.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_dustlog(&self) -> ParamBuilder<'_, '_, DustlogParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/userAssetDribbletLog.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_trade_fee(&self) -> ParamBuilder<'_, '_, TradeFeeParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/tradeFee.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_asset_detail(&self) -> ParamBuilder<'_, '_, AssetDetailsParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/assetDetail.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_sub_accounts(&self) -> ParamBuilder<'_, '_, SubAccountParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/sub-account/list.html").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_sub_account_tranfers<'a>(&self, email: &'a str) -> ParamBuilder<'a, '_, SubAccountTranferParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/sub-account/transfer/history.html").unwrap();

        ParamBuilder::new(
            Parameters { email: Some(email), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn tranfer_sub_account<'a>(&self, 
        from_email: &'a str, 
        to_email: &'a str, 
        asset: &'a str, 
        amount: f64
    ) -> ParamBuilder<'a, '_, TransferSubAccountParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/sub-account/transfer.html").unwrap();

        ParamBuilder::new(
            Parameters { 
                from_email: Some(from_email), 
                to_email: Some(to_email), 
                asset: Some(asset), 
                amount: Some(amount), 
                ..Parameters::default() 
            },
            client.post(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_sub_account_assets<'a>(&self, email: &'a str) -> ParamBuilder<'a, '_, SubAccountAssetParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/wapi/v3/sub-account/assets.html").unwrap();

        ParamBuilder::new(
            Parameters { email: Some(email), ..Parameters::default() },
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn dust_transfer<'a>(&self, asset: &'a str) -> ParamBuilder<'a, '_, DustTransferParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/sapi/v1/asset/dust").unwrap();

        ParamBuilder::new(
            Parameters { asset: Some(asset), ..Parameters::default() },
            client.post(url),
            Some(api_key),
            Some(secret_key)
        )
    }

    pub fn get_asset_dividends(&self) -> ParamBuilder<'_, '_, AssetDividendParams>{
        let Self { api_key, secret_key, url, client } = self;
        let url = url.join("/sapi/v1/asset/assetDividend").unwrap();

        ParamBuilder::new(
            Parameters::default(),
            client.get(url),
            Some(api_key),
            Some(secret_key)
        )
    }
}

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