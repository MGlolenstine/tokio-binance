use async_tungstenite::{
    stream::Stream as StreamSwitcher,
    tokio::{connect_async, TokioAdapter},
    tungstenite::{
        handshake::client::Response,
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
    WebSocketStream as WsStream,
};
use core::pin::Pin;
use futures::{
    sink::Sink,
    stream::{Stream, TryStreamExt},
    task::{Context, Poll},
    SinkExt,
};
use tokio::net::TcpStream;
use tokio_tls::TlsStream;

use crate::error::{Error, Kind, WsCloseError};
use serde::de::DeserializeOwned;
use serde_json::Value;
use serde::Serialize;

/// wss://stream.binance.us:9443
pub const BINANCE_US_WSS_URL: &'static str = "wss://stream.binance.us:9443";

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Channel {
    AggTrade,
    Depth,
    Trade,
}

#[derive(Serialize)]
struct SubscribeMessage<'a> {
    method: &'a str,
    params: &'a [Value],
    id: u64,
}

type InnerStream = (
    WsStream<
        StreamSwitcher<
            TokioAdapter<TcpStream>,
            TokioAdapter<TlsStream<TokioAdapter<TokioAdapter<TcpStream>>>>,
        >,
    >,
    Response,
);

pub struct WebSocketStream {
    inner: InnerStream,
    id: u64
}

impl WebSocketStream {
    pub async fn connect<T: Into<String>>(symbol: &str, channel: Channel, url: T) -> crate::error::Result<Self> {
        let channel = serde_json::to_value(channel)?;
        let channel = if let Some(channel) = channel.as_str() {
            Ok(channel)
        } else {
            // this is to avoid calling unwrap but I know this will never fail anyways...
            Err(Error::new(Kind::Other, "Can't convert channel to string".into()))
        };

        let url = url.into() + "/ws/" + &symbol.to_lowercase() + "@" + channel?;

        let inner = connect_async(url).await?;
        let mut stream = Self { inner, id: 0 };

        let message = SubscribeMessage { method: "SET_PROPERTY", params: &["combined".into(), true.into()], id: stream.id };
        let message = serde_json::to_string(&message)?;
        stream.send(Message::Text(message)).await?;
        stream.id += 1;

        Ok(stream)
    }

    pub async fn text(&mut self) -> crate::error::Result<Option<String>> {
        match self.try_next().await? {
            Some(msg) => {
                match msg {
                    Message::Text(text) => Ok(Some(text)),
                    Message::Ping(ref value) => {
                        self.send(Message::Pong(value.clone())).await?;
                        let ping = serde_json::json!({
                            "ping": msg.into_text()?,
                        });
                        Ok(Some(serde_json::to_string(&ping)?))
                    },
                    Message::Pong(ref value) => {
                        self.send(Message::Ping(value.clone())).await?;
                        let pong = serde_json::json!({
                            "pong": msg.into_text()?,
                        });
                        Ok(Some(serde_json::to_string(&pong)?))
                    },
                    Message::Binary(_) => Ok(Some(msg.into_text()?)),
                    Message::Close(Some(frame)) => Err(WsCloseError::new(frame.code, frame.reason).into()),
                    Message::Close(None) => Err(WsCloseError::new(CloseCode::Abnormal, "Close message with no frame received").into()),
                }
            },
            None => Ok(None)
        }
    }

    pub async fn json<J: DeserializeOwned>(&mut self) -> crate::error::Result<Option<J>> {
        match self.text().await? {
            Some(text) => Ok(Some(serde_json::from_str(&text)?)),
            None => Ok(None)
        }
    }

    pub async fn subscribe(&mut self, channels: &[(&str, Channel)]) -> crate::error::Result<()> {
        self.send_msg("SUBSCRIBE", channels).await
    }

    pub async fn unsubscribe(&mut self, channels: &[(&str, Channel)]) -> crate::error::Result<()> {
        self.send_msg("UNSUBSCRIBE", channels).await
    }

    pub fn get_ref(&self) -> &InnerStream {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut InnerStream {
        &mut self.inner
    }

    pub async fn close(&mut self, msg: Option<CloseFrame<'_>>) -> crate::error::Result<()> {
        self.inner.0.close(msg).await?;
        Ok(())
    }

    async fn send_msg(&mut self, method: &str, channels: &[(&str, Channel)]) -> crate::error::Result<()> {
        let params: Result<Vec<_>, _> = channels
            .iter()
            .map(|(symbol, channel)| -> crate::error::Result<Value> {
                let channel = serde_json::to_value(channel)?;
                let channel = if let Some(channel) = channel.as_str() {
                    Ok(channel)
                } else {
                    // this is to avoid calling unwrap but I know this will never fail anyways...
                    Err(Error::new(Kind::Other, "Can't convert channel to string".into()))
                };

                let channel = symbol.to_lowercase() + "@" + channel?;
                Ok(channel.into())
            })
            .collect();
        
        let message = SubscribeMessage { method, params: &params?, id: self.id };
        let message = serde_json::to_string(&message)?;
        self.send(Message::Text(message)).await?;
        self.id += 1;
        Ok(())
    }
}

impl Stream for WebSocketStream {
    type Item = crate::error::Result<Message>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner.0).poll_next(cx) {
            Poll::Ready(Some(val)) => Poll::Ready(Some(Ok(val?))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Sink<Message> for WebSocketStream {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match Pin::new(&mut self.inner.0).poll_ready(cx) {
            Poll::Ready(Ok(val)) => Poll::Ready(Ok(val)),
            Poll::Ready(Err(val)) => Poll::Ready(Err(Error::new(Kind::Tungstenite, Some(val)))),
            Poll::Pending => Poll::Pending,
        }
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        match Pin::new(&mut self.inner.0).start_send(item) {
            Ok(val) => Ok(val),
            Err(val) => Err(Error::new(Kind::Tungstenite, Some(val))),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match Pin::new(&mut self.inner.0).poll_flush(cx) {
            Poll::Ready(Ok(val)) => Poll::Ready(Ok(val)),
            Poll::Ready(Err(val)) => Poll::Ready(Err(Error::new(Kind::Tungstenite, Some(val)))),
            Poll::Pending => Poll::Pending,
        }
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match Pin::new(&mut self.inner.0).poll_close(cx) {
            Poll::Ready(Ok(val)) => Poll::Ready(Ok(val)),
            Poll::Ready(Err(val)) => Poll::Ready(Err(Error::new(Kind::Tungstenite, Some(val)))),
            Poll::Pending => Poll::Pending,
        }
    }
}
