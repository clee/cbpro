#![allow(dead_code)]
#![allow(unused_imports)]

use core::{future::Future, pin::Pin};
use futures::{
    future::{BoxFuture, FutureExt},
    stream::{Stream, StreamExt, TryStream},
    task::{Context, Poll},
};
use reqwest::header::HeaderMap;
use reqwest::{Client, Error, Method, Request, RequestBuilder, Response, Url};
use serde_json::Value;
use std::borrow::Cow;
use std::sync::Arc;

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
            client: reqwest::Client::new(),
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

    pub fn get_trades(&self, product_id: &str, limit: u32) -> Paginate {
        let endpoint = format!("/products/{}/trades", product_id);
        let url = self.url.join(&endpoint).unwrap();
        //let query = &[("limit", &limit.to_string()[..])];
        //let req = self.client.get(url).query(query).build().unwrap();

        Paginate {
            in_flight: ResponseFuture::new(Box::new(self.client.get(url.clone()).send())),
            client: self.client.clone(),
            method: Method::GET,
            url: url.to_string(),
            after: String::new(),
            limit: 100,
            state: State::Start,
        }
    }

    pub async fn get_trades2(&self, product_id: &str, limit: &str) -> Result<Response, Error> {
        let endpoint = format!("/products/{}/trades", product_id);
        let url = self.url.join(&endpoint).unwrap();
        let query = &[("limit", limit)];
        let req = self.client.get(url).query(query).build().unwrap();

        let resp = self.client.execute(req).await?;
        Ok(resp)
    }
}

enum State {
    Start,
    Stop,
}

pub struct Paginate {
    in_flight: ResponseFuture,
    client: Client,
    method: Method,
    url: String,
    after: String,
    limit: u32,
    state: State,
}

impl Paginate {
    fn in_flight(self: Pin<&mut Self>) -> Pin<&mut ResponseFuture> {
        unsafe { Pin::map_unchecked_mut(self, |x| &mut x.in_flight) }
    }

    fn after(self: Pin<&mut Self>) -> &mut String {
        unsafe { &mut Pin::get_unchecked_mut(self).after }
    }

    fn state(self: Pin<&mut Self>) -> &mut State {
        unsafe { &mut Pin::get_unchecked_mut(self).state }
    }
}

impl Stream for Paginate {
    type Item = Result<Response, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let State::Stop = self.state {
            return Poll::Ready(None);
        }

        let res = match self.as_mut().in_flight().as_mut().poll(cx) {
            Poll::Ready(Err(e)) => {
                return Poll::Ready(Some(Err(e)));
            }
            Poll::Ready(Ok(res)) => res,
            Poll::Pending => return Poll::Pending,
        };

        if let Some(after) = res.headers().get("cb-after") {
            println!("cb-before_mut {}", &self.after);
            let limit = self.limit.to_string();

            self.after = String::from(after.to_str().unwrap());
            println!("cb-after_mut {}", &self.after);

            *self.as_mut().in_flight().get_mut() = ResponseFuture::new(Box::new(
                self.client.request(self.method.clone(), Url::parse(&self.url).unwrap()).query(&[("limit", limit), ("after", String::from(&self.after))]).send(),
            ));
        } else {
            *self.as_mut().state() = State::Stop;
        }

        Poll::Ready(Some(Ok(res)))
    }
}

struct ResponseFuture {
    inner: Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>>,
}

impl ResponseFuture {
    fn new(fut: Box<dyn Future<Output = Result<Response, Error>> + Send>) -> Self {
        Self { inner: fut.into() }
    }
}

impl Future for ResponseFuture {
    type Output = Result<Response, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx)
    }
}
