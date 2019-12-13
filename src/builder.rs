use crate::stream::{Pages, Paginate};
use chrono::offset::Utc;
use chrono::{offset::TimeZone, DateTime};
use hmac::{Hmac, Mac};
use reqwest::Error;
use reqwest::{Client, Request};
use serde::Serialize;
use serde_json::Value;
use sha2::Sha256;

#[derive(Copy, Clone)]
pub(super) struct Auth<'a> {
    pub key: &'a str,
    pub pass: &'a str,
    pub secret: &'a str,
}

#[derive(Serialize)]
pub struct EmptyQuery;

#[derive(Serialize)]
pub struct BookQuery {
    pub level: Option<String>,
}

#[derive(Serialize)]
pub struct CandleQuery {
    pub start: Option<String>,
    pub end: Option<String>,
    pub granularity: Option<String>,
}

#[derive(Serialize)]
pub struct PaginateQuery {
    pub limit: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
}

type HmacSha256 = Hmac<Sha256>;

pub(super) fn apply_query<T: Serialize>(req: &mut Request, query: &T) {
    // This function only exist because reqwest::RequestBuilder::new() is private (insert sad face)
    let url = req.url_mut();
    let mut pairs = url.query_pairs_mut();
    let serializer = serde_urlencoded::Serializer::new(&mut pairs);
    query.serialize(serializer).unwrap();
}

pub struct QueryBuilder<'a, T: Serialize> {
    client: Client,
    request: Request,
    query: T,
    auth: Option<Auth<'a>>,
}

impl<'a, T: Serialize> QueryBuilder<'a, T> {
    pub(super) fn new(
        client: Client,
        request: Request,
        query: T,
        auth: Option<Auth<'a>>,
    ) -> Self {
        Self {
            client,
            request,
            query,
            auth,
        }
    }

    fn sign_request(&self) -> Request {
        let mut request = self.request.try_clone().unwrap();

        if let Some(auth) = self.auth {
            let Auth { key, pass, secret } = auth;

            let timestamp = Utc::now().timestamp().to_string();
            let method = request.method().as_str();
            let path = request.url().path();
            let hmac_key = base64::decode(secret).unwrap();

            let message = if let Some(body) = request.body() {
                timestamp.clone()
                    + method
                    + path
                    + std::str::from_utf8(body.as_bytes().unwrap()).unwrap()
            } else {
                timestamp.clone() + method + path
            };

            let mut mac = HmacSha256::new_varkey(&hmac_key).unwrap();
            mac.input(message.as_bytes());
            let signature = mac.result().code();
            let b64_signature = base64::encode(&signature);

            request
                .headers_mut()
                .insert("CB-ACCESS-KEY", key.parse().unwrap());
            request
                .headers_mut()
                .insert("CB-ACCESS-PASSPHRASE", pass.parse().unwrap());
            request
                .headers_mut()
                .insert("CB-ACCESS-TIMESTAMP", (&timestamp[..]).parse().unwrap());
            request
                .headers_mut()
                .insert("CB-ACCESS-SIGN", (&b64_signature[..]).parse().unwrap());

            apply_query(&mut request, &self.query);
            return request
        }

        apply_query(&mut request, &self.query);
        request
    }

    pub async fn json(self) -> Result<Value, Error> {
        self.client.execute(self.sign_request()).await?.json().await
    }
}

impl<'a> QueryBuilder<'a, BookQuery> {
    pub fn level(mut self, value: u32) -> Self {
        self.query.level = Some(value.to_string());
        self
    }
}

impl<'a> QueryBuilder<'a, PaginateQuery> {
    pub fn limit(mut self, value: u32) -> Self {
        self.query.limit = Some(value.to_string());
        self
    }

    pub fn before(mut self, value: &str) -> Self {
        self.query.before = Some(value.to_string());
        self.query.after = None;
        self
    }

    pub fn after(mut self, value: &str) -> Self {
        self.query.after = Some(value.to_string());
        self.query.before = None;
        self
    }

    pub fn paginate(self) -> Pages {
        let request = self.sign_request();
        Paginate::new(self.client, request, self.query).pages()
    }
}

impl<'a> QueryBuilder<'a, CandleQuery> {
    pub fn range<Tz: TimeZone>(mut self, start: DateTime<Tz>, end: DateTime<Tz>) -> Self
    where
        Tz::Offset: core::fmt::Display,
    {
        self.query.start = Some(start.to_rfc3339());
        self.query.end = Some(end.to_rfc3339());
        self
    }
}
