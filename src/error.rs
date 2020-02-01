use std::fmt;
use std::error;

pub type Result<T> = std::result::Result<T, Error>;
pub(crate) type BoxError = Box<dyn error::Error + Send + Sync>;

#[derive(Debug)]
pub(super) enum Kind {
    BinanceError,
    SerdeUrlEncoded,
    Reqwest
}

#[derive(Debug)]
pub struct BinanceError {
    code: u16,
    message: String
}

impl BinanceError {
    pub(super) fn new<T: Into<String>>(code: u16, message: T) -> Self {
        BinanceError { code, message: message.into() }
    }
}

impl fmt::Display for BinanceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status Code: {}, Reason: {}", self.code, self.message)
    }
}

impl error::Error for BinanceError {
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
        let err_msg = match self.kind {
            Kind::BinanceError => "Invalid inputs provided",
            _ => "Wrong inputs provided",
        };

        write!(f, "{}", err_msg)
 
    }
}

impl From<BinanceError> for Error {
    fn from(error: BinanceError) -> Self {
        Error::new(Kind::BinanceError, Some(error))
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