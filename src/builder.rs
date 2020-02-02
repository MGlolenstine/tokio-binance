use crate::param::*;
use std::ops::{Deref, DerefMut};
use reqwest::{RequestBuilder, Response, header::CONTENT_TYPE};
use crate::error::BinanceError;
use log::warn;

pub struct Auth<'a> {
    pub api_key: &'a str,
    pub secret_key: &'a str
}

pub struct ParamBuilder<'b, T> {
    params: T,
    builder: RequestBuilder,
    auth: Option<Auth<'b>>
}

impl<'a, 'b, T: Deref<Target=Parameters<'a>>> ParamBuilder<'b, T> 
where
    T: Deref<Target=Parameters<'a>> + DerefMut
{
    pub(super) fn new(params: T, builder: RequestBuilder, auth: Option<Auth<'b>>) -> Self {
        Self {
            params,
            builder,
            auth
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

    fn builder(self) -> crate::error::Result<RequestBuilder> {
        let Self {mut params, auth, builder} = self;
        let (params, builder) = if let Some(Auth {api_key, secret_key}) = auth {
            (params.sign(secret_key)?, builder.header("X-MBX-APIKEY", api_key))
        } else {
            (&*params, builder)
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
