use reqwest::{Url, Client};
use crate::param::{
    Parameters, 
    OrderType, 
    Side, 
    TimeInForce,
    ID
};
use crate::builder::ParamBuilder;
use crate::types::*;
use crate::client::*;

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