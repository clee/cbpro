#![allow(dead_code)]
#![allow(unused_imports)]

use core::future::Future;
use reqwest::Response;

struct AuthenticatedClient {
    public: PublicClient,
}

pub struct PublicClient {
    client: reqwest::Client,
}

impl PublicClient {
    pub fn new() -> PublicClient {
        PublicClient {
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_products(&self) -> Result<String, reqwest::Error> {
        self.client
            .get("http://httpbin.org/range/26")
            .send()
            .await?
            .text()
            .await
    }
}
