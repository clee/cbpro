use reqwest::{Client, Url};

pub const SANDBOX_URL: &str = "https://api-public.sandbox.pro.coinbase.com";

pub struct AuthenticatedClient<'a> {
    auth: crate::builder_v2::Auth<'a>,
    public: PublicClient,
}

impl<'a> AuthenticatedClient<'a> {
    pub fn new(key: &'a str, pass: &'a str, secret: &'a str, url: &str) -> Self {
        Self {
            auth: crate::builder_v2::Auth { key, pass, secret },
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

    pub fn list_accounts(&self) -> crate::builder_v2::QueryBuilder<'a, crate::builder_v2::BookParams<'a>> {
        let url = self.url().join("/accounts").unwrap();
        crate::builder_v2::QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            crate::builder_v2::BookParams::new(),
            Some(self.auth),
        )
    }

/*     pub fn get_account(&self, account_id: &str) -> QueryBuilder<EmptyQuery> {
        let endpoint = format!("/accounts/{}", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            EmptyQuery,
            Some(self.auth),
        )
    }
    
    pub fn get_account_history(&self, account_id: &str) -> QueryBuilder<PaginateQuery> {
        let endpoint = format!("/accounts/{}/ledger", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            PaginateQuery {
                limit: None,
                before: None,
                after: None,
            },
            Some(self.auth),
        )
    }

    pub fn get_account_holds(&self, account_id: &str) -> QueryBuilder<PaginateQuery> {
        let endpoint = format!("/accounts/{}/holds", account_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            PaginateQuery {
                limit: None,
                before: None,
                after: None,
            },
            Some(self.auth),
        )
    }

    pub fn place_limit_order(&self, product_id: &str, side: &str, price: f64, size: f64) -> QueryBuilder<LimitOrderQuery> {
        let url = self.url().join("/orders").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            LimitOrderQuery {
                client_oid: None,
                order_type: None,
                side: Some(side.to_string()),
                product_id: Some(product_id.to_string()),
                stp: None,
                stop: None,
                stop_price: None,
                price: Some(price.to_string()),
                size: Some(size.to_string()),
                time_in_force: None,
                cancel_after: None,
                post_only: None
            },
            Some(self.auth),
        )
    }

    pub fn place_market_order(&self, product_id: &str, side: &str, size: f64) -> QueryBuilder<MarketOrderQuery> {
        let url = self.url().join("/orders").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().post(url).build().unwrap(),
            MarketOrderQuery {
                client_oid: None,
                order_type: Some("market".to_string()),
                side: Some(side.to_string()),
                product_id: Some(product_id.to_string()),
                size: Some(size.to_string()),
                funds: None,
            },
            Some(self.auth),
        )
    }
    
    pub fn cancel_order(&self, order_id: &str) -> QueryBuilder<EmptyQuery> {
        let endpoint = format!("/orders/{}", order_id);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().delete(url).build().unwrap(),
            EmptyQuery,
            Some(self.auth),
        )
    }

    pub fn cancel_by_client_oid(&self, client_oid: &str) -> QueryBuilder<EmptyQuery> {
        let endpoint = format!("/orders/client:{}", client_oid);
        let url = self.url().join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().delete(url).build().unwrap(),
            EmptyQuery,
            Some(self.auth),
        )
    }

    pub fn cancel_all(&self) -> QueryBuilder<CancelAllQuery> {
        let url = self.url().join("/orders").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().delete(url).build().unwrap(),
            CancelAllQuery { product_id: None },
            Some(self.auth),
        )
    }

    pub fn list_orders(&self, status: &[&str]) -> QueryBuilder<ListOrderQuery> {
        let url = self.url().join("/orders").unwrap();
        let status: Vec<_> = status.iter().map(|x| ("status", x)).collect();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).query(&status).build().unwrap(),
            ListOrderQuery { product_id: None, limit: None, before: None, after: None },
            Some(self.auth),
        )
    } */

    
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
    pub fn get_products<'a>(&self) -> crate::builder_v2::QueryBuilder<'a, crate::builder_v2::NoParams<'a>> {
        let url = self.url.join("/products").unwrap();
        crate::builder_v2::QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            crate::builder_v2::NoParams::new(),
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
    pub fn get_product_order_book<'a>(&self, product_id: &str) -> crate::builder_v2::QueryBuilder<'a, crate::builder_v2::BookParams<'a>> {
        let endpoint = format!("/products/{}/book", product_id);
        let url = self.url.join(&endpoint).unwrap();
        crate::builder_v2::QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            crate::builder_v2::BookParams::new(),
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
    pub fn get_product_ticker<'a>(&self, product_id: &str) -> crate::builder_v2::QueryBuilder<'a, crate::builder_v2::NoParams<'a>> {
        let endpoint = format!("/products/{}/ticker", product_id);
        let url = self.url.join(&endpoint).unwrap();
        crate::builder_v2::QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            crate::builder_v2::NoParams::new(),
            None,
        )
    }
    /// # Example
    ///
    /// ```no_run
    /// use cbpro::{PublicClient, SANDBOX_URL};
    /// use futures::stream::StreamExt;
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = PublicClient::new(SANDBOX_URL);
    /// let mut stream = client.get_trades("BTC-USD").paginate();
    ///
    /// while let Some(Ok(json)) = stream.next().await {
    ///     println!("{}", serde_json::to_string_pretty(&json).unwrap());
    ///     tokio_timer::delay_for(core::time::Duration::new(1, 0)).await;
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_trades<'a>(&self, product_id: &str) -> crate::builder_v2::QueryBuilder<'a, crate::builder_v2::TradeParams<'a>> {
        let endpoint = format!("/products/{}/trades", product_id);
        let url = self.url.join(&endpoint).unwrap();
        crate::builder_v2::QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            crate::builder_v2::TradeParams::new(),
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
    /// let rates = client.get_historic_rates("BTC-USD", "3600").range(start, end).json().await?;
    /// println!("{}", serde_json::to_string_pretty(&rates).unwrap());
    /// # Ok(())
    /// # }
    /// ```
    pub fn get_historic_rates<'a>(&self, product_id: &str, granularity: &'a str) -> crate::builder_v2::QueryBuilder<'a, crate::builder_v2::CandleParams<'a>>{
        let endpoint = format!("/products/{}/candles", product_id);
        let url = self.url.join(&endpoint).unwrap();
        crate::builder_v2::QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            crate::builder_v2::CandleParams::new(granularity),
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
    pub fn get_24hr_stats<'a>(&self, product_id: &str) -> crate::builder_v2::QueryBuilder<'a, crate::builder_v2::NoParams<'a>> {
        let endpoint = format!("/products/{}/stats", product_id);
        let url = self.url.join(&endpoint).unwrap();
        crate::builder_v2::QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            crate::builder_v2::NoParams::new(),
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
    pub fn get_currencies<'a>(&self) -> crate::builder_v2::QueryBuilder<'a, crate::builder_v2::NoParams<'a>> {
        let url = self.url.join("/currencies").unwrap();
        crate::builder_v2::QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            crate::builder_v2::NoParams::new(),
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
    pub fn get_time<'a>(&self) -> crate::builder_v2::QueryBuilder<'a, crate::builder_v2::NoParams<'a>> {
        let url = self.url.join("/time").unwrap();
        crate::builder_v2::QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            crate::builder_v2::NoParams::new(),
            None,
        )
    }
}
