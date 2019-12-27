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

pub const WEBSOCKET_FEED_URL: &'static str = "wss://ws-feed.pro.coinbase.com";

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
}

pub struct WebSocketFeed {
    inner: WebSocketStream<MaybeTlsStream<TcpStream>>

}

impl WebSocketFeed {
    pub async fn connect(url: &str) -> Result<Self, Error> {
    
        let url = url::Url::parse(url).unwrap();
        let (ws_stream, _) = connect_async(url).await?;

        Ok(Self {
            inner: ws_stream
        })
        
    }

    pub async fn subscribe<'a>(&mut self, product_ids: &'a [&'a str], channels: &'a [&'a str]) -> Result<(), Error> {
        let message = SubscribeMessage {type_: "subscribe", product_ids, channels};
        let message = serde_json::to_string(&message).unwrap();
        self.send(Message::Text(message)).await?;
        Ok(())
    }

    pub async fn unsubscribe<'a>(&mut self, product_ids: &'a [&'a str], channels: &'a [&'a str]) -> Result<(), Error> {
        let message = SubscribeMessage {type_: "unsubscribe", product_ids, channels};
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

impl Stream for WebSocketFeed {
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

impl Sink<Message> for WebSocketFeed {
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