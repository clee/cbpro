use crate::builder::{
    ArgBuilder, CandleArgs, NoArgs, PaginateArgs, BookArgs,
};
use reqwest::{Client, Url};

pub const SANDBOX_URL: &str = "https://api-public.sandbox.pro.coinbase.com";

pub struct AuthenticatedClient {
    public: PublicClient,
}

impl AuthenticatedClient {
    fn new(url: &str) -> Self {
        Self {
            public: PublicClient::new(url),
        }
    }

    fn client(&self) -> &Client {
        &self.public.client
    }

    fn url(&self) -> &Url {
        &self.public.url
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
    pub fn get_products(&self) -> ArgBuilder<NoArgs> {
        let url = self.url.join("/products").unwrap();
        ArgBuilder::new(self.client.get(url), NoArgs)
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
    pub fn get_product_order_book(&self, product_id: &str) -> ArgBuilder<BookArgs> {
        let endpoint = format!("/products/{}/book", product_id);
        let url = self.url.join(&endpoint).unwrap();
        ArgBuilder::new(self.client.get(url), BookArgs { level: None })
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
    pub fn get_product_ticker(&self, product_id: &str) -> ArgBuilder<NoArgs> {
        let endpoint = format!("/products/{}/ticker", product_id);
        let url = self.url.join(&endpoint).unwrap();
        ArgBuilder::new(self.client.get(url), NoArgs)
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
    pub fn get_trades(&self, product_id: &str) -> ArgBuilder<PaginateArgs> {
        let endpoint = format!("/products/{}/trades", product_id);
        let url = self.url.join(&endpoint).unwrap();
        ArgBuilder::new(
            self.client.get(url),
            PaginateArgs {
                limit: None,
                before: None,
                after: None,
            },
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
    pub fn get_historic_rates(
        &self,
        product_id: &str,
        granularity: u32,
    ) -> ArgBuilder<CandleArgs> {
        let endpoint = format!("/products/{}/candles", product_id);
        let url = self.url.join(&endpoint).unwrap();
        ArgBuilder::new(
            self.client.get(url),
            CandleArgs {
                start: None,
                end: None,
                granularity: Some(granularity.to_string()),
            },
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
    pub fn get_24hr_stats(&self, product_id: &str) -> ArgBuilder<NoArgs> {
        let endpoint = format!("/products/{}/stats", product_id);
        let url = self.url.join(&endpoint).unwrap();
        ArgBuilder::new(self.client.get(url), NoArgs)
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
    pub fn get_currencies(&self) -> ArgBuilder<NoArgs> {
        let url = self.url.join("/currencies").unwrap();
        ArgBuilder::new(self.client.get(url), NoArgs)
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
    pub fn get_time(&self) -> ArgBuilder<NoArgs> {
        let url = self.url.join("/time").unwrap();
        ArgBuilder::new(self.client.get(url), NoArgs)
    }
}
