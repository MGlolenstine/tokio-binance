use crate::param::*;
use std::ops::{Deref, DerefMut};
use reqwest::{RequestBuilder, header::CONTENT_TYPE};
use crate::error::BinanceError;

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
        
        let res = self.builder()?.send().await?;
        let status = res.status();

        if status.is_success() {
            Ok(res.text().await?)
        } else {
            let reason = status.canonical_reason().unwrap_or("UNKNOWN");
            println!("{:?}", res.headers());
            println!("{:?}", res.text().await?);
            Err(BinanceError::new(status.as_u16(), reason).into())
        }
    }

    fn builder(self) -> crate::error::Result<RequestBuilder> {
        let Self {mut params, auth, builder} = self;
        let (params, builder) = if let Some(Auth {api_key, api_secret}) = auth {
            (params.sign(api_secret)?, builder.header("X-MBX-APIKEY", api_key))
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
