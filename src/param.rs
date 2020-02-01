use serde::Serialize;
use std::ops::{Deref, DerefMut};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use chrono::Utc;
use serde_urlencoded::ser::Error;

type HmacSha256 = Hmac<Sha256>;

#[derive(Copy, Clone)]
pub struct Auth<'a> {
    pub api_key: &'a str,
    pub api_secret: &'a str
}

impl<'a> Auth<'a> {
    pub(super) fn new(api_key: &'a str, api_secret: &'a str) -> Self {
        Self {api_key, api_secret}
    }
}

#[derive(Serialize)]
pub enum Side {
    Buy,
    Sell
}

#[derive(Serialize)]
pub enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker
}

#[derive(Serialize)]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok
}

#[derive(Serialize)]
pub enum OrderRespType {
    Ack,
    Result,
    Full
}

#[derive(Default, Serialize)]
pub struct Parameters<'a> {
    pub symbol: Option<&'a str>,
    pub limit: Option<usize>,
    pub fromid: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub side: Option<Side>,
    pub order_type: Option<OrderType>,
    pub time_in_force: Option<TimeInForce>,
    pub quantity: Option<f64>,
    pub price: Option<f64>,
    pub new_client_order_id: Option<&'a str>,
    pub stop_price: Option<f64>,
    pub iceberg_qty: Option<f64>,
    pub new_order_resp_type: Option<OrderRespType>,
    pub order_id: Option<i64>,
    pub orig_client_order_id: Option<&'a str>,
    pub list_client_order_id: Option<&'a str>,
    pub limit_client_order_id:  Option<&'a str>,
    pub stop_client_order_id: Option<&'a str>,
    pub limit_iceberg_qty:  Option<f64>,
    pub stop_iceberg_qty:  Option<f64>,
    pub stop_limit_price: Option<f64>,
    pub stop_limit_time_in_force: Option<TimeInForce>,
    pub order_list_id: Option<f64>,
    pub listen_key: Option<&'a str>,
    pub recv_window: Option<usize>,
    pub timestamp: Option<i64>,
    pub signature: Option<String>,
}

impl<'a> Parameters<'a> {
    pub fn sign<T: Into<String>>(&mut self, secret: T) -> Result<&Self, Error> {
        self.timestamp = Some(Utc::now().timestamp_millis());
        let message = serde_urlencoded::to_string(&self)?;
        let mut mac = HmacSha256::new_varkey(secret.into().as_bytes()).unwrap();
        mac.input(message.as_bytes());
        let signature = mac.result().code();
        self.signature = Some(hex::encode(signature));

        Ok(self)
    }
}

#[derive(Default)]
pub struct PingParams<'a>(Parameters<'a>);

impl<'a> Deref for PingParams<'a> {
    type Target = Parameters<'a>;

    fn deref(&self) -> &Parameters<'a> {
        &self.0
    }
}

impl<'a> DerefMut for PingParams<'a> {
    fn deref_mut(&mut self) -> &mut Parameters<'a> {
        &mut self.0
    }
}