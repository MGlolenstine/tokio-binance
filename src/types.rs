pub trait Symbol {}
pub trait Limit {}
pub trait FromId {}
pub trait StartTime {}
pub trait EndTime {}
pub trait TimeInForce {}
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

pub trait LimitOrderStopPrice {}
pub trait MarketOrderStopPrice {}

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

pub struct LimitOrderParams;
impl TimeInForce for LimitOrderParams {}
impl LimitOrderStopPrice for LimitOrderParams {}
impl NewClientOrderId for LimitOrderParams {}
impl IcebergQty for LimitOrderParams {}
impl NewOrderRespType for LimitOrderParams {}
impl RecvWindow for LimitOrderParams {}

pub struct MarketOrderParams;
impl MarketOrderStopPrice for MarketOrderParams {}
impl NewClientOrderId for MarketOrderParams {}
impl NewOrderRespType for MarketOrderParams {}
impl RecvWindow for MarketOrderParams {}

pub struct OrderStatusParams;
impl RecvWindow for OrderStatusParams {}