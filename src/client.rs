use reqwest::{Client, Error, Method, Response, Url};
use serde_json::Value;
use crate::streams::{Paginate, ResponseFuture, JsonStream};
use futures::stream::{StreamExt, Then};

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
   
        let stream = Paginate::new(
            ResponseFuture::new(Box::new(self.client.get(url.clone()).send())),
            self.client.clone(),
            Method::GET,
            url.clone(),
            String::new(),
            limit.to_string(),
        );

        JsonStream::new(Box::new(stream.then(|x| async move { x?.json::<Value>().await })))
    }
}