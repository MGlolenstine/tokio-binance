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
    stream::TryStreamExt,
    task::{Context, Poll},
    SinkExt, Stream,
};
use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;

use crate::error::{Error, Kind, WsCloseError};
use crate::param::Interval;
use serde::de::DeserializeOwned;
use serde::Serialize;
use serde_json::Value;
use std::fmt;

/// wss://stream.binance.us:9443
pub const BINANCE_US_WSS_URL: &'static str = "wss://stream.binance.us:9443";

#[derive(Copy, Clone)]
pub enum Channel<'c> {
    AggTrade(&'c str),
    Depth(&'c str, Speed),
    Trade(&'c str),
    Kline(&'c str, Interval),
    MiniTicker(&'c str),
    AllMiniTickers,
    Ticker(&'c str),
    AllTickers,
    BookTicker(&'c str),
    AllBookTickers,
    PartialDepth(&'c str, Level, Speed),
    /// The only channel that takes a listen-key instead of a symbol
    UserData(&'c str),
}

impl<'c> fmt::Display for Channel<'c> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AggTrade(symbol) => write!(f, "{}", symbol.to_lowercase() + "@aggTrade"),
            Self::Trade(symbol) => write!(f, "{}", symbol.to_lowercase() + "@trade"),
            Self::Kline(symbol, interval) => {
                let interval = serde_json::to_value(interval).unwrap();
                write!(
                    f,
                    "{}",
                    symbol.to_lowercase() + "@kline_" + interval.as_str().unwrap()
                )
            }
            Self::MiniTicker(symbol) => write!(f, "{}", symbol.to_lowercase() + "@miniTicker"),
            Self::AllMiniTickers => write!(f, "!miniTicker@arr"),
            Self::Ticker(symbol) => write!(f, "{}", symbol.to_lowercase() + "@ticker"),
            Self::AllTickers => write!(f, "!ticker@arr"),
            Self::BookTicker(symbol) => write!(f, "{}", symbol.to_lowercase() + "@bookTicker"),
            Self::AllBookTickers => write!(f, "!bookTicker"),
            Self::PartialDepth(symbol, level, speed) => {
                let level = serde_json::to_value(level).unwrap();
                let speed = serde_json::to_value(speed).unwrap();
                write!(
                    f,
                    "{}",
                    symbol.to_lowercase()
                        + "@depth"
                        + level.as_str().unwrap()
                        + "@"
                        + speed.as_str().unwrap()
                )
            }
            Self::Depth(symbol, speed) => {
                let speed = serde_json::to_value(speed).unwrap();
                write!(
                    f,
                    "{}",
                    symbol.to_lowercase() + "@depth@" + speed.as_str().unwrap()
                )
            }
            Self::UserData(listen_key) => write!(f, "{}", listen_key),
        }
    }
}

impl<'a, 'c> PartialEq<&'a str> for Channel<'c> {
    fn eq(&self, other: &&str) -> bool {
        self.to_string() == *other
    }
}

impl<'c> PartialEq<String> for Channel<'c> {
    fn eq(&self, other: &String) -> bool {
        self.to_string() == *other
    }
}

impl<'c> PartialEq<Value> for Channel<'c> {
    fn eq(&self, other: &Value) -> bool {
        self.to_string() == *other
    }
}

#[derive(Copy, Clone, Serialize)]
pub enum Level {
    #[serde(rename = "5")]
    Five,
    #[serde(rename = "10")]
    Ten,
    #[serde(rename = "20")]
    Twenty,
}

#[derive(Copy, Clone, Serialize)]
pub enum Speed {
    #[serde(rename = "100ms")]
    HundredMillis,
    #[serde(rename = "1000ms")]
    ThousandMillis,
}

#[derive(Serialize)]
struct SubscribeMessage<'a> {
    method: &'a str,
    params: &'a [Value],
    id: u64,
}

type InnerStream = (
    WsStream<StreamSwitcher<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>,
    Response,
);

/// Websocket stream for the various binance channels aka streams.
pub struct WebSocketStream {
    inner: InnerStream,
    id: u64,
}

impl WebSocketStream {
    /// Start websocket stream by connecting to a channel.
    /// # Example
    ///
    /// ```no_run
    /// use tokio_binance::{WebSocketStream, BINANCE_US_WSS_URL, Channel};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let channel = Channel::Ticker("BNBUSDT");
    ///     let mut stream = WebSocketStream::connect(channel, BINANCE_US_WSS_URL).await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect<U: Into<String>>(
        channel: Channel<'_>,
        url: U,
    ) -> crate::error::Result<Self> {
        let url = url.into() + "/ws/" + &channel.to_string();

        let inner = connect_async(url).await?;
        let mut stream = Self { inner, id: 0 };

        let message = SubscribeMessage {
            method: "SET_PROPERTY",
            params: &["combined".into(), true.into()],
            id: stream.id,
        };
        let message = serde_json::to_string(&message)?;
        stream.send(Message::Text(message)).await?;
        stream.id += 1;

        Ok(stream)
    }
    /// Helper method for getting messages as text.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WebSocketStream, BINANCE_US_WSS_URL, Channel};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let channel = Channel::Ticker("BNBUSDT");
    /// # let mut stream = WebSocketStream::connect(channel, BINANCE_US_WSS_URL).await?;
    /// while let Some(text) = stream.text().await? {
    ///     println!("{}", text);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn text(&mut self) -> crate::error::Result<Option<String>> {
        match self.try_next().await? {
            Some(msg) => match msg {
                Message::Text(text) => Ok(Some(text)),
                Message::Ping(ref value) => {
                    self.send(Message::Pong(value.clone())).await?;
                    let ping = serde_json::json!({
                        "ping": msg.into_text()?,
                    });
                    Ok(Some(serde_json::to_string(&ping)?))
                }
                Message::Pong(ref value) => {
                    self.send(Message::Ping(value.clone())).await?;
                    let pong = serde_json::json!({
                        "pong": msg.into_text()?,
                    });
                    Ok(Some(serde_json::to_string(&pong)?))
                }
                Message::Binary(_) => Ok(Some(msg.into_text()?)),
                Message::Close(Some(frame)) => {
                    Err(WsCloseError::new(frame.code, frame.reason).into())
                }
                Message::Close(None) => Err(WsCloseError::new(
                    CloseCode::Abnormal,
                    "Close message with no frame received",
                )
                .into()),
            },
            None => Ok(None),
        }
    }
    /// Helper method for getting messages as a serde deserializable.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WebSocketStream, BINANCE_US_WSS_URL, Channel};
    /// use serde_json::Value;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let channel = Channel::Ticker("BNBUSDT");
    /// # let mut stream = WebSocketStream::connect(channel, BINANCE_US_WSS_URL).await?;
    /// while let Some(value) = stream.json::<Value>().await? {
    ///     // filter the messages before accessing a field.
    ///     if channel == value["stream"] {
    ///         println!("{}", serde_json::to_string_pretty(&value)?);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn json<J: DeserializeOwned>(&mut self) -> crate::error::Result<Option<J>> {
        match self.text().await? {
            Some(text) => Ok(Some(serde_json::from_str(&text)?)),
            None => Ok(None),
        }
    }
    /// Subscribe to one or more channels aka streams.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WebSocketStream, BINANCE_US_WSS_URL};
    /// use tokio_binance::{Channel, Interval};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let channel = Channel::Ticker("BNBUSDT");
    /// # let mut stream = WebSocketStream::connect(channel, BINANCE_US_WSS_URL).await?;
    /// stream.subscribe(&[
    ///     Channel::AggTrade("BNBUSDT"),
    ///     Channel::Ticker("BTCUSDT"),
    ///     Channel::Kline("BNBUSDT", Interval::OneMinute)
    ///     // and so on
    /// ]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn subscribe(&mut self, channels: &[Channel<'_>]) -> crate::error::Result<()> {
        self.send_msg("SUBSCRIBE", channels).await
    }
    /// Unsubscribe from one or more channels aka streams.
    /// # Example
    ///
    /// ```no_run
    /// # use tokio_binance::{WebSocketStream, BINANCE_US_WSS_URL};
    /// use tokio_binance::{Channel, Interval};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let channel = Channel::Ticker("BNBUSDT");
    /// # let mut stream = WebSocketStream::connect(channel, BINANCE_US_WSS_URL).await?;
    /// stream.unsubscribe(&[
    ///     Channel::AggTrade("BNBUSDT"),
    ///     Channel::Kline("BNBUSDT", Interval::OneMinute)
    ///     // and so on
    /// ]).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn unsubscribe(&mut self, channels: &[Channel<'_>]) -> crate::error::Result<()> {
        self.send_msg("UNSUBSCRIBE", channels).await
    }
    /// Returns a shared reference to the inner stream.
    pub fn get_ref(&self) -> &InnerStream {
        &self.inner
    }
    /// Returns a mutable reference to the inner stream.
    pub fn get_mut(&mut self) -> &mut InnerStream {
        &mut self.inner
    }
    /// Close the underlying web socket
    pub async fn close(&mut self, msg: Option<CloseFrame<'_>>) -> crate::error::Result<()> {
        self.inner.0.close(msg).await?;
        Ok(())
    }

    async fn send_msg(
        &mut self,
        method: &str,
        channels: &[Channel<'_>],
    ) -> crate::error::Result<()> {
        let params: Vec<_> = channels
            .iter()
            .map(|channel| Value::String(channel.to_string()))
            .collect();

        let message = SubscribeMessage {
            method,
            params: &params,
            id: self.id,
        };
        let message = serde_json::to_string(&message)?;
        self.send(Message::Text(message)).await?;
        self.id += 1;
        Ok(())
    }
}

impl Stream for WebSocketStream {
    type Item = crate::error::Result<Message>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match self.inner.0.try_poll_next_unpin(cx) {
            Poll::Ready(Some(val)) => Poll::Ready(Some(Ok(val?))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Sink<Message> for WebSocketStream {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match self.inner.0.poll_ready_unpin(cx) {
            Poll::Ready(Ok(val)) => Poll::Ready(Ok(val)),
            Poll::Ready(Err(val)) => Poll::Ready(Err(Error::new(Kind::Tungstenite, Some(val)))),
            Poll::Pending => Poll::Pending,
        }
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        match self.inner.0.start_send_unpin(item) {
            Ok(val) => Ok(val),
            Err(val) => Err(Error::new(Kind::Tungstenite, Some(val))),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match self.inner.0.poll_flush_unpin(cx) {
            Poll::Ready(Ok(val)) => Poll::Ready(Ok(val)),
            Poll::Ready(Err(val)) => Poll::Ready(Err(Error::new(Kind::Tungstenite, Some(val)))),
            Poll::Pending => Poll::Pending,
        }
    }
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match self.inner.0.poll_close_unpin(cx) {
            Poll::Ready(Ok(val)) => Poll::Ready(Ok(val)),
            Poll::Ready(Err(val)) => Poll::Ready(Err(Error::new(Kind::Tungstenite, Some(val)))),
            Poll::Pending => Poll::Pending,
        }
    }
}
