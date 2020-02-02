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

pub struct TradesParams;
impl Limit for TradesParams {}