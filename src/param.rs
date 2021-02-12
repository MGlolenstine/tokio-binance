use chrono::Utc;
use hmac::{Hmac, Mac, NewMac};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

#[derive(Copy, Clone, Debug)]
pub enum ID<'a> {
    OrderId(i64),
    ClientOId(&'a str),
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum Side {
    Buy,
    Sell,
}

#[derive(Copy, Clone, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(super) enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum TimeInForce {
    Gtc,
    Ioc,
    Fok,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub enum OrderRespType {
    Ack,
    Result,
    Full,
}

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum Interval {
    #[serde(rename = "1m")]
    OneMinute,
    #[serde(rename = "3m")]
    ThreeMinutes,
    #[serde(rename = "5m")]
    FiveMinutes,
    #[serde(rename = "15m")]
    FifTeenMinutes,
    #[serde(rename = "30m")]
    ThirtyMinutes,
    #[serde(rename = "1h")]
    OneHour,
    #[serde(rename = "2h")]
    TwoHours,
    #[serde(rename = "4h")]
    FourHours,
    #[serde(rename = "6h")]
    SixHours,
    #[serde(rename = "8h")]
    EightHours,
    #[serde(rename = "12h")]
    TwelveHours,
    #[serde(rename = "1d")]
    OneDay,
    #[serde(rename = "3d")]
    ThreeDays,
    #[serde(rename = "1w")]
    OneWeek,
    #[serde(rename = "1M")]
    OneMonth,
}

#[derive(Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct Parameters<'a> {
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
    pub limit_client_order_id: Option<&'a str>,
    pub stop_client_order_id: Option<&'a str>,
    pub limit_iceberg_qty: Option<f64>,
    pub stop_iceberg_qty: Option<f64>,
    pub stop_limit_price: Option<f64>,
    pub stop_limit_time_in_force: Option<TimeInForce>,
    pub order_list_id: Option<i64>,
    pub listen_key: Option<&'a str>,
    pub address: Option<&'a str>,
    pub address_tag: Option<&'a str>,
    pub name: Option<&'a str>,
    pub asset: Option<&'a str>,
    pub status: Option<Value>,
    pub email: Option<&'a str>,
    pub page: Option<usize>,
    pub from_email: Option<&'a str>,
    pub to_email: Option<&'a str>,
    pub amount: Option<f64>,
    pub recv_window: Option<usize>,
    pub timestamp: Option<i64>,
    pub signature: Option<String>,
}

impl<'a> Parameters<'a> {
    pub fn sign<T: Into<String>>(&mut self, secret: T) -> crate::error::Result<&Self> {
        self.timestamp = Some(Utc::now().timestamp_millis());

        let message = serde_urlencoded::to_string(&self)?;
        let mut mac = HmacSha256::new_varkey(secret.into().as_bytes())?;
        mac.update(message.as_bytes());
        let signature = mac.finalize();

        self.signature = Some(hex::encode(signature.into_bytes()));
        Ok(self)
    }
}
