use core::pin::Pin;
use futures::{
    SinkExt,
    sink::Sink,
    stream::Stream,
    task::{Context, Poll},
};
use async_tungstenite::{WebSocketStream, MaybeTlsStream};
use async_tungstenite::tungstenite::{
    Message, error::Error, protocol::{
        CloseFrame, 
        frame::coding::CloseCode
    }
};
use async_std::net::TcpStream;
use async_tungstenite::connect_async;
use serde::Serialize;

use std::collections::HashMap;
use chrono::Utc;
use hmac::{Hmac, Mac};
use sha2::Sha256;
use crate::client::Auth;

pub const WEBSOCKET_FEED_URL: &'static str = "wss://ws-feed-public.sandbox.pro.coinbase.com";

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

pub struct WebSocketFeed<'a> {
    inner: WebSocketStream<MaybeTlsStream<TcpStream>>,
    auth: Option<Auth<'a>>
}

impl<'a> WebSocketFeed<'a> {
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::websocket::{WebSocketFeed, WEBSOCKET_FEED_URL, Channels};
    /// use futures::TryStreamExt;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut feed = WebSocketFeed::connect(WEBSOCKET_FEED_URL).await?;
    /// feed.subscribe(&["BTC-USD"], &[Channels::LEVEL2]).await?;
    ///
    /// while let Some(value) = feed.try_next().await? {
    ///     println!("{}", serde_json::to_string_pretty(&value).unwrap());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect(url: &str) -> Result<WebSocketFeed<'a>, Error> {
    
        let url = url::Url::parse(url).unwrap();
        let (ws_stream, _) = connect_async(url).await?;

        Ok(WebSocketFeed {
            inner: ws_stream,
            auth: None
        })
        
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::websocket::{WebSocketFeed, WEBSOCKET_FEED_URL, Channels};
    /// use futures::TryStreamExt;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut feed = WebSocketFeed::connect_auth("key", "pass", "secret", WEBSOCKET_FEED_URL).await?;
    /// feed.subscribe(&["BTC-USD"], &[Channels::LEVEL2]).await?;
    ///
    /// while let Some(value) = feed.try_next().await? {
    ///     println!("{}", serde_json::to_string_pretty(&value).unwrap());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn connect_auth(key: &'a str, pass: &'a str, secret: &'a str, url: &str) -> Result<WebSocketFeed<'a>, Error> {
    
        let url = url::Url::parse(url).unwrap();
        let (ws_stream, _) = connect_async(url).await?;

        Ok(WebSocketFeed {
            inner: ws_stream,
            auth: Some(Auth { key, pass, secret })
        })
        
    }

    pub async fn subscribe(&mut self, product_ids: &'a [&'a str], channels: &'a [&'a str]) -> Result<(), Error> {
        let auth = match self.auth {
            Some(auth) => {
                let timestamp = Utc::now().timestamp().to_string();
                let message = timestamp.clone() + "GET" + "/users/self/verify";
        
                let hmac_key = base64::decode(auth.secret).unwrap();
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

    pub async fn unsubscribe(&mut self, product_ids: &'a [&'a str], channels: &'a [&'a str]) -> Result<(), Error> {
        let message = SubscribeMessage {type_: "unsubscribe", product_ids, channels, auth: None};
        let message = serde_json::to_string(&message).unwrap();
        self.send(Message::Text(message)).await?;
        Ok(())
    }

    pub fn get_ref(&self) -> &WebSocketStream<MaybeTlsStream<TcpStream>> {
        &self.inner
    }

    pub fn get_mut(&mut self) -> &mut WebSocketStream<MaybeTlsStream<TcpStream>> {
        &mut self.inner
    }

    pub async fn close(mut self) -> Result<(), Error> {
        let close_frame = CloseFrame {code: CloseCode::Normal, reason: "closed manually".to_string().into()};
        self.inner.close(Some(close_frame)).await
    }
}

impl<'a> Stream for WebSocketFeed<'a> {
    type Item = Result<serde_json::Value, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.inner).poll_next(cx) {
            Poll::Ready(Some(val)) => {
                let text = val?.into_text()?;
                let value: serde_json::Value = serde_json::from_str(&text).unwrap();
                Poll::Ready(Some(Ok(value)))
            },
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending
        }
    }
}

impl<'a> Sink<Message> for WebSocketFeed<'a> {
    type Error = Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_ready(cx)
    }

    fn start_send(mut self: Pin<&mut Self>, item: Message) -> Result<(), Self::Error> {
        Pin::new(&mut self.inner).start_send(item)
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_flush(cx)
    }
       
    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Result<(), Self::Error>> {
        Pin::new(&mut self.inner).poll_close(cx)
    }
}