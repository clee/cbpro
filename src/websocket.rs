use core::pin::Pin;
use futures::{
    SinkExt,
    sink::Sink,
    stream::{Stream, TryStreamExt},
    task::{Context, Poll},
};
use async_tungstenite::{
    WebSocketStream,
    tokio::{
        connect_async,
        TokioAdapter
    },
    tungstenite::{
        Message,
        protocol::{
            CloseFrame,
            frame::coding::CloseCode,
        },
        handshake::client::Response
    },
    stream::Stream as StreamSwitcher
};

use tokio::net::TcpStream;
use tokio_native_tls::TlsStream;
use serde::Serialize;
use std::collections::HashMap;
use chrono::Utc;
use hmac::{ Hmac, Mac };
use sha2::Sha256;
use crate::client::Auth;
use crate::error::{Error, Kind, WsCloseError};
use serde::de::DeserializeOwned;

/// wss://ws-feed-public.sandbox.pro.coinbase.com
pub const SANDBOX_FEED_URL: &'static str = "wss://ws-feed-public.sandbox.pro.coinbase.com";
/// wss://ws-feed.pro.coinbase.com
pub const MAIN_FEED_URL: &'static str = "wss://ws-feed.pro.coinbase.com";

/// Channel constants
pub struct Channels;

impl Channels {
    pub const TICKER: &'static str = "ticker";
    pub const HEARTBEAT: &'static str = "heartbeat";
    pub const STATUS: &'static str = "status";
    pub const LEVEL2: &'static str = "level2";
    pub const USER: &'static str = "user";
    pub const MATCHES: &'static str = "matches";
    pub const FULL: &'static str = "full";
}

#[derive(Serialize)]
struct SubscribeMessage<'a> {
    #[serde(rename(serialize = "type"))]
    type_: &'a str,
    product_ids: &'a [&'a str],
    channels: &'a [&'a str],

    #[serde(flatten)]
    auth: Option<HashMap<&'a str, String>>,
}

type HmacSha256 = Hmac<Sha256>;

/// Stream with private or public access to Coinbase's Websocket Feed
pub struct WebSocketFeed {
    inner: WebSocketStream<StreamSwitcher<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>>,
    response: Response,
    auth: Option<Auth>
}

impl WebSocketFeed {
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::websocket::{WebSocketFeed, SANDBOX_FEED_URL, Channels};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut feed = WebSocketFeed::connect(SANDBOX_FEED_URL).await?;
    /// feed.subscribe(&["BTC-USD"], &[Channels::LEVEL2]).await?;
    ///
    /// while let Some(value) = feed.json::<serde_json::Value>().await? {
    ///     println!("{}", serde_json::to_string_pretty(&value).unwrap());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect<U: Into<String>>(url: U) -> crate::error::Result<WebSocketFeed> {

        let url = url::Url::parse(&url.into()).unwrap();
        let (ws_stream, res) = connect_async(url).await?;

        Ok(WebSocketFeed {
            inner: ws_stream,
            response: res,
            auth: None
        })

    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::websocket::{WebSocketFeed, SANDBOX_FEED_URL, Channels};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut feed = WebSocketFeed::connect_auth("<key>", "<pass>", "<secret>", SANDBOX_FEED_URL).await?;
    /// feed.subscribe(&["BTC-USD"], &[Channels::LEVEL2]).await?;
    ///
    /// while let Some(value) = feed.json::<serde_json::Value>().await? {
    ///     println!("{}", serde_json::to_string_pretty(&value).unwrap());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect_auth<K, P, S, U>(key: K, pass: P, secret: S, url: U) -> crate::error::Result<WebSocketFeed>
    where
        K: Into<String>,
        P: Into<String>,
        S: Into<String>,
        U: Into<String>,
    {
        let url = url::Url::parse(&url.into()).unwrap();
        let (ws_stream, res) = connect_async(url).await?;

        Ok(WebSocketFeed {
            inner: ws_stream,
            response: res,
            auth: Some(
                Auth {
                key: key.into(),
                pass: pass.into(),
                secret: secret.into()
            }
        )
        })
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

    /// Subscribe to a list of channels and products.
    pub async fn subscribe(&mut self, product_ids: &[&str], channels: &[&str]) -> crate::error::Result<()> {
        let auth = match self.auth {
            Some(ref auth) => {
                let timestamp = Utc::now().timestamp().to_string();
                let message = timestamp.clone() + "GET" + "/users/self/verify";

                let hmac_key = base64::decode(&auth.secret).unwrap();
                let mut mac = HmacSha256::new_varkey(&hmac_key).unwrap();
                mac.input(message.as_bytes());
                let signature = mac.result().code();
                let b64_signature = base64::encode(&signature);

                let mut map = HashMap::new();

                map.insert("key", auth.key.to_string());
                map.insert("passphrase", auth.pass.to_string());
                map.insert("timestamp", timestamp);
                map.insert("signature", b64_signature);
                Some(map)
            },
            None => None
        };
        let message = SubscribeMessage {type_: "subscribe", product_ids, channels, auth};
        let message = serde_json::to_string(&message).unwrap();
        self.send(Message::Text(message)).await?;

        Ok(())
    }

    /// Unsubscribe to a list of channels and products.
    pub async fn unsubscribe(&mut self, product_ids: &[&str], channels: &[&str]) -> crate::error::Result<()> {
        let message = SubscribeMessage {type_: "unsubscribe", product_ids, channels, auth: None};
        let message = serde_json::to_string(&message).unwrap();
        self.send(Message::Text(message)).await?;
        Ok(())
    }

    /// Returns a shared reference to the inner stream.
    pub fn get_ref(&self) -> &WebSocketStream<StreamSwitcher<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>> {
        &self.inner
    }

    /// Returns a mutable reference to the inner stream.
    pub fn get_mut(&mut self) -> &mut WebSocketStream<StreamSwitcher<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TcpStream>>>> {
        &mut self.inner
    }

    /// Returns a shared reference to the feed response.
    pub fn response(&self) -> &Response {
        &self.response
    }

    /// Sends a close frame
    pub async fn close(mut self) -> crate::error::Result<()> {
        let close_frame = CloseFrame {code: CloseCode::Normal, reason: "closed manually".to_string().into()};
        self.inner.close(Some(close_frame)).await?;
        Ok(())
    }
}

impl Stream for WebSocketFeed {
    type Item = crate::error::Result<Message>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner).poll_next(cx) {
            Poll::Ready(Some(val)) => Poll::Ready(Some(Ok(val?))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl Sink<Message> for WebSocketFeed {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match Pin::new(&mut self.inner).poll_ready(cx) {
            Poll::Ready(Ok(val)) => Poll::Ready(Ok(val)),
            Poll::Ready(Err(val)) => Poll::Ready(Err(Error::new(Kind::Tungstenite, Some(val)))),
            Poll::Pending => Poll::Pending
        }
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        match Pin::new(&mut self.inner).start_send(item) {
            Ok(val) => Ok(val),
            Err(val) => Err(Error::new(Kind::Tungstenite, Some(val))),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match Pin::new(&mut self.inner).poll_flush(cx) {
            Poll::Ready(Ok(val)) => Poll::Ready(Ok(val)),
            Poll::Ready(Err(val)) => Poll::Ready(Err(Error::new(Kind::Tungstenite, Some(val)))),
            Poll::Pending => Poll::Pending
        }
    }
       
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        match Pin::new(&mut self.inner).poll_close(cx) {
            Poll::Ready(Ok(val)) => Poll::Ready(Ok(val)),
            Poll::Ready(Err(val)) => Poll::Ready(Err(Error::new(Kind::Tungstenite, Some(val)))),
            Poll::Pending => Poll::Pending
        }
    }
}
