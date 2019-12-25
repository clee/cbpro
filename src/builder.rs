use crate::{
    stream::{Pages, Paginated},
    Auth, QTY, RPT,
};
use actix_web::client::ClientRequest;
use chrono::{
    offset::{TimeZone, Utc},
    DateTime,
};
use hmac::{Hmac, Mac};
use serde::Serialize;
use serde_json::Value;
use sha2::Sha256;

#[derive(Serialize)]
pub struct CBParams<'a> {
    level: Option<i32>,
    start: Option<String>,
    end: Option<String>,
    granularity: Option<i32>,
    client_oid: Option<&'a str>,
    pub(super) order_id: Option<&'a str>,
    #[serde(rename(serialize = "type"))]
    type_: Option<&'a str>,
    //limit
    side: Option<&'a str>,
    pub(super) product_id: Option<&'a str>,
    stp: Option<&'a str>,
    stop: Option<&'a str>,
    stop_price: Option<f64>,
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
    //deposits/withdrawals
    pub(super) amount: Option<f64>,
    pub(super) currency: Option<&'a str>,
    pub(super) payment_method_id: Option<&'a str>,
    pub(super) coinbase_account_id: Option<&'a str>,
    pub(super) crypto_address: Option<&'a str>,
    pub(super) destination_tag: Option<&'a str>,
    pub(super) no_destination_tag: Option<bool>,
    //conversion
    pub(super) from: Option<&'a str>,
    pub(super) to: Option<&'a str>,
    //report
    start_date: Option<String>,
    end_date: Option<String>,
    format: Option<&'a str>,
    email: Option<&'a str>,
    account_id: Option<&'a str>,
}

impl<'a> CBParams<'a> {
    pub(super) fn new() -> Self {
        Self {
            level: None,
            start: None,
            end: None,
            granularity: None,
            client_oid: None,
            order_id: None,
            type_: None,
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
            amount: None,
            currency: None,
            payment_method_id: None,
            coinbase_account_id: None,
            crypto_address: None,
            destination_tag: None,
            no_destination_tag: None,
            from: None,
            to: None,
            start_date: None,
            end_date: None,
            format: None,
            email: None,
            account_id: None,
        }
    }
}

pub trait Params<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a>;
    fn params(&self) -> &CBParams<'a>;
}

pub trait ProductID<'a> {
    fn set_product_id(&mut self, value: &'a str);
}

pub trait Paginate<'a> {
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

pub trait ClientOID<'a> {
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

pub trait Report<'a> {
    fn set_format(&mut self, value: &'a str);
    fn set_email(&mut self, value: &'a str);
}
//////////////////////////////////////////////////

pub struct NoParams<'a> {
    params: CBParams<'a>,
}

impl<'a> NoParams<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new(),
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

pub struct CancelParams<'a> {
    params: CBParams<'a>,
}

impl<'a> CancelParams<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new(),
        }
    }
}

impl<'a> Params<'a> for CancelParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> ProductID<'a> for CancelParams<'a> {
    fn set_product_id(&mut self, value: &'a str) {
        self.params_mut().product_id = Some(value);
    }
}

pub struct ListOrderParams<'a> {
    params: CBParams<'a>,
}

impl<'a> ListOrderParams<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new(),
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

impl<'a> ProductID<'a> for ListOrderParams<'a> {
    fn set_product_id(&mut self, value: &'a str) {
        self.params_mut().product_id = Some(value);
    }
}

impl<'a> Paginate<'a> for ListOrderParams<'a> {
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
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new(),
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
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new(),
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

impl<'a> Paginate<'a> for PaginateParams<'a> {
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
    pub(super) fn new(granularity: i32) -> Self {
        let mut params = CBParams::new();
        params.granularity = Some(granularity);
        Self { params: params }
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
    pub(super) fn new(product_id: &'a str, side: &'a str, price: f64, size: f64) -> Self {
        let mut params = CBParams::new();
        params.type_ = Some("limit");
        params.product_id = Some(product_id);
        params.side = Some(side);
        params.price = Some(price);
        params.size = Some(size);
        Self { params: params }
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

impl<'a> ClientOID<'a> for LimitOrderParams<'a> {
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
    pub(super) fn new(product_id: &'a str, side: &'a str, qty: QTY) -> Self {
        let mut params = CBParams::new();
        params.type_ = Some("market");
        params.product_id = Some(product_id);
        params.side = Some(side);
        match qty {
            QTY::Size(value) => params.size = Some(value),
            QTY::Funds(value) => params.funds = Some(value),
        };
        Self { params: params }
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

impl<'a> ClientOID<'a> for MarketOrderParams<'a> {
    fn set_client_oid(&mut self, value: &'a str) {
        self.params_mut().client_oid = Some(value);
    }
}

pub struct ReportParams<'a> {
    params: CBParams<'a>,
}

impl<'a> ReportParams<'a> {
    pub(super) fn new(start_date: String, end_date: String, rpt: RPT<'a>) -> Self {
        let mut params = CBParams::new();
        params.start_date = Some(start_date);
        params.end_date = Some(end_date);

        match rpt {
            RPT::Fills { product_id } => {
                params.product_id = Some(product_id);
                params.type_ = Some("fills");
            }
            RPT::Account { account_id } => {
                params.account_id = Some(account_id);
                params.type_ = Some("account");
            }
        }

        Self { params: params }
    }
}

impl<'a> Params<'a> for ReportParams<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Report<'a> for ReportParams<'a> {
    fn set_format(&mut self, value: &'a str) {
        self.params_mut().format = Some(value);
    }
    fn set_email(&mut self, value: &'a str) {
        self.params_mut().email = Some(value);
    }
}
//////////////////

type HmacSha256 = Hmac<Sha256>;

pub(super) fn sign_request<'a>(
    request: ClientRequest,
    body: Option<&CBParams<'a>>,
    auth: Option<Auth<'a>>,
) -> ClientRequest {
    let request = request.header("User-Agent", "Actix-web");
    
    if let Some(auth) = auth {
        let Auth { key, pass, secret } = auth;

        let timestamp = Utc::now().timestamp().to_string();
        let method = request.get_method().as_str();

        let path = if let Some(query) = request.get_uri().query() {
            String::new() + request.get_uri().path() + "?" + query
        } else {
            request.get_uri().path().to_string()
        };

        let message = if let Some(json) = body {
            timestamp.clone() + method + &path + &serde_json::to_string(json).unwrap()
        } else {
            timestamp.clone() + method + &path
        };

        let hmac_key = base64::decode(secret).unwrap();
        let mut mac = HmacSha256::new_varkey(&hmac_key).unwrap();
        mac.input(message.as_bytes());
        let signature = mac.result().code();
        let b64_signature = base64::encode(&signature);

        let request = request
            .header("CB-ACCESS-KEY", key)
            .header("CB-ACCESS-PASSPHRASE", pass)
            .header("CB-ACCESS-TIMESTAMP", &timestamp[..])
            .header("CB-ACCESS-SIGN", &b64_signature[..]);
        return request;
    }
    request
}

pub struct QueryBuilder<'a, T: Params<'a>> {
    request: ClientRequest,
    query: T,
    auth: Option<Auth<'a>>,
}

impl<'a, T: Params<'a>> QueryBuilder<'a, T> {
    pub(super) fn new(request: ClientRequest, query: T, auth: Option<Auth<'a>>) -> Self {
        Self {
            request: request,
            query,
            auth,
        }
    }

    pub async fn json(self) -> actix_web::Result<Value> {
        let Self {
            request,
            query,
            auth,
        } = self;

        let value = if request.get_method().as_str() == "POST" {
            sign_request(request, Some(query.params()), auth)
                .send_json(query.params())
                .await?
                .json::<Value>()
                .await?
        } else {
            let request = request.query(query.params())?;
            sign_request(request, None, auth)
                .send()
                .await?
                .json::<Value>()
                .await?
        };
        Ok(value)
    }
}

impl<'a, T: Params<'a> + ProductID<'a>> QueryBuilder<'a, T> {
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

impl<'a, T: Params<'a> + Paginate<'a> + Send + Unpin + 'a> QueryBuilder<'a, T> {
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
        Paginated::new(self.request.get_uri().to_string(), self.query, self.auth).pages()
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

impl<'a, T: Params<'a> + ClientOID<'a>> QueryBuilder<'a, T> {
    pub fn client_oid(mut self, value: &'a str) -> Self {
        self.query.set_client_oid(value);
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

impl<'a, T: Params<'a> + Report<'a>> QueryBuilder<'a, T> {
    pub fn format(mut self, value: &'a str) -> Self {
        self.query.set_format(value);
        self
    }

    pub fn email(mut self, value: &'a str) -> Self {
        self.query.set_email(value);
        self
    }
}
