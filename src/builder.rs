use crate::{paging::{Pages, Paginated}, client::Auth};
use chrono::{offset::{TimeZone, Utc}, DateTime};
use hmac::{Hmac, Mac};
use reqwest::{
    header::{HeaderValue, CONTENT_TYPE},
    Client, Request
};
use serde::Serialize;
use serde::de::DeserializeOwned;
use sha2::Sha256;
use crate::error::CBError;

#[derive(Serialize)]
pub struct CBParams<'a> {
    level: Option<i32>,
    start: Option<String>,
    end: Option<String>,
    pub(super) granularity: Option<i32>,
    client_oid: Option<&'a str>,
    pub(super) order_id: Option<&'a str>,
    #[serde(rename(serialize = "type"))]
    pub(super) type_: Option<&'a str>,
    //limit
    pub(super) side: Option<&'a str>,
    pub(super) product_id: Option<&'a str>,
    stp: Option<&'a str>,
    stop: Option<&'a str>,
    stop_price: Option<f64>,
    pub(super) price: Option<f64>,
    pub(super) size: Option<f64>,
    time_in_force: Option<&'a str>,
    cancel_after: Option<&'a str>,
    post_only: Option<bool>,
    //market
    pub(super) funds: Option<f64>,
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
    pub(super) start_date: Option<String>,
    pub(super) end_date: Option<String>,
    format: Option<&'a str>,
    email: Option<&'a str>,
    pub(super) account_id: Option<&'a str>,
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

pub struct NoOptions<'a> {
    params: CBParams<'a>,
}

impl<'a> NoOptions<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for NoOptions<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

pub struct CancelOptions<'a> {
    params: CBParams<'a>,
}

impl<'a> CancelOptions<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for CancelOptions<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> ProductID<'a> for CancelOptions<'a> {
    fn set_product_id(&mut self, value: &'a str) {
        self.params_mut().product_id = Some(value);
    }
}

pub struct ListOrderOptions<'a> {
    params: CBParams<'a>,
}

impl<'a> ListOrderOptions<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for ListOrderOptions<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> ProductID<'a> for ListOrderOptions<'a> {
    fn set_product_id(&mut self, value: &'a str) {
        self.params_mut().product_id = Some(value);
    }
}

impl<'a> Paginate<'a> for ListOrderOptions<'a> {
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

pub struct BookOptions<'a> {
    params: CBParams<'a>,
}

impl<'a> BookOptions<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for BookOptions<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Book<'a> for BookOptions<'a> {
    fn set_level(&mut self, value: i32) {
        self.params_mut().level = Some(value);
    }
}

pub struct PageOptions<'a> {
    params: CBParams<'a>,
}

impl<'a> PageOptions<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for PageOptions<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Paginate<'a> for PageOptions<'a> {
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

pub struct CandleOptions<'a> {
    params: CBParams<'a>,
}

impl<'a> CandleOptions<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for CandleOptions<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Candle<'a> for CandleOptions<'a> {
    fn set_start(&mut self, value: String) {
        self.params_mut().start = Some(value);
    }
    fn set_end(&mut self, value: String) {
        self.params_mut().end = Some(value);
    }
}


pub struct LimitOrderOptions<'a> {
    params: CBParams<'a>,
}

impl<'a> LimitOrderOptions<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for LimitOrderOptions<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> ClientOID<'a> for LimitOrderOptions<'a> {
    fn set_client_oid(&mut self, value: &'a str) {
        self.params_mut().client_oid = Some(value);
    }
}

impl<'a> Limit<'a> for LimitOrderOptions<'a> {
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

pub struct MarketOrderOptions<'a> {
    params: CBParams<'a>,
}

impl<'a> MarketOrderOptions<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for MarketOrderOptions<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> ClientOID<'a> for MarketOrderOptions<'a> {
    fn set_client_oid(&mut self, value: &'a str) {
        self.params_mut().client_oid = Some(value);
    }
}

pub struct ReportOptions<'a> {
    params: CBParams<'a>,
}

impl<'a> ReportOptions<'a> {
    pub(super) fn new() -> Self {
        Self {
            params: CBParams::new()
        }
    }
}

impl<'a> Params<'a> for ReportOptions<'a> {
    fn params_mut(&mut self) -> &mut CBParams<'a> {
        &mut self.params
    }

    fn params(&self) -> &CBParams<'a> {
        &self.params
    }
}

impl<'a> Report<'a> for ReportOptions<'a> {
    fn set_format(&mut self, value: &'a str) {
        self.params_mut().format = Some(value);
    }
    fn set_email(&mut self, value: &'a str) {
        self.params_mut().email = Some(value);
    }
}
//////////////////

type HmacSha256 = Hmac<Sha256>;

pub(super) fn apply_query<T: Serialize>(req: &mut Request, query: &T) -> crate::error::Result<()> {
    {
        let url = req.url_mut();
        let mut pairs = url.query_pairs_mut();
        let serializer = serde_urlencoded::Serializer::new(&mut pairs);
        query.serialize(serializer)?;
    }
    if let Some("") = req.url().query() {
        req.url_mut().set_query(None);
    }
    Ok(())
}

pub(super) fn apply_json<T: Serialize>(req: &mut Request, json: &T) -> crate::error::Result<()> {
    let body = serde_json::to_vec(json)?;
    req.headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    *req.body_mut() = Some(body.into());
    Ok(())
}

/// Builder returned by the public and private client. 
/// All methods are optional but the builder must be consumed with one of the terminal methods.
/// Methods belonging to this struct can be chained and calling the same method more than once will overwrite the previously set value.
pub struct QueryBuilder<T> {
    client: Client,
    request: Request,
    query: T,
    auth: Option<Auth>,
}

impl<'a, T: Params<'a>> QueryBuilder<T> {
    pub(super) fn new(
        client: Client,
        request: Request,
        query: T,
        auth: Option<Auth>,
    ) -> Self {
        Self {
            client,
            request,
            query,
            auth,
        }
    }

    fn auth_request(&self) -> crate::error::Result<Request> {
        let mut request = self.request.try_clone().unwrap();

        if let &reqwest::Method::POST = request.method() {
            apply_json(&mut request, self.query.params())?;
        } else {
            apply_query(&mut request, self.query.params())?;
        }

        if let Some(ref auth) = self.auth {
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
                    + std::str::from_utf8(body.as_bytes().unwrap())?
            } else {
                timestamp.clone() + method + &path[..]
            };

            let hmac_key = base64::decode(secret)?;
            let mut mac = HmacSha256::new_varkey(&hmac_key)?;
            mac.input(message.as_bytes());
            let signature = mac.result().code();
            let b64_signature = base64::encode(&signature);

            request.headers_mut().insert("CB-ACCESS-KEY", key.parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-PASSPHRASE", pass.parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-TIMESTAMP", (&timestamp[..]).parse().unwrap());
            request.headers_mut().insert("CB-ACCESS-SIGN", (&b64_signature[..]).parse().unwrap());
        }

        request.headers_mut().insert("User-Agent", "cbpro".parse().unwrap());
        Ok(request)
    }
    
    pub async fn text(self) -> crate::error::Result<String> {
        let resp = self.client.execute(self.auth_request()?).await?;
        if resp.status().is_success() {
            Ok(resp.text().await?)
        } else {
            let error = CBError::new(resp.status().as_u16(), resp.text().await?);
            Err(error.into())
        }   
    }

    /// General terminal method
    /// # Example
    /// 
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let products = client
    ///     .get_products()
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&products).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn json<J: DeserializeOwned>(self) -> crate::error::Result<J> {
        let json = serde_json::from_str(&self.text().await?)?;
        Ok(json)
    }
}

impl<'a, T: Params<'a> + ProductID<'a>> QueryBuilder<T> {
    /// Sets product id
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new("<key>", "<pass>", "<secret>", SANDBOX_URL);
    /// let response = client.cancel_all()
    ///     .product_id("BTC-USD")
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn product_id(mut self, value: &'a str) -> Self {
        self.query.set_product_id(value);
        self
    }
}

impl<'a, T: Params<'a> + Book<'a>> QueryBuilder<T> {
    /// Sets level for order book data. Max level is 3.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let order_book = client.get_product_order_book("BTC-USD")
    ///     .level(3)
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&order_book).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn level(mut self, value: i32) -> Self {
        self.query.set_level(value);
        self
    }
}

impl<'a, T: Params<'a> + Paginate<'a> + Send + Unpin + 'a> QueryBuilder<T> {
    /// Sets limit for the ammount of pages each request will return. 
    /// Max number of pages is 100.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # use futures::TryStreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let mut pages = client.get_trades("BTC-USD")
    ///     .limit(10)
    ///     .after(7102310) // after or before but not both
    ///     .paginate::<serde_json::Value>()?; // or .json::<serde_json::Value>().await? for a single request
    ///
    /// while let Some(json) = pages.try_next().await? {
    ///     println!("{}", serde_json::to_string_pretty(&json).unwrap());
    ///     tokio::time::delay_for(core::time::Duration::new(1, 0)).await;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn limit(mut self, value: i32) -> Self {
        self.query.set_limit(value);
        self
    }

    /// Gets newer page from the trade id specified.
    pub fn before(mut self, value: i32) -> Self {
        self.query.set_before(value);
        self
    }

    /// Gets older page from the trade id specified.
    pub fn after(mut self, value: i32) -> Self {
        self.query.set_after(value);
        self
    }
    /// Terminal method returning a stream of json pages
    pub fn paginate<J: DeserializeOwned>(self) -> crate::error::Result<Pages<'a, J>> {
        let pages = Paginated::new(self.client.clone(), self.auth_request()?, self.query).pages();
        Ok(pages)
    }
}
impl<'a, T: Params<'a> + Candle<'a>> QueryBuilder<T> {
    /// Sets start and end time for historic rates.
    /// If the range results in more than 300 candles, the request will be rejected.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let end = chrono::offset::Utc::now();
    /// let start = end - chrono::Duration::hours(5);
    ///
    /// let rates = client
    ///     .get_historic_rates("BTC-USD", 3600)
    ///     .range(start, end)
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&rates).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn range<Tz: TimeZone>(mut self, start: DateTime<Tz>, end: DateTime<Tz>) -> Self 
    where
        Tz::Offset: core::fmt::Display,
    {
        self.query.set_start(start.to_rfc3339());
        self.query.set_end(end.to_rfc3339());
        self
    }
}

impl<'a, T: Params<'a> + ClientOID<'a>> QueryBuilder<T> {
/// Sets uuid as part of this order
/// # Example
///
/// ```no_run
/// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL, QTY};
/// # #[tokio::main]
/// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
/// # let client = AuthenticatedClient::new("<key>", "<pass>", "<secret>", SANDBOX_URL);
/// let response = client.place_market_order("BTC-USD", "buy", QTY::Size(10.00))
///     .client_oid("<client_oid>")
///     .json::<serde_json::Value>()
///     .await?;
/// 
/// println!("{}", serde_json::to_string_pretty(&response).unwrap());
/// # Ok(())
/// # }
/// ```
    pub fn client_oid(mut self, value: &'a str) -> Self {
        self.query.set_client_oid(value);
        self
    }
}

impl<'a, T: Params<'a> + Limit<'a>> QueryBuilder<T> {
    /// Sets Self-trade prevention flag.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new("<key>", "<pass>", "<secret>", SANDBOX_URL);
    /// let response = client
    ///     .place_limit_order("BTC-USD", "sell", 7000.00, 10.00)
    ///     .stp("dc")
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn stp(mut self, value: &'a str) -> Self {
        self.query.set_stp(value);
        self
    }
    /// Turns sell limit order into a stop loss or a stop entry for a buy limit order.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new("<key>", "<pass>", "<secret>", SANDBOX_URL);
    /// let response = client
    ///     .place_limit_order("BTC-USD", "sell", 7000.00, 10.00)
    ///     .stop_price(7010.00)
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
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
    /// Sets time in force policy. (default is GTC)
    /// Valid inputs are: "GTC", "GTT", "IOC", "FOK". 
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new("<key>", "<pass>", "<secret>", SANDBOX_URL);
    /// let response = client
    ///     .place_limit_order("BTC-USD", "sell", 7000.00, 10.00)
    ///     .time_in_force("GTT")
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn time_in_force(mut self, value: &'a str) -> Self {
        self.query.set_time_in_force(value);
        self
    }
    /// Sets time before order is cancelled
    /// Valid inputs are: "min", "hour", "day".
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new("<key>", "<pass>", "<secret>", SANDBOX_URL);
    /// let response = client
    ///     .place_limit_order("BTC-USD", "sell", 7000.00, 10.00)
    ///     .cancel_after("min")
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_after(mut self, value: &'a str) -> Self {
        self.query.set_cancel_after(value);
        self.query.set_time_in_force("GTT");
        self
    }
    /// The post-only flag indicates that the order should only make liquidity. 
    /// If any part of the order results in taking liquidity, the order will be rejected and no part of it will execute.
    /// Invalid when time_in_force is IOC or FOK.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new("<key>", "<pass>", "<secret>", SANDBOX_URL);
    /// let response = client
    ///     .place_limit_order("BTC-USD", "sell", 7000.00, 10.00)
    ///     .post_only(true)
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn post_only(mut self, value: bool) -> Self {
        self.query.set_post_only(value);
        self
    }
}
impl<'a, T: Params<'a> + Report<'a>> QueryBuilder<T> {
    /// Sets format of output report.
    /// Valid inputs are "pdf" or "csv" (defualt is pdf)
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL, RPT};
    /// use chrono::{ TimeZone, Utc };
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new("<key>", "<pass>", "<secret>", SANDBOX_URL);
    /// let start_date = Utc.ymd(2018, 8, 10).and_hms(0, 0, 0);
    /// let end_date = Utc.ymd(2018, 8, 28).and_hms(0, 0, 0);
    ///
    /// let rates = client.create_report(start_date, end_date, RPT::Fills { product_id: "BTC-USD" })
    ///     .format("pdf")
    ///     .email("<email>")
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&rates).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn format(mut self, value: &'a str) -> Self {
        self.query.set_format(value);
        self
    }
    /// Sets to send report to.
    /// Valid inputs are "pdf" or "csv" (defualt is pdf)
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL, RPT};
    /// use chrono::{ TimeZone, Utc };
    /// 
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new("<key>", "<pass>", "<secret>", SANDBOX_URL);
    /// let start_date = Utc.ymd(2018, 8, 10).and_hms(0, 0, 0);
    /// let end_date = Utc.ymd(2018, 8, 28).and_hms(0, 0, 0);
    ///
    /// let rates = client
    ///     .create_report(start_date, end_date, RPT::Fills { product_id: "BTC-USD" })
    ///     .email("<email>")
    ///     .json::<serde_json::Value>()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&rates).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn email(mut self, value: &'a str) -> Self {
        self.query.set_email(value);
        self
    }
}