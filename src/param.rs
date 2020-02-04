use serde::Serialize;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use chrono::Utc;
use serde_urlencoded::ser::Error;

type HmacSha256 = Hmac<Sha256>;

pub enum ID<'a> {
    OrderId(i64),
    OriClientOrderId(&'a str)
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok
}

#[derive(Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderRespType {
    Ack,
    Result,
    Full
}

#[derive(Serialize)]
pub enum Interval {
    #[serde(rename = "1m")]  OneMinute,
    #[serde(rename = "3m")]  ThreeMinutes,
    #[serde(rename = "5m")]  FiveMinutes,
    #[serde(rename = "15m")] FifTeenMinutes,
    #[serde(rename = "30m")] ThirtyMinutes,
    #[serde(rename = "1h")]  OneHour,
    #[serde(rename = "2h")]  TwoHours,
    #[serde(rename = "4h")]  FourHours,
    #[serde(rename = "6h")]  SixHours,
    #[serde(rename = "8h")]  EightHours,
    #[serde(rename = "12h")] TwelveHours,
    #[serde(rename = "1d")]  OneDay,
    #[serde(rename = "3d")]  ThreeDays,
    #[serde(rename = "1w")]  OneWeek,
    #[serde(rename = "1M")]  OneMonth,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Parameters<'a> {
    pub symbol: Option<&'a str>,
    pub limit: Option<usize>,
    pub from_id: Option<i64>,
    pub start_time: Option<i64>,
    pub end_time: Option<i64>,
    pub interval: Option<Interval>,
    pub side: Option<Side>,
    #[serde(rename = "type")]
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
        let mut mac = HmacSha256::new_varkey(secret.into().as_bytes()).expect("Invalid Key Length");
        mac.input(message.as_bytes());
        let signature = mac.result().code();

        self.signature = Some(hex::encode(signature));
        Ok(self)
    }
}