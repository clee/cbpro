use crate::stream::{Pages, Paginate};
use chrono::{offset::TimeZone, DateTime};
use reqwest::Error;
use reqwest::{RequestBuilder, Client};
use sha2::Sha256;
use hmac::{Hmac, Mac};
use chrono::offset::Utc;
use serde::Serialize;
use serde_json::Value;

type HmacSha256 = Hmac<Sha256>;

#[derive(Copy, Clone)]
pub(super) struct Auth<'a> {
    pub key: &'a str,
    pub pass: &'a str,
    pub secret: &'a str
}

#[derive(Serialize)]
pub struct NoArgs;

#[derive(Serialize)]
pub struct BookArgs {
    pub level: Option<String>,
}

#[derive(Serialize)]
pub struct CandleArgs {
    pub start: Option<String>,
    pub end: Option<String>,
    pub granularity: Option<String>,
}

#[derive(Serialize)]
pub struct PaginateArgs {
    pub limit: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
}

pub struct ArgBuilder<'a, T: Serialize> {
    client: Client,
    request_builder: RequestBuilder,
    serializable: T,
    auth: Option<Auth<'a>>
}

impl<'a, T: Serialize> ArgBuilder<'a, T> {
    pub(super) fn new(client: Client, request_builder: reqwest::RequestBuilder, serializable: T, auth: Option<Auth<'a>>) -> Self {
        Self {
            client,
            request_builder,
            serializable,
            auth
        }
    }

    pub async fn json(self) -> Result<Value, Error> {
        if let Some(auth) = self.auth {
            let mut request = self.request_builder.query(&self.serializable).build()?;
            let Auth { key, pass, secret } = auth;

            let timestamp = Utc::now().timestamp().to_string();
            let method = request.method().as_str();
            let path = request.url().path();
            let hmac_key = base64::decode(&secret).unwrap();

            let message = if let Some(body) = request.body() {
                timestamp.clone() + method + path + std::str::from_utf8(body.as_bytes().unwrap()).unwrap()
            } else {
                timestamp.clone() + method + path
            };

            let mut mac = HmacSha256::new_varkey(&hmac_key).unwrap();
            mac.input(message.as_bytes());
            
            let signature = mac.result().code();
            let b64_signature = base64::encode(&signature);

            request.headers_mut().insert("CB-ACCESS-KEY", key.parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-PASSPHRASE", pass.parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-TIMESTAMP", (&timestamp[..]).parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-SIGN", (&b64_signature[..]).parse().unwrap());

            return self.client.execute(request).await?.json().await
        }
        self.request_builder
        .query(&self.serializable)
        .send()
        .await?
        .json()
        .await
    }
}

impl<'a> ArgBuilder<'a, BookArgs> {
    pub fn level(mut self, value: u32) -> Self {
        self.serializable.level = Some(value.to_string());
        self
    }
}

impl<'a> ArgBuilder<'a, PaginateArgs> {
    pub fn limit(mut self, value: u32) -> Self {
        self.serializable.limit = Some(value.to_string());
        self
    }

    pub fn before(mut self, value: &str) -> Self {
        self.serializable.before = Some(value.to_string());
        self.serializable.after = None;
        self
    }

    pub fn after(mut self, value: &str) -> Self {
        self.serializable.after = Some(value.to_string());
        self.serializable.before = None;
        self
    }

    pub fn paginate(self) -> Pages {
        Paginate::new(self.request_builder, self.serializable).pages()
    }
}

impl<'a> ArgBuilder<'a, CandleArgs> {
    pub fn range<Tz: TimeZone>(mut self, start: DateTime<Tz>, end: DateTime<Tz>) -> Self
    where
        Tz::Offset: core::fmt::Display,
    {
        self.serializable.start = Some(start.to_rfc3339());
        self.serializable.end = Some(end.to_rfc3339());
        self
    }
}
