use reqwest::{ Client, Url };
use crate::builder::*;

pub const SANDBOX_URL: &str = "https://api-public.sandbox.pro.coinbase.com";

pub enum ORD<'a> {
    OrderID(&'a str),
    ClientOID(&'a str),
}

pub enum FILL<'a> {
    OrderID(&'a str),
    ProductID(&'a str),
}

pub enum DEP<'a> {
    CBAccountID(&'a str),
    PYMTMethodID(&'a str),
}

pub enum WDL<'a> {
    CBAccountID(&'a str),
    PYMTMethodID(&'a str),
    Crypto { addr: &'a str, tag: Option<&'a str> },
}
                                                                                                                                                                                                                                                                                    
pub enum QTY {
    Size(f64),
    Funds(f64),
}

#[derive(Copy, Clone)]
pub(super) struct Auth<'a> {
    pub key: &'a str,
    pub pass: &'a str,
    pub secret: &'a str,
}

pub struct AuthenticatedClient<'a> {
    auth: Auth<'a>,
    public: PublicClient,
}

impl<'a> AuthenticatedClient<'a> {
    pub fn new(key: &'a str, pass: &'a str, secret: &'a str, url: &str) -> Self {
        Self {
            auth: Auth { key, pass, secret },
            public: PublicClient::new(url),
        }
    }

    fn client(&self) -> &Client {
        &self.public.client
    }

    fn url(&self) -> &Url {
        &self.public.url
    }

    pub fn public(&self) -> &PublicClient {
        &self.public
    }

    pub fn list_accounts(&self) -> QueryBuilder<'a, NoParams<'a>> {
        let url = self.url().join("/accounts").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoParams::new(),
            Some(self.auth),
        )
    }
    
    pub fn get_account(&self, account_id: &str) -> QueryBuilder<'a, NoParams<'a>> {
        let endpoint = format!("/accounts/{}", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoParams::new(),
            Some(self.auth),
        )
    }

    pub fn get_account_history(&self, account_id: &str) -> QueryBuilder<'a, PaginateParams<'a>> {
        let endpoint = format!("/accounts/{}/ledger", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            PaginateParams::new(),
            Some(self.auth),
        )
    }

    pub fn get_holds(&self, account_id: &str) -> QueryBuilder<'a, PaginateParams<'a>> {
        let endpoint = format!("/accounts/{}/holds", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            PaginateParams::new(),
            Some(self.auth),
        )
    }

    pub fn place_limit_order(&self, product_id: &'a str, side: &'a str, price: f64, size: f64) -> QueryBuilder<'a, LimitOrderParams<'a>> {
        let url = self.url().join("/orders").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            LimitOrderParams::new(product_id, side, price, size),
            Some(self.auth),
        )
    }

    pub fn place_market_order(&self, product_id: &'a str, side: &'a str, qty: QTY) -> QueryBuilder<'a, MarketOrderParams<'a>> {
        let url = self.url().join("/orders").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            MarketOrderParams::new(product_id, side, qty),
            Some(self.auth),
        )
    }

    pub fn cancel_order(&self, ord: ORD<'a>) -> QueryBuilder<'a, NoParams<'a>> {
        let endpoint = match ord {
            ORD::OrderID(id) => format!("/orders/{}", id),
            ORD::ClientOID(id) => format!("/orders/client:{}", id)
        };
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().delete(url).build().unwrap(),
            NoParams::new(),
            Some(self.auth),
        )
    }

    pub fn cancel_all(&self) -> QueryBuilder<'a, ProductParams<'a>> {
        let url = self.url().join("/orders").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().delete(url).build().unwrap(),
            ProductParams::new(),
            Some(self.auth),
        )
    }

    pub fn list_orders(&self, status: &[&str]) -> QueryBuilder<'a, ListOrderParams<'a>> {
        let url = self.url().join("/orders").unwrap();
        let status: Vec<_> = status.iter().map(|x| ("status", x)).collect();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).query(&status).build().unwrap(),
            ListOrderParams::new(),
            Some(self.auth),
        )
    }

    pub fn get_order(&self, ord: ORD<'a>) -> QueryBuilder<'a, NoParams<'a>> {
        let endpoint = match ord {
            ORD::OrderID(id) => format!("/orders/{}", id),
            ORD::ClientOID(id) => format!("/orders/client:{}", id)
        };
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoParams::new(),
            Some(self.auth),
        )
    }

    pub fn get_fills(&self, fill: FILL<'a>) -> QueryBuilder<'a, FillsParams<'a>> {
        let url = self.url().join("/fills").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            FillsParams::new(fill),
            Some(self.auth),
        )
    }

    pub fn deposit(&self, amount: f64, currency: &'a str, dep: DEP<'a>) -> QueryBuilder<'a, DepositsParams<'a>> {
        let endpoint = match dep {
            DEP::CBAccountID(_) => "/deposits/coinbase-account",
            DEP::PYMTMethodID(_) => "/deposits/payment-method"
        };
        let url = self.url().join(endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            DepositsParams::new(amount, currency, dep),
            Some(self.auth),
        )
    }

    pub fn withdraw(&self, amount: f64, currency: &'a str, wdl: WDL<'a>) -> QueryBuilder<'a, WithdrawalsParams<'a>> {
        let endpoint = match wdl {
            WDL::CBAccountID(_) => "/withdrawals/coinbase-account",
            WDL::PYMTMethodID(_) => "/withdrawals/payment-method",
            WDL::Crypto { addr: _, tag: _ } => "/withdrawals/crypto",
        };
        let url = self.url().join(endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            WithdrawalsParams::new(amount, currency, wdl),
            Some(self.auth),
        )
    }

    pub fn convert(&self, from: &'a str, to: &'a str, amount: f64) -> QueryBuilder<'a, ConversionParams<'a>> {
        let url = self.url().join("/conversions").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            ConversionParams::new(from, to, amount),
            Some(self.auth),
        )
    }

    pub fn list_payment_methods(&self) -> QueryBuilder<'a, NoParams<'a>> {
        let url = self.url().join("/payment-methods").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoParams::new(),
            Some(self.auth),
        )
    }

    pub fn list_coinbase_accounts(&self) -> QueryBuilder<'a, NoParams<'a>> {
        let url = self.url().join("/coinbase-accounts").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoParams::new(),
            Some(self.auth),
        )
    }

    pub fn get_current_fees(&self) -> QueryBuilder<'a, NoParams<'a>> {
        let url = self.url().join("/fees").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoParams::new(),
            Some(self.auth),
        )
    }
}

pub struct PublicClient {
    client: Client,
    url: Url,
}

impl PublicClient {
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            url: Url::parse(url).expect("Invalid Url"),
        }
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let products = client.get_products().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&products).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_products<'a>(&self) -> QueryBuilder<'a, NoParams<'a>> {
        let url = self.url.join("/products").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoParams::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let order_book = client.get_product_order_book("BTC-USD").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&order_book).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_product_order_book<'a>(&self, product_id: &str) -> QueryBuilder<'a, BookParams<'a>> {
        let endpoint = format!("/products/{}/book", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            BookParams::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let ticker = client.get_product_ticker("BTC-USD").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&ticker).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_product_ticker<'a>(&self, product_id: &str) -> QueryBuilder<'a, NoParams<'a>> {
        let endpoint = format!("/products/{}/ticker", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoParams::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::{PublicClient, SANDBOX_URL};
    /// use futures::stream::TryStreamExt;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let mut stream = client.get_trades("BTC-USD").paginate();
    ///
    /// while let Some(json) = stream.try_next().await? {
    ///     println!("{}", serde_json::to_string_pretty(&json).unwrap());
    ///     tokio_timer::delay_for(core::time::Duration::new(1, 0)).await;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_trades<'a>(&self, product_id: &str) -> QueryBuilder<'a, PaginateParams<'a>> {
        let endpoint = format!("/products/{}/trades", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            PaginateParams::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let end = chrono::offset::Utc::now();
    /// let start = end - chrono::Duration::hours(5);
    ///
    /// let rates = client.get_historic_rates("BTC-USD", 3600).range(start, end).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&rates).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_historic_rates<'a>(&self, product_id: &str, granularity: i32) -> QueryBuilder<'a, CandleParams<'a>>{
        let endpoint = format!("/products/{}/candles", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            CandleParams::new(granularity),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let stats = client.get_24hr_stats("BTC-USD").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&stats).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_24hr_stats<'a>(&self, product_id: &str) -> QueryBuilder<'a, NoParams<'a>> {
        let endpoint = format!("/products/{}/stats", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoParams::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let currencies = client.get_currencies().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&currencies).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_currencies<'a>(&self) -> QueryBuilder<'a, NoParams<'a>> {
        let url = self.url.join("/currencies").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoParams::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let time = client.get_time().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&time).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_time<'a>(&self) -> QueryBuilder<'a, NoParams<'a>> {
        let url = self.url.join("/time").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoParams::new(),
            None,
        )
    }
}
