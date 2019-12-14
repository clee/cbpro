use crate::stream::{Pages, Paginate};
use chrono::offset::Utc;
use chrono::{offset::TimeZone, DateTime};
use hmac::{Hmac, Mac};
use reqwest::Error;
use reqwest::{Client, Request, header::{HeaderValue, CONTENT_TYPE}};
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

#[derive(Serialize)]
pub struct LimitOrderQuery {
    pub client_oid: Option<String>,

    #[serde(rename(serialize = "type"))]
    pub order_type: Option<String>,

    pub side: Option<String>,
    pub product_id: Option<String>,
    pub stp: Option<String>,
    pub stop: Option<String>,
    pub stop_price: Option<String>,

    //limit
    pub price: Option<String>,
    pub size: Option<String>,
    pub time_in_force: Option<String>,
    pub cancel_after: Option<String>,
    pub post_only: Option<String>,
}

#[derive(Serialize)]
pub struct MarketOrderQuery {
    pub client_oid: Option<String>,

    #[serde(rename(serialize = "type"))]
    pub order_type: Option<String>,

    pub side: Option<String>,
    pub product_id: Option<String>,
    pub stp: Option<String>,
    pub stop: Option<String>,
    pub stop_price: Option<String>,

    //market
    pub size: Option<String>,
    pub funds: Option<String>,
}

type HmacSha256 = Hmac<Sha256>;

pub(super) fn apply_query<T: Serialize>(req: &mut Request, query: &T) {
    let url = req.url_mut();
    let mut pairs = url.query_pairs_mut();
    let serializer = serde_urlencoded::Serializer::new(&mut pairs);
    query.serialize(serializer).unwrap();
}

pub(super) fn apply_json<T: Serialize>(req: &mut Request, json: &T) {
    let body = serde_json::to_vec(json).unwrap();
    req.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    *req.body_mut() = Some(body.into());
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

        if let &reqwest::Method::POST = request.method() {
            apply_json(&mut request, &self.query);
        } else {
            apply_query(&mut request, &self.query);
        }

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

            request.headers_mut().insert("CB-ACCESS-KEY", key.parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-PASSPHRASE", pass.parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-TIMESTAMP", (&timestamp[..]).parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-SIGN", (&b64_signature[..]).parse().unwrap());

            return request
        }

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
        Paginate::new(self.client.clone(), self.sign_request(), self.query).pages()
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

impl<'a> QueryBuilder<'a, LimitOrderQuery> {
    pub fn client_oid(mut self, value: &'a str) -> Self {
        self.query.client_oid = Some(value.to_string());
        self
    }

    pub fn order_type(mut self, value: &str) -> Self {
        self.query.order_type = Some(value.to_string());
        self
    }

    pub fn stp(mut self, value: &str) -> Self {
        self.query.stp = Some(value.to_string());
        self
    }

    pub fn stop_price(mut self, value: f64) -> Self {
        self.query.stop_price = Some(value.to_string());
        if let Some(ref value) = self.query.side {
            if value == "buy" {
                self.query.stop = Some("entry".to_string())
            } else {
                self.query.stop = Some("loss".to_string())
            }
        }
        self
    }

    pub fn time_in_force(mut self, value: &str) -> Self {
        self.query.time_in_force = Some(value.to_string());
        self
    }

    pub fn cancel_after(mut self, value: &str) -> Self {
        self.query.cancel_after = Some(value.to_string());
        self.query.time_in_force = Some("GTT".to_string());
        self
    }

    pub fn post_only(mut self, value: bool) -> Self {
        self.query.post_only = Some(value.to_string());
        self
    }
}