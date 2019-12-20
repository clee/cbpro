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
    level: Option<i32>,
    start: Option<String>,
    end: Option<String>,
    granularity: Option<i32>,
    client_oid: Option<&'a str>,
    #[serde(rename(serialize = "type"))]
    order_type: Option<&'a str>,
    side: Option<&'a str>,
    product_id: Option<&'a str>,
    stp: Option<&'a str>,
    stop: Option<&'a str>,
    stop_price: Option<f64>,
    //limit
    price: Option<f64>,
    size: Option<f64>,
    time_in_force: Option<&'a str>,
    cancel_after: Option<&'a str>,
    post_only: Option<bool>,
    //market
    funds: Option<f64>,
    //paginate
    limit: Option<i32>,
    pub(super) before: Option<i32>,
    after: Option<i32>,
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

pub trait Product<'a> {
    fn set_product_id(&mut self, value: &'a str);
}

pub trait Paginated<'a> {
    fn set_limit(&mut self, value: i32);
    fn set_before(&mut self, value: i32);
    fn set_after(&mut self, value: i32);
}

pub trait Book<'a> {
    fn set_level(&mut self, value: i32);
}

pub trait Candle<'a> {
    fn set_start(&mut self, value: String);
    fn set_end(&mut self, value: String);
}

pub trait Order<'a> {
    fn set_client_oid(&mut self, value: &'a str);
}

pub trait Limit<'a> {
    fn set_stp(&mut self, value: &'a str);
    fn set_stop(&mut self, value: &'a str);
    fn set_stop_price(&mut self, value: f64);
    fn set_time_in_force(&mut self, value: &'a str);
    fn set_cancel_after(&mut self, value: &'a str);
    fn set_post_only(&mut self, value: bool);
}

pub trait Market<'a> {
    fn set_funds(&mut self, value: f64);
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

pub struct ProductParams<'a> {
    params: CBParams<'a>,
}

impl<'a> ProductParams<'a> {
    pub fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for ProductParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Product<'a> for ProductParams<'a> {
    fn set_product_id(&mut self, value: &'a str) {
        self.params_mut().product_id = Some(value);
    }
}

pub struct ListOrderParams<'a> {
    params: CBParams<'a>,
}

impl<'a> ListOrderParams<'a> {
    pub fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for ListOrderParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Product<'a> for ListOrderParams<'a> {
    fn set_product_id(&mut self, value: &'a str) {
        self.params_mut().product_id = Some(value);
    }
}

impl<'a> Paginated<'a> for ListOrderParams<'a> {
    fn set_limit(&mut self, value: i32) {
        self.params_mut().limit = Some(value);
    }
    fn set_before(&mut self, value: i32) {
        self.params_mut().before = Some(value);
        self.params_mut().after = None;
    }
    fn set_after(&mut self, value: i32) {
        self.params_mut().after = Some(value);
        self.params_mut().before = None;
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
    fn set_level(&mut self, value: i32) {
        self.params_mut().level = Some(value);
    }
}

pub struct PaginateParams<'a> {
    params: CBParams<'a>,
}

impl<'a> PaginateParams<'a> {
    pub fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for PaginateParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Paginated<'a> for PaginateParams<'a> {
    fn set_limit(&mut self, value: i32) {
        self.params_mut().limit = Some(value);
    }
    fn set_before(&mut self, value: i32) {
        self.params_mut().before = Some(value);
        self.params_mut().after = None;
    }
    fn set_after(&mut self, value: i32) {
        self.params_mut().after = Some(value);
        self.params_mut().before = None;
    }
}

pub struct CandleParams<'a> {
    params: CBParams<'a>,
}

impl<'a> CandleParams<'a> {
    pub fn new(granularity: i32) -> Self {
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


pub struct LimitOrderParams<'a> {
    params: CBParams<'a>,
}

impl<'a> LimitOrderParams<'a> {
    pub fn new(product_id: &'a str, side: &'a str, price: f64, size: f64) -> Self {
        let mut params =  CBParams::new();
        params.order_type = Some("limit");
        params.product_id = Some(product_id);
        params.side = Some(side);
        params.price = Some(price);
        params.size = Some(size);
        Self {
            params: params
        }
    }
}

impl<'a> Params<'a> for LimitOrderParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Order<'a> for LimitOrderParams<'a> {
    fn set_client_oid(&mut self, value: &'a str) {
        self.params_mut().client_oid = Some(value);
    }
}

impl<'a> Limit<'a> for LimitOrderParams<'a> {
    fn set_stp(&mut self, value: &'a str) {
        self.params_mut().stp = Some(value);
    }
    fn set_stop(&mut self, value: &'a str) {
        self.params_mut().stop = Some(value);
    }
    fn set_stop_price(&mut self, value: f64) {
        self.params_mut().stop_price = Some(value);
    }
    fn set_time_in_force(&mut self, value: &'a str) {
        self.params_mut().time_in_force = Some(value);
    }
    fn set_cancel_after(&mut self, value: &'a str) {
        self.params_mut().cancel_after = Some(value);
    }
    fn set_post_only(&mut self, value: bool) {
        self.params_mut().post_only = Some(value);
    }
}

pub struct MarketOrderParams<'a> {
    params: CBParams<'a>,
}

impl<'a> MarketOrderParams<'a> {
    pub fn new(product_id: &'a str, side: &'a str, size: f64) -> Self {
        let mut params =  CBParams::new();
        params.order_type = Some("market");
        params.product_id = Some(product_id);
        params.side = Some(side);
        params.size = Some(size);
        Self {
            params: params
        }
    }
}

impl<'a> Params<'a> for MarketOrderParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Order<'a> for MarketOrderParams<'a> {
    fn set_client_oid(&mut self, value: &'a str) {
        self.params_mut().client_oid = Some(value);
    }
}

impl<'a> Market<'a> for MarketOrderParams<'a> {
    fn set_funds(&mut self, value: f64) {
        self.params_mut().funds = Some(value);
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

impl<'a, T: Params<'a> + Product<'a>> QueryBuilder<'a, T> {
    pub fn product_id(mut self, value: &'a str) -> Self {
        self.query.set_product_id(value);
        self
    }
}

impl<'a, T: Params<'a> + Book<'a>> QueryBuilder<'a, T> {
    pub fn level(mut self, value: i32) -> Self {
        self.query.set_level(value);
        self
    }
}

impl<'a, T: Params<'a> + Paginated<'a> + Send + Unpin + 'a> QueryBuilder<'a, T> {
    pub fn limit(mut self, value: i32) -> Self {
        self.query.set_limit(value);
        self
    }

    pub fn before(mut self, value: i32) -> Self {
        self.query.set_before(value);
        self
    }

    pub fn after(mut self, value: i32) -> Self {
        self.query.set_after(value);
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

impl<'a, T: Params<'a> + Order<'a>> QueryBuilder<'a, T> {
    pub fn client_oid(mut self, value: &'a str) -> Self {
        self.query.set_client_oid(value);
        self
    }
}

impl<'a, T: Params<'a> + Market<'a>> QueryBuilder<'a, T> {
    pub fn funds(mut self, value: f64) -> Self {
        self.query.set_funds(value);
        self
    }
}

impl<'a, T: Params<'a> + Limit<'a>> QueryBuilder<'a, T> {
    pub fn stp(mut self, value: &'a str) -> Self {
        self.query.set_stp(value);
        self
    }

    pub fn stop_price(mut self, value: f64) -> Self {
        self.query.set_stop_price(value);
        if let Some(value) = self.query.params().side {
            if value == "buy" {
                self.query.set_stop("entry");
            } else {
                self.query.set_stop("loss");
            }
        }
        self
    }

    pub fn time_in_force(mut self, value: &'a str) -> Self {
        self.query.set_time_in_force(value);
        self
    }

    pub fn cancel_after(mut self, value: &'a str) -> Self {
        self.query.set_cancel_after(value);
        self.query.set_time_in_force("GTT");
        self
    }

    pub fn post_only(mut self, value: bool) -> Self {
        self.query.set_post_only(value);
        self
    }
}