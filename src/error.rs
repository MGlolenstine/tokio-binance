use std::fmt;
use std::error;
use async_tungstenite::tungstenite::protocol::frame::coding::CloseCode;

pub type Result<T> = std::result::Result<T, Error>;
pub(crate) type BoxError = Box<dyn error::Error + Send + Sync>;

#[derive(Debug)]
pub(super) enum Kind {
    Binance,
    SerdeUrlEncoded,
    Reqwest,
    Tungstenite,
    SerdeJson,
    Hmac,
    Url,
}

#[derive(Debug)]
pub struct WsCloseError {
    code: CloseCode,
    reason: String,
}

impl WsCloseError {
    pub(super) fn new<T: Into<String>>(code: CloseCode, reason: T) -> Self {
        WsCloseError { code, reason: reason.into() }
    }
}

impl fmt::Display for WsCloseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}

impl error::Error for WsCloseError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub struct ClientError {
    code: u16,
    reason: String,
    message: String
}

impl ClientError {
    pub(super) fn new<T: Into<String>>(code: u16, reason: T, message: T) -> Self {
        ClientError { code, reason: reason.into(), message: message.into() }
    }
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("ClientError");

        builder.field("code", &self.code);
        builder.field("reason", &self.reason);
        builder.finish()
    }
}

impl error::Error for ClientError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

pub struct Error {
    kind: Kind,
    source: Option<BoxError>
}

impl Error {
    pub(super) fn new<E>(kind: Kind, source: Option<E>) -> Self 
    where
        E: Into<BoxError>,
    {
        Error {
            kind,
            source: source.map(Into::into)
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut builder = f.debug_struct("tokio-binance::Error");

        builder.field("kind", &self.kind);

        if let Some(ref source) = self.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(ref source) = self.source {
            write!(f, "{:?}: {}", self.kind, source)
        } else {
           write!(f, "No source for this error") 
        }
    }
}

impl From<ClientError> for Error {
    fn from(error: ClientError) -> Self {
        Error::new(Kind::Binance, Some(error))
    }
}

impl From<WsCloseError> for Error {
    fn from(error: WsCloseError) -> Self {
        Error::new(Kind::Binance, Some(error))
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::new(Kind::Reqwest, Some(error))
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(error: serde_urlencoded::ser::Error) -> Self {
        Error::new(Kind::SerdeUrlEncoded, Some(error))
    }
}

impl From<async_tungstenite::tungstenite::Error> for Error {
    fn from(error: async_tungstenite::tungstenite::Error) -> Self {
        Error::new(Kind::Tungstenite, Some(error))
    }
}

impl From<url::ParseError> for Error {
    fn from(error: url::ParseError) -> Self {
        Error::new(Kind::Url, Some(error))
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::new(Kind::SerdeJson, Some(error))
    }
}

impl From<hmac::crypto_mac::InvalidKeyLength> for Error {
    fn from(error: hmac::crypto_mac::InvalidKeyLength) -> Self {
        Error::new(Kind::Hmac, Some(error))
    }
}