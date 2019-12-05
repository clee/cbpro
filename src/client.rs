use crate::stream::{JsonStream, Paginate};
use reqwest::{Client, Error, Method, Url};
use serde_json::Value;

pub const SANDBOX_URL: &str = "https://api-public.sandbox.pro.coinbase.com";

#[derive(Debug)]
pub struct AuthenticatedClient {
    public: PublicClient,
}

#[derive(Debug)]
pub struct PublicClient {
    client: reqwest::Client,
    url: Url,
}

impl PublicClient {
    pub fn new(url: &str) -> PublicClient {
        PublicClient {
            client: Client::new(),
            url: Url::parse(url).expect("Invalid Url"),
        }
    }

    pub async fn get_products(&self) -> Result<Value, Error> {
        let url = self.url.join("/products").unwrap();
        self.client.get(url).send().await?.json().await
    }

    pub async fn get_product_order_book(
        &self,
        product_id: &str,
        level: u32,
    ) -> Result<Value, Error> {
        let endpoint = format!("/products/{}/book", product_id);
        let url = self.url.join(&endpoint).unwrap();
        let query = &[("level", &level.to_string()[..])];
        self.client.get(url).query(query).send().await?.json().await
    }

    pub async fn get_product_ticker(&self, product_id: &str) -> Result<Value, Error> {
        let endpoint = format!("/products/{}/ticker", product_id);
        let url = self.url.join(&endpoint).unwrap();
        self.client.get(url).send().await?.json().await
    }

    pub fn get_trades(&self, product_id: &str, limit: u32) -> JsonStream {
        let endpoint = format!("/products/{}/trades", product_id);
        let url = self.url.join(&endpoint).unwrap();
        Paginate::new(
            self.client.clone(),
            Method::GET,
            url.clone(),
            limit.to_string(),
        )
        .into_json()
    }
}
