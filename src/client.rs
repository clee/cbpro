use crate::builder::{
    QueryBuilder, 
    Auth, 
    BookQuery, 
    CandleQuery, 
    EmptyQuery, 
    PaginateQuery,
    LimitOrderQuery
};
use reqwest::{Client, Url};

pub const SANDBOX_URL: &str = "https://api-public.sandbox.pro.coinbase.com";

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

    pub fn list_accounts(&self) -> QueryBuilder<EmptyQuery> {
        let url = self.url().join("/accounts").unwrap();
        QueryBuilder::new(
            self.client().clone(),
            self.client().get(url).build().unwrap(),
            EmptyQuery,
            Some(self.auth),
        )
    }

    pub fn get_account(&self, account_id: &str) -> QueryBuilder<EmptyQuery> {
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
    pub fn get_products(&self) -> QueryBuilder<EmptyQuery> {
        let url = self.url.join("/products").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            EmptyQuery,
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
    pub fn get_product_order_book(&self, product_id: &str) -> QueryBuilder<BookQuery> {
        let endpoint = format!("/products/{}/book", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            BookQuery { level: None },
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
    pub fn get_product_ticker(&self, product_id: &str) -> QueryBuilder<EmptyQuery> {
        let endpoint = format!("/products/{}/ticker", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            EmptyQuery,
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
    pub fn get_trades(&self, product_id: &str) -> QueryBuilder<PaginateQuery> {
        let endpoint = format!("/products/{}/trades", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            PaginateQuery {
                limit: None,
                before: None,
                after: None,
            },
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
    pub fn get_historic_rates(&self, product_id: &str, granularity: u32) -> QueryBuilder<CandleQuery> {
        let endpoint = format!("/products/{}/candles", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            CandleQuery {
                start: None,
                end: None,
                granularity: Some(granularity.to_string()),
            },
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
    pub fn get_24hr_stats(&self, product_id: &str) -> QueryBuilder<EmptyQuery> {
        let endpoint = format!("/products/{}/stats", product_id);
        let url = self.url.join(&endpoint).unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            EmptyQuery,
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
    pub fn get_currencies(&self) -> QueryBuilder<EmptyQuery> {
        let url = self.url.join("/currencies").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            EmptyQuery,
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
    pub fn get_time(&self) -> QueryBuilder<EmptyQuery> {
        let url = self.url.join("/time").unwrap();
        QueryBuilder::new(
            self.client.clone(),
            self.client.get(url).build().unwrap(),
            EmptyQuery,
            None,
        )
    }
}
