use std::fmt;
use std::error;

pub type Result<T> = std::result::Result<T, Error>;
pub(crate) type BoxError = Box<dyn error::Error + Send + Sync>;

#[derive(Debug)]
pub(super) enum Kind {
    Client,
    Websocket,
    Coinbase,
    Other
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
        let mut builder = f.debug_struct("cbpro::Error");

        builder.field("kind", &self.kind);

        if let Some(ref source) = self.source {
            builder.field("source", source);
        }

        builder.finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self.kind {
            Kind::Client => "Client error",
            Kind::Websocket => "Websocket error",
            Kind::Coinbase => "Coinbase error",
            _ => "Sorry, something is wrong! Please Try Again!",
        };

        write!(f, "{}", err_msg)
 
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        self.source.as_ref().map(|e| &**e as _)
    }
}

impl From<CBError> for Error {
    fn from(error: CBError) -> Self {
        Error::new(Kind::Coinbase, Some(error))
    }
}

impl From<reqwest::Error> for Error {
    fn from(error: reqwest::Error) -> Self {
        Error::new(Kind::Client, Some(error))
    }
}

impl From<reqwest::header::ToStrError> for Error {
    fn from(error: reqwest::header::ToStrError) -> Self {
        Error::new(Kind::Client, Some(error))
    }
}

impl From<async_tungstenite::tungstenite::Error> for Error {
    fn from(error: async_tungstenite::tungstenite::Error) -> Self {
        Error::new(Kind::Websocket, Some(error))
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(error: serde_json::error::Error) -> Self {
        Error::new(Kind::Other, Some(error))
    }
}

impl From<serde_urlencoded::ser::Error> for Error {
    fn from(error: serde_urlencoded::ser::Error) -> Self {
        Error::new(Kind::Other, Some(error))
    }
}

impl From<std::str::Utf8Error> for Error {
    fn from(error: std::str::Utf8Error) -> Self {
        Error::new(Kind::Other, Some(error))
    }
}

impl From<base64::DecodeError> for Error {
    fn from(error: base64::DecodeError) -> Self {
        Error::new(Kind::Other, Some(error))
    }
}

impl From<hmac::crypto_mac::InvalidKeyLength> for Error {
    fn from(error: hmac::crypto_mac::InvalidKeyLength) -> Self {
        Error::new(Kind::Other, Some(error))
    }
}

#[derive(Debug)]
pub struct CBError {
    code: u16,
    message: String
}

impl CBError {
    pub(super) fn new(code: u16, message: String) -> Self {
        CBError { code, message }
    }
}

impl fmt::Display for CBError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Status Code: {}, Reason: {}", self.code, self.message)
    }
}

impl error::Error for CBError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}