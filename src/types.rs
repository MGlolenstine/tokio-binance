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
pub trait ListClientOrderId {}
pub trait LimitClientOrderId {}
pub trait StopClientOrderId {}
pub trait LimitIcebergQty {}
pub trait StopIcebergQty {}
pub trait StopLimitPrice {}
pub trait RecvWindow {}

pub trait LimitMaker {}
pub trait LimitOrderStopPrice {}
pub trait MarketOrderStopPrice {}

pub trait AddressTag {}
pub trait Name {}
pub trait Asset {}
pub trait Status {}
pub trait Email {}
pub trait Page {}

pub struct PingParams;
pub struct TimeParams;
pub struct ExchangeInfoParams;
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
impl LimitMaker for LimitOrderParams {}
impl LimitOrderStopPrice for LimitOrderParams {}
impl NewClientOrderId for LimitOrderParams {}
impl IcebergQty for LimitOrderParams {}
impl NewOrderRespType for LimitOrderParams {}
impl RecvWindow for LimitOrderParams {}

pub struct LimitMakerOrderParams;
impl NewClientOrderId for LimitMakerOrderParams {}
impl NewOrderRespType for LimitMakerOrderParams {}
impl RecvWindow for LimitMakerOrderParams {}

pub struct MarketOrderParams;
impl MarketOrderStopPrice for MarketOrderParams {}
impl NewClientOrderId for MarketOrderParams {}
impl NewOrderRespType for MarketOrderParams {}
impl RecvWindow for MarketOrderParams {}

pub struct OrderStatusParams;
impl RecvWindow for OrderStatusParams {}

pub struct CancelOrderParams;
impl NewClientOrderId for CancelOrderParams {}
impl RecvWindow for CancelOrderParams {}

pub struct OpenOrderParams;
impl Symbol for OpenOrderParams {}
impl RecvWindow for OpenOrderParams {}

pub struct AllOrdersParams;
impl OrderId for AllOrdersParams {}
impl StartTime for AllOrdersParams {}
impl EndTime for AllOrdersParams {}
impl Limit for AllOrdersParams {}
impl RecvWindow for AllOrdersParams {}

pub struct OcoParams;
impl ListClientOrderId for OcoParams {}
impl LimitClientOrderId for OcoParams {}
impl LimitIcebergQty for OcoParams {}
impl StopClientOrderId for OcoParams {}
impl StopIcebergQty for OcoParams {}
impl StopLimitPrice for OcoParams {}
impl NewOrderRespType for OcoParams {}
impl RecvWindow for OcoParams {}

pub struct CancelOcoParams;
impl NewClientOrderId for CancelOcoParams {}
impl RecvWindow for CancelOcoParams {}

pub struct OcoStatusParams;
impl RecvWindow for OcoStatusParams {}

pub struct AllOcoParams;
impl Limit for AllOcoParams {}
impl FromId for AllOcoParams {}
impl StartTime for AllOcoParams {}
impl EndTime for AllOcoParams {}
impl RecvWindow for AllOcoParams {}

pub struct OpenOcoParams;
impl RecvWindow for OpenOcoParams {}

pub struct AccountParams;
impl RecvWindow for AccountParams {}

pub struct AccountTradesParams;
impl Limit for AccountTradesParams {}
impl FromId for AccountTradesParams {}
impl StartTime for AccountTradesParams {}
impl EndTime for AccountTradesParams {}
impl RecvWindow for AccountTradesParams {}

pub struct StartStreamParams;
pub struct KeepAliveStreamParams;
pub struct CloseStreamParams;

pub struct WithdrawParams;
impl AddressTag for WithdrawParams {}
impl Name for WithdrawParams {}
impl RecvWindow for WithdrawParams {}

pub struct DepositHistoryParams;
impl Asset for DepositHistoryParams {}
impl Status for DepositHistoryParams {}
impl StartTime for DepositHistoryParams {}
impl EndTime for DepositHistoryParams {}
impl RecvWindow for DepositHistoryParams {}

pub struct WithdrawHistoryParams;
impl Asset for WithdrawHistoryParams {}
impl Status for WithdrawHistoryParams {}
impl StartTime for WithdrawHistoryParams {}
impl EndTime for WithdrawHistoryParams {}
impl RecvWindow for WithdrawHistoryParams {}

pub struct DepositAddressParams;
impl Status for DepositAddressParams {}
impl RecvWindow for DepositAddressParams {}

pub struct AccountStatusParams;
impl RecvWindow for AccountStatusParams {}

pub struct SystemStatusParams;

pub struct ApiStatusParams;
impl RecvWindow for ApiStatusParams {}

pub struct DustlogParams;
impl RecvWindow for DustlogParams {}

pub struct TradeFeeParams;
impl Symbol for TradeFeeParams {}
impl RecvWindow for TradeFeeParams {}

pub struct AssetDetailParams;
impl RecvWindow for AssetDetailParams {}

pub struct SubAccountParams;
impl Email for SubAccountParams {}
impl Status for SubAccountParams {}
impl Page for SubAccountParams {}
impl Limit for SubAccountParams {}
impl RecvWindow for SubAccountParams {}

pub struct SubAccountTranferParams;
impl StartTime for SubAccountTranferParams {}
impl EndTime for SubAccountTranferParams {}
impl Page for SubAccountTranferParams {}
impl Limit for SubAccountTranferParams {}
impl RecvWindow for SubAccountTranferParams {}

pub struct TransferSubAccountParams;
impl RecvWindow for TransferSubAccountParams {}

pub struct SubAccountAssetParams;
impl Symbol for SubAccountAssetParams {}
impl RecvWindow for SubAccountAssetParams {}

pub struct DustTransferParams;
impl RecvWindow for DustTransferParams {}

pub struct AssetDividendParams;
impl Asset for AssetDividendParams {}
impl StartTime for AssetDividendParams {}
impl EndTime for AssetDividendParams {}
impl RecvWindow for AssetDividendParams {}
