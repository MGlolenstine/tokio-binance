#[derive(Default)]
struct Parameters<'a> {
    symbol: Option<&'a str>,
    limit: Option<usize>,
    fromid: Option<i64>,
    start_time: Option<i64>,
    end_time: Option<i64>,
    side: Option<Side>,
    order_type: Option<OrderType>,
    time_in_force: Option<TimeInForce>,
    quantity: Option<f64>,
    price: Option<f64>,
    new_client_order_id: Option<&'a str>,
    stop_price: Option<f64>,
    iceberg_qty: Option<f64>,
    new_order_resp_type: Option<OrderRespType>,

}

enum Side {
    Buy,
    Sell
}

enum OrderType {
    Limit,
    Market,
    StopLoss,
    StopLossLimit,
    TakeProfit,
    TakeProfitLimit,
    LimitMaker
}

enum TimeInForce {
    Gtc,
    Ioc,
    Fok
}

enum OrderRespType {
    Ack,
    Result,
    Full
}