use reqwest::{ Client, Url };
use chrono::{offset::TimeZone, DateTime};
use crate::builder::*;

/// https://api-public.sandbox.pro.coinbase.com
pub const SANDBOX_URL: &'static str = "https://api-public.sandbox.pro.coinbase.com";
/// https://api.pro.coinbase.com
pub const MAIN_URL: &'static str = "https://api.pro.coinbase.com";

/// ID variants for orders
pub enum ORD<'a> {
    /// order_id param
    OrderID(&'a str),
    /// client_oid param
    ClientOID(&'a str)
}
/// ID variants for fills
pub enum FILL<'a> {
    /// order_id param
    OrderID(&'a str),
    /// product_id param
    ProductID(&'a str)
}
/// Deposits variants
pub enum DEP<'a> {
    /// coinbase_account_id param
    CBAccountID(&'a str),
    /// payment_method_id param
    PYMTMethodID(&'a str)
}
/// Withdrawal variants
pub enum WDL<'a> {
    /// coinbase_account_id param
    CBAccountID(&'a str),
    /// payment_method_id param
    PYMTMethodID(&'a str),
    /// crypto withdrawal params
    Crypto { 
        /// A crypto address of the recipient
        addr: &'a str, 
        /// A destination tag for currencies that support one
        tag: Option<&'a str> 
    }
}
/// Report variants
pub enum RPT<'a> {
    /// product_id param to generate product_id
    Fills { product_id: &'a str },
    /// account_id param to generate product_id
    Account { account_id: &'a str }
}
/// Quantity variants for market orders                                                                                                                                                                                                                                                              
pub enum QTY {
    /// Quantity to buy or sell
    Size(f64),
    /// Quantity to use for buying or selling
    Funds(f64)
}

#[derive(Clone)]
pub(super) struct Auth {
    pub key: String,
    pub pass: String,
    pub secret: String,
}

/// Private client
pub struct AuthenticatedClient {
    auth: Auth,
    public: PublicClient,
}

impl AuthenticatedClient {
    /// Creates new instance
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = AuthenticatedClient::new("key".to_owned(), "pass".to_owned(), "secret".to_owned(), SANDBOX_URL);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(key: String, pass: String, secret: String, url: &str) -> Self {
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
    /// Get public client
    pub fn public(&self) -> &PublicClient {
        &self.public
    }
    /// Get a list of trading accounts from the profile of the API key.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let accounts = client
    ///     .list_accounts()
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&accounts).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_accounts<'a>(&self) -> QueryBuilder<NoOptions<'a>> {
        let url = self.url().join("/accounts").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// Information for a single account. 
    /// Use this endpoint when you know the account_id. 
    /// API key must belong to the same profile as the account.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let account = client
    ///     .get_account("<account_id>")
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&account).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_account<'a>(&self, account_id: &str) -> QueryBuilder<NoOptions<'a>> {
        let endpoint = format!("/accounts/{}", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// List account activity of the API key’s profile. 
    /// Account activity either increases or decreases your account balance. 
    /// Items are paginated and sorted latest first.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let history = client
    ///     .get_account_history("<account_id>")
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&history).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_account_history<'a>(&self, account_id: &str) -> QueryBuilder<PageOptions<'a>> {
        let endpoint = format!("/accounts/{}/ledger", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            PageOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// List holds of an account that belong to the same profile as the API key. 
    /// Holds are placed on an account for any active orders or pending withdraw requests. 
    /// As an order is filled, the hold amount is updated. If an order is canceled, any remaining hold is removed. 
    /// For a withdraw, once it is completed, the hold is removed.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let holds = client
    ///     .get_holds("<account_id>")
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&holds).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_holds<'a>(&self, account_id: &str) -> QueryBuilder<PageOptions<'a>> {
        let endpoint = format!("/accounts/{}/holds", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            PageOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// Orders can only be placed if your account has sufficient funds. 
    /// Once an order is placed, your account funds will be put on hold for the duration of the order. 
    /// How much and which funds are put on hold depends on the order type and parameters specified.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let response = client
    ///     .place_limit_order("BTC-USD", "buy", 7000.00, 10.00)
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn place_limit_order<'a>(&self, product_id: &'a str, side: &'a str, price: f64, size: f64) -> QueryBuilder<LimitOrderOptions<'a>> {
        let mut limit_options =  LimitOrderOptions::new();
        limit_options.params_mut().type_ = Some("limit");
        limit_options.params_mut().product_id = Some(product_id);
        limit_options.params_mut().side = Some(side);
        limit_options.params_mut().price = Some(price);
        limit_options.params_mut().size = Some(size);
        
        let url = self.url().join("/orders").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            limit_options,
            Some(self.auth.clone()),
        )
    }
    /// Orders can only be placed if your account has sufficient funds. 
    /// Once an order is placed, your account funds will be put on hold for the duration of the order. 
    /// How much and which funds are put on hold depends on the order type and parameters specified.
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, QTY};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let response = client
    ///     .place_market_order("BTC-USD", "buy", QTY::Size(10.00))
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn place_market_order<'a>(&self, product_id: &'a str, side: &'a str, qty: QTY) -> QueryBuilder<MarketOrderOptions<'a>> {
        let mut market_options =  MarketOrderOptions::new();
        market_options.params_mut().type_ = Some("market");
        market_options.params_mut().product_id = Some(product_id);
        market_options.params_mut().side = Some(side);

        match qty {
            QTY::Size(value) => market_options.params_mut().size = Some(value),
            QTY::Funds(value) => market_options.params_mut().funds = Some(value),
        };
        
        let url = self.url().join("/orders").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            market_options,
            Some(self.auth.clone()),
        )
    }
    /// Cancel a previously placed order. 
    /// Order must belong to the profile that the API key belongs to.
    ///
    /// If the order had no matches during its lifetime its record may be purged. 
    /// This means the order details will not be available with GET /orders/<id>.
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, ORD};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let response = client
    ///     .cancel_order(ORD::OrderID("<order_id>"))
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_order<'a>(&self, ord: ORD<'a>) -> QueryBuilder<NoOptions<'a>> {
        let endpoint = match ord {
            ORD::OrderID(id) => format!("/orders/{}", id),
            ORD::ClientOID(id) => format!("/orders/client:{}", id)
        };
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().delete(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// With best effort, cancel all open orders from the profile that the API key belongs to. 
    /// The response is a list of ids of the canceled orders.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let response = client
    ///     .cancel_all()
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_all<'a>(&self) -> QueryBuilder<CancelOptions<'a>> {
        let url = self.url().join("/orders").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().delete(url).build().unwrap(),
            CancelOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// List your current open orders from the profile that the API key belongs to. 
    /// Only open or un-settled orders are returned. 
    /// As soon as an order is no longer open and settled, it will no longer appear in the default request.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let orders = client
    ///     .list_orders(&["open"])
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&orders).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_orders<'a>(&self, status: &[&str]) -> QueryBuilder<ListOrderOptions<'a>> {
        let url = self.url().join("/orders").unwrap();
        let status: Vec<_> = status.iter().map(|x| ("status", x)).collect();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).query(&status).build().unwrap(),
            ListOrderOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// Get a single order by order id from the profile that the API key belongs to.
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, ORD};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let order = client
    ///     .get_order(ORD::OrderID("<order_id>"))
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&order).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_order<'a>(&self, ord: ORD<'a>) -> QueryBuilder<NoOptions<'a>> {
        let endpoint = match ord {
            ORD::OrderID(id) => format!("/orders/{}", id),
            ORD::ClientOID(id) => format!("/orders/client:{}", id)
        };
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// Get a list of recent fills of the API key’s profile.
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, FILL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let fills = client
    ///     .get_fills(FILL::ProductID("BTC-USD"))
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&fills).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_fills<'a>(&self, fill: FILL<'a>) -> QueryBuilder<NoOptions<'a>> {
        let url = self.url().join("/fills").unwrap();

        let mut no_options = NoOptions::new();
        match fill {
            FILL::OrderID(id) => no_options.params_mut().order_id = Some(id),
            FILL::ProductID(id) => no_options.params_mut().product_id = Some(id)
        }

        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            no_options,
            Some(self.auth.clone()),
        )
    }
    /// Deposit funds from a payment method.
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, DEP};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let response = client
    ///     .deposit(10.00, "BTC", DEP::CBAccountID("<account_id>"))
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn deposit<'a>(&self, amount: f64, currency: &'a str, dep: DEP<'a>) -> QueryBuilder<NoOptions<'a>> {
        let mut no_options =  NoOptions::new();
        no_options.params_mut().amount = Some(amount);
        no_options.params_mut().currency = Some(currency);

        let endpoint = match dep {
            DEP::CBAccountID(id) => {
                no_options.params_mut().coinbase_account_id = Some(id);
                "/deposits/coinbase-account"
            },
            DEP::PYMTMethodID(id) => {
                no_options.params_mut().payment_method_id = Some(id);
                "/deposits/payment-method"
            }
        };

        let url = self.url().join(endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            no_options,
            Some(self.auth.clone()),
        )
    }
    /// Deposit funds from a coinbase account. 
    /// You can move funds between your Coinbase accounts and your Coinbase Pro trading accounts within your daily limits. 
    /// Moving funds between Coinbase and Coinbase Pro is instant and free.
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, WDL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let response = client
    ///     .withdraw(10.00, "BTC", WDL::CBAccountID("<account_id>"))
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn withdraw<'a>(&self, amount: f64, currency: &'a str, wdl: WDL<'a>) -> QueryBuilder<NoOptions<'a>> {
        let mut no_options = NoOptions::new();
        no_options.params_mut().amount = Some(amount);
        no_options.params_mut().currency = Some(currency);

        let endpoint = match wdl {
            WDL::CBAccountID(id) => {
                no_options.params_mut().coinbase_account_id = Some(id);
                "/withdrawals/coinbase-account"
            },
            WDL::PYMTMethodID(id) => {
                no_options.params_mut().payment_method_id = Some(id);
                "/withdrawals/payment-method"
            },
            WDL::Crypto { addr, tag } => {
                no_options.params_mut().crypto_address = Some(addr);

                if let Some(t) = tag {
                    no_options.params_mut().destination_tag = Some(t);
                } else {
                    no_options.params_mut().no_destination_tag = Some(true);
                }

                "/withdrawals/crypto"
            }
        };

        let url = self.url().join(endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            no_options,
            Some(self.auth.clone()),
        )
    }
    /// Convert $10,000.00 to 10,000.00 USDC.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let response = client
    ///     .convert("USD", "USDC", 100.00)
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn convert<'a>(&self, from: &'a str, to: &'a str, amount: f64) -> QueryBuilder<NoOptions<'a>> {
        let mut no_options =  NoOptions::new();
        no_options.params_mut().from = Some(from);
        no_options.params_mut().to = Some(to);
        no_options.params_mut().amount = Some(amount);

        let url = self.url().join("/conversions").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            no_options,
            Some(self.auth.clone()),
        )
    }
    /// Get a list of your payment methods.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let payment_methods = client
    ///     .list_payment_methods()
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&payment_methods).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_payment_methods<'a>(&self) -> QueryBuilder<NoOptions<'a>> {
        let url = self.url().join("/payment-methods").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// Get a list of your coinbase accounts.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let coinbase_accounts = client
    ///     .list_coinbase_accounts()
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&coinbase_accounts).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_coinbase_accounts<'a>(&self) -> QueryBuilder<NoOptions<'a>> {
        let url = self.url().join("/coinbase-accounts").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// This request will return your current maker & taker fee rates, as well as your 30-day trailing volume. 
    /// Quoted rates are subject to change.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let current_fees = client
    ///     .get_current_fees()
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&current_fees).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_current_fees<'a>(&self) -> QueryBuilder<NoOptions<'a>> {
        let url = self.url().join("/fees").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// Reports provide batches of historic information about your profile in various human and machine readable forms.
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, RPT};
    /// use chrono::{ TimeZone, Utc };
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let start_date = Utc.ymd(2018, 8, 10).and_hms(0, 0, 0);
    /// let end_date = Utc.ymd(2018, 8, 28).and_hms(0, 0, 0);
    ///
    /// let rates = client
    ///     .create_report(start_date, end_date, RPT::Fills { product_id: "BTC-USD" })
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&rates).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_report<'a, Tz: TimeZone>(&self, start_date: DateTime<Tz>, end_date: DateTime<Tz>, rpt: RPT<'a>) -> QueryBuilder<ReportOptions<'a>> 
        where
            Tz::Offset: core::fmt::Display,
    {
        let mut report_options =  ReportOptions::new();
        report_options.params_mut().start_date = Some(start_date.to_rfc3339());
        report_options.params_mut().end_date = Some(end_date.to_rfc3339());

        match rpt {
            RPT::Fills { product_id } => {
                report_options.params_mut().product_id = Some(product_id);
                report_options.params_mut().type_ = Some("fills");
            },
            RPT::Account { account_id } => {
                report_options.params_mut().account_id = Some(account_id);
                report_options.params_mut().type_ = Some("account");
            },
        }

        let url = self.url().join("/reports").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            report_options,
            Some(self.auth.clone()),
        )
    }
    /// Once a report request has been accepted for processing, the status is available by polling the report resource endpoint.
    ///
    /// The final report will be uploaded and available at file_url once the status indicates ready
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let report_status = client
    ///     .get_report_status("<report_id>")
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&report_status).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_report_status<'a>(&self, report_id: &'a str) -> QueryBuilder<NoOptions<'a>> {
        let endpoint = format!("/reports/:{}", report_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// List your profiles.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let profiles = client
    ///     .list_profiles()
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&profiles).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_profiles<'a>(&self) -> QueryBuilder<NoOptions<'a>> {
        let url = self.url().join("/profiles").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// Get a single profile by profile id.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let profile = client
    ///     .get_profile("<profile_id>")
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&profile).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_profile<'a>(&self, profile_id: &'a str) -> QueryBuilder<NoOptions<'a>> {
        let endpoint = format!("/profiles/{}", profile_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
    /// Transfer funds from API key’s profile to another user owned profile.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let response = client
    ///     .transfer_profile("<from_profile_id>", "<to_profile_id>", "BTC-USD", 10.00)
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn transfer_profile<'a>(&self, from: &'a str, to: &'a str, currency: &'a str, amount: f64) -> QueryBuilder<NoOptions<'a>> {
        let mut no_options =  NoOptions::new();
        no_options.params_mut().from = Some(from);
        no_options.params_mut().to = Some(to);
        no_options.params_mut().currency = Some(currency);
        no_options.params_mut().amount = Some(amount);
        
        let url = self.url().join("/profiles/transfer").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            no_options,
            Some(self.auth.clone()),
        )
    }
    /// This endpoint requires either the “view” or “trade” permission.
    ///
    /// This request will return your 30-day trailing volume for all products of the API key’s profile. 
    /// This is a cached value that’s calculated every day at midnight UTC.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = AuthenticatedClient::new(String::new(), String::new(), String::new(), SANDBOX_URL);
    /// let trailing_volume = client
    ///     .get_trailing_volume()
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&trailing_volume).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_trailing_volume<'a>(&self) -> QueryBuilder<NoOptions<'a>> {
        let url = self.url().join("/users/self/trailing-volume").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth.clone()),
        )
    }
}

/// Public client
pub struct PublicClient {
    client: Client,
    url: Url,
}

impl PublicClient {
    /// Creates new instance
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{PublicClient, SANDBOX_URL};
    /// 
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = PublicClient::new(SANDBOX_URL);
    ///     Ok(())
    /// }
    /// ```
    pub fn new(url: &str) -> Self {
        Self {
            client: Client::new(),
            url: Url::parse(url).expect("Invalid Url"),
        }
    }
    /// Get a list of available currency pairs for trading.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let products = client
    ///     .get_products()
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&products).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_products<'a>(&self) -> QueryBuilder<NoOptions<'a>> {
        let url = self.url.join("/products").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoOptions::new(),
            None,
        )
    }
    /// Get a list of open orders for a product. 
    /// The amount of detail shown can be customized with the level parameter.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let order_book = client
    ///     .get_product_order_book("BTC-USD")
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&order_book).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_product_order_book<'a>(&self, product_id: &str) -> QueryBuilder<BookOptions<'a>> {
        let endpoint = format!("/products/{}/book", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            BookOptions::new(),
            None,
        )
    }
    /// Snapshot information about the last trade (tick), best bid/ask and 24h volume.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let ticker = client
    ///     .get_product_ticker("BTC-USD")
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&ticker).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_product_ticker<'a>(&self, product_id: &str) -> QueryBuilder<NoOptions<'a>> {
        let endpoint = format!("/products/{}/ticker", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoOptions::new(),
            None,
        )
    }
    /// List the latest trades for a product.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// use futures::TryStreamExt;
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let mut trades = client
    ///     .get_trades("BTC-USD")
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&trades).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_trades<'a>(&self, product_id: &str) -> QueryBuilder<PageOptions<'a>> {
        let endpoint = format!("/products/{}/trades", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            PageOptions::new(),
            None,
        )
    }
    /// Historic rates for a product. 
    /// Rates are returned in grouped buckets based on requested granularity.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let rates = client
    ///     .get_historic_rates("BTC-USD", 3600)
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&rates).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_historic_rates<'a>(&self, product_id: &str, granularity: i32) -> QueryBuilder<CandleOptions<'a>>{
        let mut candle_options =  CandleOptions::new();
        candle_options.params_mut().granularity = Some(granularity);
        
        let endpoint = format!("/products/{}/candles", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            candle_options,
            None,
        )
    }
    /// Get 24 hr stats for the product. 
    /// volume is in base currency units. open, high, low are in quote currency units.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let stats = client
    ///     .get_24hr_stats("BTC-USD")
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&stats).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_24hr_stats<'a>(&self, product_id: &str) -> QueryBuilder<NoOptions<'a>> {
        let endpoint = format!("/products/{}/stats", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoOptions::new(),
            None,
        )
    }
    /// List known currencies.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let currencies = client
    ///     .get_currencies()
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&currencies).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_currencies<'a>(&self) -> QueryBuilder<NoOptions<'a>> {
        let url = self.url.join("/currencies").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoOptions::new(),
            None,
        )
    }
    /// Get the API server time.
    /// # Example
    ///
    /// ```no_run
    /// # use cbpro::client::{PublicClient, SANDBOX_URL};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// # let client = PublicClient::new(SANDBOX_URL);
    /// let time = client
    ///     .get_time()
    ///     .json()
    ///     .await?;
    /// 
    /// println!("{}", serde_json::to_string_pretty(&time).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_time<'a>(&self) -> QueryBuilder<NoOptions<'a>> {
        let url = self.url.join("/time").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoOptions::new(),
            None,
        )
    }
}
