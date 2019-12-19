use crate::stream::{Pages, Paginate};
use chrono::offset::Utc;
use chrono::{offset::TimeZone, DateTime};
use hmac::{Hmac, Mac};
use reqwest::Error;
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Client, Request,
};
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
pub struct CBParams<'a> {
    level: Option<&'a str>,
    start: Option<String>,
    end: Option<String>,
    granularity: Option<&'a str>,
    client_oid: Option<&'a str>,
    #[serde(rename(serialize = "type"))]
    order_type: Option<&'a str>,
    side: Option<&'a str>,
    product_id: Option<&'a str>,
    stp: Option<&'a str>,
    stop: Option<&'a str>,
    stop_price: Option<&'a str>,
    //limit
    price: Option<&'a str>,
    size: Option<&'a str>,
    time_in_force: Option<&'a str>,
    cancel_after: Option<&'a str>,
    post_only: Option<&'a str>,
    //market
    funds: Option<&'a str>,
    //paginate
    limit: Option<&'a str>,
    pub(super) before: Option<String>,
    after: Option<String>,
}

impl<'a> CBParams<'a> {
    pub(super) fn new() -> Self {
        Self {
            level: None,
            start: None,
            end: None,
            granularity: None,
            client_oid: None,
            order_type: None,
            side: None,
            product_id: None,
            stp: None,
            stop: None,
            stop_price: None,
            price: None,
            size: None,
            time_in_force: None,
            cancel_after: None,
            post_only: None,
            funds: None,
            limit: None,
            before: None,
            after: None,
        }
    }
}

pub trait Params<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a>;
    fn params(&self) -> &CBParams<'a>;
}

pub trait Paginated<'a> {
    fn set_limit(&mut self, value: &'a str);
    fn set_before(&mut self, value: String);
    fn set_after(&mut self, value: String);
}

pub trait Book<'a> {
    fn set_level(&mut self, value: &'a str);
}

pub trait Candle<'a> {
    fn set_start(&mut self, value: String);
    fn set_end(&mut self, value: String);
}
//////////////////////////////////////////////////

pub struct NoParams<'a> {
    params: CBParams<'a>,
}

impl<'a> NoParams<'a> {
    pub fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for NoParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

pub struct BookParams<'a> {
    params: CBParams<'a>,
}

impl<'a> BookParams<'a> {
    pub fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for BookParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Book<'a> for BookParams<'a> {
    fn set_level(&mut self, value: &'a str) {
        self.params_mut().level = Some(value);
    }
}

pub struct TradeParams<'a> {
    params: CBParams<'a>,
}

impl<'a> TradeParams<'a> {
    pub fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for TradeParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Paginated<'a> for TradeParams<'a> {
    fn set_limit(&mut self, value: &'a str) {
        self.params_mut().limit = Some(value);
    }
    fn set_before(&mut self, value: String) {
        self.params_mut().before = Some(value);
        self.params_mut().after = None;
    }
    fn set_after(&mut self, value: String) {
        self.params_mut().after = Some(value);
        self.params_mut().before = None;
    }
}

pub struct CandleParams<'a> {
    params: CBParams<'a>,
}

impl<'a> CandleParams<'a> {
    pub fn new(granularity: &'a str) -> Self {
        let mut params =  CBParams::new();
        params.granularity = Some(granularity);
        Self {
            params: params
        }
    }
}

impl<'a> Params<'a> for CandleParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Candle<'a> for CandleParams<'a> {
    fn set_start(&mut self, value: String) {
        self.params_mut().start = Some(value);
    }
    fn set_end(&mut self, value: String) {
        self.params_mut().end = Some(value);
    }
}
//////////////////

type HmacSha256 = Hmac<Sha256>;

pub(super) fn apply_query<T: Serialize>(req: &mut Request, query: &T) {
    {
        let url = req.url_mut();
        let mut pairs = url.query_pairs_mut();
        let serializer = serde_urlencoded::Serializer::new(&mut pairs);
        query.serialize(serializer).unwrap();
    }
    if let Some("") = req.url().query() {
        req.url_mut().set_query(None);
    }
}

pub(super) fn apply_json<T: Serialize>(req: &mut Request, json: &T) {
    let body = serde_json::to_vec(json).unwrap();
    req.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    *req.body_mut() = Some(body.into());
}

pub struct QueryBuilder<'a, T: Params<'a>> {
    client: Client,
    request: Request,
    query: T,
    auth: Option<Auth<'a>>,
}

impl<'a, T: Params<'a>> QueryBuilder<'a, T> {
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

    fn signed_request(&self) -> Request {
        let mut request = self.request.try_clone().unwrap();

        if let &reqwest::Method::POST = request.method() {
            apply_json(&mut request, self.query.params());
        } else {
            apply_query(&mut request, self.query.params());
        }

        if let Some(auth) = self.auth {
            let Auth { key, pass, secret } = auth;

            let timestamp = Utc::now().timestamp().to_string();
            let method = request.method().as_str();

            let path = if let Some(query) = request.url().query() {
                String::new() + request.url().path() + "?" + query
            } else {
                request.url().path().to_string()
            };

            let message = if let Some(body) = request.body() {
                timestamp.clone()
                    + method
                    + &path[..]
                    + std::str::from_utf8(body.as_bytes().unwrap()).unwrap()
            } else {
                timestamp.clone() + method + &path[..]
            };

            let hmac_key = base64::decode(secret).unwrap();
            let mut mac = HmacSha256::new_varkey(&hmac_key).unwrap();
            mac.input(message.as_bytes());
            let signature = mac.result().code();
            let b64_signature = base64::encode(&signature);

            request.headers_mut().insert("CB-ACCESS-KEY", key.parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-PASSPHRASE", pass.parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-TIMESTAMP", (&timestamp[..]).parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-SIGN", (&b64_signature[..]).parse().unwrap());
        }
        request
    }

    pub async fn json(self) -> Result<Value, Error> {
        let request = self.signed_request();
        println!("{:?}", &request.url().query());
        self.client.execute(request).await?.json().await
    }
}

impl<'a, T: Params<'a> + Book<'a>> QueryBuilder<'a, T> {
    pub fn level(mut self, value: &'a str) -> Self {
        self.query.set_level(value);
        self
    }
}

impl<'a, T: Params<'a> + Paginated<'a> + Send + 'a> QueryBuilder<'a, T> {
    pub fn limit(mut self, value: &'a str) -> Self {
        self.query.set_limit(value);
        self
    }

    pub fn before(mut self, value: &'a str) -> Self {
        self.query.set_before(value.to_string());
        self
    }

    pub fn after(mut self, value: &'a str) -> Self {
        self.query.set_after(value.to_string());
        self
    }

    pub fn paginate(self) -> Pages<'a> {
        Paginate::new(self.client.clone(), self.signed_request(), self.query).pages()
    }
}

impl<'a, T: Params<'a> + Candle<'a>> QueryBuilder<'a, T> {
    pub fn range<Tz: TimeZone>(mut self, start: DateTime<Tz>, end: DateTime<Tz>) -> Self 
    where
        Tz::Offset: core::fmt::Display,
    {
        self.query.set_start(start.to_rfc3339());
        self.query.set_end(end.to_rfc3339());
        self
    }
}