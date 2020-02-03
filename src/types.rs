pub trait Symbol {}
pub trait Limit {}
pub trait FromId {}
pub trait StartTime {}
pub trait EndTime {}
pub trait Side {}
pub trait OrderType {}
pub trait TimeInForce {}
pub trait Quantity {}
pub trait Price {}
pub trait NewClientOrderId {}
pub trait StopPrice {}
pub trait IcebergQty {}
pub trait NewOrderRespType {}
pub trait OrderId {}
pub trait OrigClientOrderId {}
pub trait ListClientOrderId {}
pub trait LimitClientOrderId {}
pub trait StopClientOrderId {}
pub trait LimitIcebergQty {}
pub trait StopIcebergQty {}
pub trait StopLimitPrice {}
pub trait StopLimitTimeInForce {}
pub trait OrderListId {}
pub trait RecvWindow {}

pub struct PingParams;
pub struct TimeParams;
pub struct ExchangeInfoParams;
pub struct AccountParams;
pub struct AveragePriceParams;

pub struct OrderBookParams;
impl Limit for OrderBookParams {}

pub struct TradesParams;
impl Limit for TradesParams {}

pub struct HistoricalTradesParams;
impl Limit for HistoricalTradesParams {}
impl FromId for HistoricalTradesParams {}

pub struct AggTradesParams;
impl Limit for AggTradesParams {}
impl FromId for AggTradesParams {}
impl StartTime for AggTradesParams {}
impl EndTime for AggTradesParams {}

pub struct KlinesParams;
impl Limit for KlinesParams {}
impl StartTime for KlinesParams {}
impl EndTime for KlinesParams {}

pub struct TwentyfourHourTickerPriceParams;
impl Symbol for TwentyfourHourTickerPriceParams {}

pub struct TickerPriceParams;
impl Symbol for TickerPriceParams {}

pub struct OrderBookTickerParams;
impl Symbol for OrderBookTickerParams {}