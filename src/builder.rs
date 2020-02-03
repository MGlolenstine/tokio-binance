use crate::param::{self, Parameters};
use reqwest::{RequestBuilder, Response, header::CONTENT_TYPE};
use crate::error::BinanceError;
use std::marker::PhantomData;
use crate::types::*;
use log::warn;

pub struct ParamBuilder<'a, 'b, T> {
    _marker: PhantomData<T>,
    params: Parameters<'a>,
    builder: RequestBuilder,
    api_key: Option<&'b str>,
    secret_key: Option<&'b str>,

}

impl<'a, 'b, T> ParamBuilder<'a, 'b, T> {
    pub(super) fn new(params: Parameters<'a>, builder: RequestBuilder, api_key: Option<&'b str>, secret_key: Option<&'b str>) -> Self {
        Self {
            _marker: PhantomData,
            params,
            builder,
            api_key,
            secret_key
        }
    }

    pub async fn text(self) -> crate::error::Result<String> {
        let text = self.response().await?.text().await?;
        Ok(text)
    }

    async fn response(self) -> crate::error::Result<Response> {
        let res = self.builder()?.send().await?;
        let status = res.status();

        if status.is_success() { 
            Ok(res) 
        } else if status.is_client_error() {
            let reason = status.canonical_reason().unwrap_or("UNKNOWN");
            let message = res.text().await.unwrap_or("".into());
            let err = BinanceError::new(status.as_u16(), reason, &message);
            Err(err.into())
        } else {
            warn!("{}", status);
            Ok(res)
        }
    }

    fn builder(mut self) -> crate::error::Result<RequestBuilder> {
        let builder = if let Some(api_key) = self.api_key {
            self.builder.header("X-MBX-APIKEY", api_key)
        } else {
            self.builder
        };

        let params = if let Some(secret_key) = self.secret_key {
            self.params.sign(secret_key)?
        } else {
            &self.params
        };

        let request = builder.try_clone().expect("Unsupported body").build()?;

        let builder = if request.method() == "POST" || request.method() == "PUT" {
            let body = serde_urlencoded::to_string(params)?;
            builder.body(body).header(CONTENT_TYPE, "application/x-www-form-urlencoded")
        } else {
            builder.query(params)
        };

        Ok(builder)
    }
}

impl<'a, 'b, T: Symbol> ParamBuilder<'a, 'b, T> {
    pub fn with_symbol(mut self, symbol: &'a str) -> Self {
        self.params.symbol = Some(symbol);
        self
    }
}

impl<'a, 'b, T: Limit> ParamBuilder<'a, 'b, T> {
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.params.limit = Some(limit);
        self
    }
}

impl<'a, 'b, T: FromId> ParamBuilder<'a, 'b, T> {
    pub fn with_from_id(mut self, from_id: i64) -> Self {
        self.params.from_id = Some(from_id);
        self
    }
}

impl<'a, 'b, T: StartTime> ParamBuilder<'a, 'b, T> {
    pub fn with_start_time(mut self, start_time: i64) -> Self {
        self.params.start_time = Some(start_time);
        self
    }
}

impl<'a, 'b, T: EndTime> ParamBuilder<'a, 'b, T> {
    pub fn with_end_time(mut self, end_time: i64) -> Self {
        self.params.end_time = Some(end_time);
        self
    }
}

impl<'a, 'b, T: Side> ParamBuilder<'a, 'b, T> {
    pub fn with_side(mut self, side: param::Side) -> Self {
        self.params.side = Some(side);
        self
    }
}

impl<'a, 'b, T: OrderType> ParamBuilder<'a, 'b, T> {
    pub fn with_type(mut self, order_type: param::OrderType) -> Self {
        self.params.order_type = Some(order_type);
        self
    }
}

impl<'a, 'b, T: TimeInForce> ParamBuilder<'a, 'b, T> {
    pub fn with_time_in_force(mut self, time_in_force: param::TimeInForce) -> Self {
        self.params.time_in_force = Some(time_in_force);
        self
    }
}

impl<'a, 'b, T: Quantity> ParamBuilder<'a, 'b, T> {
    pub fn with_quantity(mut self, quantity: f64) -> Self {
        self.params.quantity = Some(quantity);
        self
    }
}

impl<'a, 'b, T: Price> ParamBuilder<'a, 'b, T> {
    pub fn with_price(mut self, price: f64) -> Self {
        self.params.price = Some(price);
        self
    }
}

impl<'a, 'b, T: NewClientOrderId> ParamBuilder<'a, 'b, T> {
    pub fn with_new_client_order_id(mut self, new_client_order_id: &'a str) -> Self {
        self.params.new_client_order_id = Some(new_client_order_id);
        self
    }
}

impl<'a, 'b, T: StopPrice> ParamBuilder<'a, 'b, T> {
    pub fn with_stop_price(mut self, stop_price: f64) -> Self {
        self.params.stop_price = Some(stop_price);
        self
    }
}

impl<'a, 'b, T: IcebergQty> ParamBuilder<'a, 'b, T> {
    pub fn with_iceberg_qty(mut self, iceberg_qty: f64) -> Self {
        self.params.iceberg_qty = Some(iceberg_qty);
        self
    }
}

impl<'a, 'b, T: NewOrderRespType> ParamBuilder<'a, 'b, T> {
    pub fn with_new_order_resp_type(mut self, new_order_resp_type: param::OrderRespType) -> Self {
        self.params.new_order_resp_type = Some(new_order_resp_type);
        self
    }
}

impl<'a, 'b, T: OrderId> ParamBuilder<'a, 'b, T> {
    pub fn with_order_id(mut self, order_id: i64) -> Self {
        self.params.order_id = Some(order_id);
        self
    }
}

impl<'a, 'b, T: OrigClientOrderId> ParamBuilder<'a, 'b, T> {
    pub fn with_orig_client_order_id(mut self, orig_client_order_id: &'a str) -> Self {
        self.params.orig_client_order_id = Some(orig_client_order_id);
        self
    }
}

impl<'a, 'b, T: ListClientOrderId> ParamBuilder<'a, 'b, T> {
    pub fn with_list_client_order_id(mut self, list_client_order_id: &'a str) -> Self {
        self.params.list_client_order_id = Some(list_client_order_id);
        self
    }
}

impl<'a, 'b, T: LimitClientOrderId> ParamBuilder<'a, 'b, T> {
    pub fn with_limit_client_order_id(mut self, limit_client_order_id: &'a str) -> Self {
        self.params.limit_client_order_id = Some(limit_client_order_id);
        self
    }
}

impl<'a, 'b, T: StopClientOrderId> ParamBuilder<'a, 'b, T> {
    pub fn with_stop_client_order_id(mut self, stop_client_order_id: &'a str) -> Self {
        self.params.stop_client_order_id = Some(stop_client_order_id);
        self
    }
}

impl<'a, 'b, T: LimitIcebergQty> ParamBuilder<'a, 'b, T> {
    pub fn with_limit_iceberg_qty(mut self, limit_iceberg_qty: f64) -> Self {
        self.params.limit_iceberg_qty = Some(limit_iceberg_qty);
        self
    }
}

impl<'a, 'b, T: StopIcebergQty> ParamBuilder<'a, 'b, T> {
    pub fn with_stop_iceberg_qty(mut self, stop_iceberg_qty: f64) -> Self {
        self.params.stop_iceberg_qty = Some(stop_iceberg_qty);
        self
    }
}

impl<'a, 'b, T: StopLimitPrice> ParamBuilder<'a, 'b, T> {
    pub fn with_stop_limit_price(mut self, stop_limit_price: f64) -> Self {
        self.params.stop_limit_price = Some(stop_limit_price);
        self
    }
}

impl<'a, 'b, T: StopLimitTimeInForce> ParamBuilder<'a, 'b, T> {
    pub fn with_stop_limit_time_in_force(mut self, stop_limit_time_in_force: param::TimeInForce) -> Self {
        self.params.stop_limit_time_in_force = Some(stop_limit_time_in_force);
        self
    }
}

impl<'a, 'b, T: OrderListId> ParamBuilder<'a, 'b, T> {
    pub fn with_order_list_id(mut self, order_list_id: f64) -> Self {
        self.params.order_list_id = Some(order_list_id);
        self
    }
}

impl<'a, 'b, T: RecvWindow> ParamBuilder<'a, 'b, T> {
    pub fn with_recv_window(mut self, recv_window: usize) -> Self {
        self.params.recv_window = Some(recv_window);
        self
    }
}