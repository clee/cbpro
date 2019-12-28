use reqwest::{ Client, Url };
use chrono::{offset::TimeZone, DateTime};
use crate::builder::*;

pub const SANDBOX_URL: &str = "https://api-public.sandbox.pro.coinbase.com";

pub enum ORD<'a> {
    OrderID(&'a str),
    ClientOID(&'a str)
}

pub enum FILL<'a> {
    OrderID(&'a str),
    ProductID(&'a str)
}

pub enum DEP<'a> {
    CBAccountID(&'a str),
    PYMTMethodID(&'a str)
}

pub enum WDL<'a> {
    CBAccountID(&'a str),
    PYMTMethodID(&'a str),
    Crypto { addr: &'a str, tag: Option<&'a str> }
}

pub enum RPT<'a> {
    Fills { product_id: &'a str },
    Account { account_id: &'a str }
}
                                                                                                                                                                                                                                                                                    
pub enum QTY {
    Size(f64),
    Funds(f64)
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
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let accounts = client.list_accounts().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&accounts).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_accounts(&self) -> QueryBuilder<'a, NoOptions<'a>> {
        let url = self.url().join("/accounts").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let account = client.get_account("<account_id>").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&account).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_account(&self, account_id: &str) -> QueryBuilder<'a, NoOptions<'a>> {
        let endpoint = format!("/accounts/{}", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let history = client.get_account_history("<account_id>").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&history).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_account_history(&self, account_id: &str) -> QueryBuilder<'a, PaginateOptions<'a>> {
        let endpoint = format!("/accounts/{}/ledger", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            PaginateOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let holds = client.get_holds("<account_id>").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&holds).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_holds(&self, account_id: &str) -> QueryBuilder<'a, PaginateOptions<'a>> {
        let endpoint = format!("/accounts/{}/holds", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            PaginateOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let response = client.place_limit_order("BTC-USD", "buy", 7000.00, 10.00).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn place_limit_order(&self, product_id: &'a str, side: &'a str, price: f64, size: f64) -> QueryBuilder<'a, LimitOrderOptions<'a>> {
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
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, QTY};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let response = client.place_market_order("BTC-USD", "buy", QTY::Size(10.00)).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn place_market_order(&self, product_id: &'a str, side: &'a str, qty: QTY) -> QueryBuilder<'a, MarketOrderOptions<'a>> {
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
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, ORD};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let response = client.cancel_order(ORD::OrderID("<order_id>")).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_order(&self, ord: ORD<'a>) -> QueryBuilder<'a, NoOptions<'a>> {
        let endpoint = match ord {
            ORD::OrderID(id) => format!("/orders/{}", id),
            ORD::ClientOID(id) => format!("/orders/client:{}", id)
        };
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().delete(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let response = client.cancel_all().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn cancel_all(&self) -> QueryBuilder<'a, CancelOptions<'a>> {
        let url = self.url().join("/orders").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().delete(url).build().unwrap(),
            CancelOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let orders = client.list_orders(&["open"]).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&orders).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_orders(&self, status: &[&str]) -> QueryBuilder<'a, ListOrderOptions<'a>> {
        let url = self.url().join("/orders").unwrap();
        let status: Vec<_> = status.iter().map(|x| ("status", x)).collect();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).query(&status).build().unwrap(),
            ListOrderOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, ORD};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let order = client.get_order(ORD::OrderID("<order_id>")).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&order).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_order(&self, ord: ORD<'a>) -> QueryBuilder<'a, NoOptions<'a>> {
        let endpoint = match ord {
            ORD::OrderID(id) => format!("/orders/{}", id),
            ORD::ClientOID(id) => format!("/orders/client:{}", id)
        };
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, FILL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let fills = client.get_fills(FILL::ProductID("BTC-USD")).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&fills).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_fills(&self, fill: FILL<'a>) -> QueryBuilder<'a, NoOptions<'a>> {
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
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, DEP};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let response = client.deposit(10.00, "BTC", DEP::CBAccountID("<account_id>")).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn deposit(&self, amount: f64, currency: &'a str, dep: DEP<'a>) -> QueryBuilder<'a, NoOptions<'a>> {
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
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, WDL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let response = client.withdraw(10.00, "BTC", WDL::CBAccountID("<account_id>")).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn withdraw(&self, amount: f64, currency: &'a str, wdl: WDL<'a>) -> QueryBuilder<'a, NoOptions<'a>> {
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
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let response = client.convert("USD", "USDC", 100.00).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn convert(&self, from: &'a str, to: &'a str, amount: f64) -> QueryBuilder<'a, NoOptions<'a>> {
        let mut no_options =  NoOptions::new();
        no_options.params_mut().from = Some(from);
        no_options.params_mut().to = Some(to);
        no_options.params_mut().amount = Some(amount);

        let url = self.url().join("/conversions").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            no_options,
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let payment_methods = client.list_payment_methods().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&payment_methods).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_payment_methods(&self) -> QueryBuilder<'a, NoOptions<'a>> {
        let url = self.url().join("/payment-methods").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let coinbase_accounts = client.list_coinbase_accounts().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&coinbase_accounts).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_coinbase_accounts(&self) -> QueryBuilder<'a, NoOptions<'a>> {
        let url = self.url().join("/coinbase-accounts").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let current_fees = client.get_current_fees().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&current_fees).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_current_fees(&self) -> QueryBuilder<'a, NoOptions<'a>> {
        let url = self.url().join("/fees").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL, RPT};
    /// use chrono::{ TimeZone, Utc };
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let start_date = Utc.ymd(2018, 8, 10).and_hms(0, 0, 0);
    /// let end_date = Utc.ymd(2018, 8, 28).and_hms(0, 0, 0);
    ///
    /// let rates = client.create_report(start_date, end_date, RPT::Fills { product_id: "BTC-USD" }).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&rates).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn create_report<Tz: TimeZone>(&self, start_date: DateTime<Tz>, end_date: DateTime<Tz>, rpt: RPT<'a>) -> QueryBuilder<'a, ReportOptions<'a>> 
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
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let report_status = client.get_report_status("<report_id>").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&report_status).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_report_status(&self, report_id: &'a str) -> QueryBuilder<'a, NoOptions<'a>> {
        let endpoint = format!("/reports/:{}", report_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let profiles = client.list_profiles().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&profiles).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn list_profiles(&self) -> QueryBuilder<'a, NoOptions<'a>> {
        let url = self.url().join("/profiles").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let profile = client.get_profile("<profile_id>").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&profile).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_profile(&self, profile_id: &'a str) -> QueryBuilder<'a, NoOptions<'a>> {
        let endpoint = format!("/profiles/{}", profile_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let response = client.transfer_profile("<from_profile_id>", "<to_profile_id>", "BTC-USD", 10.00).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&response).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn transfer_profile(&self, from: &'a str, to: &'a str, currency: &'a str, amount: f64) -> QueryBuilder<'a, NoOptions<'a>> {
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
            Some(self.auth),
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{AuthenticatedClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = AuthenticatedClient::new("key", "pass", "secret", SANDBOX_URL);
    /// let trailing_volume = client.get_trailing_volume().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&trailing_volume).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_trailing_volume(&self) -> QueryBuilder<'a, NoOptions<'a>> {
        let url = self.url().join("/users/self/trailing-volume").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            NoOptions::new(),
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
    /// use cbpro::client::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let products = client.get_products().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&products).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_products<'a>(&self) -> QueryBuilder<'a, NoOptions<'a>> {
        let url = self.url.join("/products").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoOptions::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let order_book = client.get_product_order_book("BTC-USD").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&order_book).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_product_order_book<'a>(&self, product_id: &str) -> QueryBuilder<'a, BookOptions<'a>> {
        let endpoint = format!("/products/{}/book", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            BookOptions::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let ticker = client.get_product_ticker("BTC-USD").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&ticker).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_product_ticker<'a>(&self, product_id: &str) -> QueryBuilder<'a, NoOptions<'a>> {
        let endpoint = format!("/products/{}/ticker", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoOptions::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{PublicClient, SANDBOX_URL};
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
    pub fn get_trades<'a>(&self, product_id: &str) -> QueryBuilder<'a, PaginateOptions<'a>> {
        let endpoint = format!("/products/{}/trades", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            PaginateOptions::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{PublicClient, SANDBOX_URL};
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
    pub fn get_historic_rates<'a>(&self, product_id: &str, granularity: i32) -> QueryBuilder<'a, CandleOptions<'a>>{
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
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let stats = client.get_24hr_stats("BTC-USD").json().await?;
    /// println!("{}", serde_json::to_string_pretty(&stats).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_24hr_stats<'a>(&self, product_id: &str) -> QueryBuilder<'a, NoOptions<'a>> {
        let endpoint = format!("/products/{}/stats", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoOptions::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let currencies = client.get_currencies().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&currencies).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_currencies<'a>(&self) -> QueryBuilder<'a, NoOptions<'a>> {
        let url = self.url.join("/currencies").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoOptions::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::client::{PublicClient, SANDBOX_URL};
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let time = client.get_time().json().await?;
    /// println!("{}", serde_json::to_string_pretty(&time).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_time<'a>(&self) -> QueryBuilder<'a, NoOptions<'a>> {
        let url = self.url.join("/time").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            NoOptions::new(),
            None,
        )
    }
}
