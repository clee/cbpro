use core::pin::Pin;
use futures::{
    SinkExt,
    sink::Sink,
    stream::Stream,
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
use log::warn;

use tokio::net::TcpStream;
use tokio_tls::TlsStream;
use serde::Serialize;
use std::collections::HashMap;
use chrono::Utc;
use hmac::{ Hmac, Mac };
use sha2::Sha256;
use crate::client::Auth;
use crate::error::{ Error, Kind };

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
    inner: WebSocketStream<StreamSwitcher<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TokioAdapter<TokioAdapter<TcpStream>>>>>>,
    response: Response,
    auth: Option<Auth>
}

impl WebSocketFeed {
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::websocket::{WebSocketFeed, SANDBOX_FEED_URL, Channels};
    /// use futures::TryStreamExt;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut feed = WebSocketFeed::connect(SANDBOX_FEED_URL).await?;
    /// feed.subscribe(&["BTC-USD"], &[Channels::LEVEL2]).await?;
    ///
    /// while let Some(value) = feed.try_next().await? {
    ///     println!("{}", serde_json::to_string_pretty(&value).unwrap());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(url: &str) -> crate::error::Result<WebSocketFeed> {
    
        let url = url::Url::parse(url).unwrap();
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
    /// use futures::TryStreamExt;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut feed = WebSocketFeed::connect_auth("key".to_owned(), "pass".to_owned(), "secret".to_owned(), SANDBOX_FEED_URL).await?;
    /// feed.subscribe(&["BTC-USD"], &[Channels::LEVEL2]).await?;
    ///
    /// while let Some(value) = feed.try_next().await? {
    ///     println!("{}", serde_json::to_string_pretty(&value).unwrap());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect_auth(key: String, pass: String, secret: String, url: &str) -> crate::error::Result<WebSocketFeed> {
    
        let url = url::Url::parse(url).unwrap();
        let (ws_stream, res) = connect_async(url).await?;

        Ok(WebSocketFeed {
            inner: ws_stream,
            response: res,
            auth: Some(Auth { key, pass, secret })
        })
        
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
    pub fn get_ref(&self) -> &WebSocketStream<StreamSwitcher<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TokioAdapter<TokioAdapter<TcpStream>>>>>> {
        &self.inner
    }

    /// Returns a mutable reference to the inner stream.
    pub fn get_mut(&mut self) -> &mut WebSocketStream<StreamSwitcher<TokioAdapter<TcpStream>, TokioAdapter<TlsStream<TokioAdapter<TokioAdapter<TcpStream>>>>>> {
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
    type Item = crate::error::Result<serde_json::Value>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner).poll_next(cx) {
            Poll::Ready(Some(val)) => {
                let val = val?;
                if val.is_text() {
                    let value: serde_json::Value = serde_json::from_str(&val.into_text()?)?;
                    Poll::Ready(Some(Ok(value)))
                } else {
                    warn!("server responded with non text: {:?}", val);
                    Poll::Pending
                }
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending
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