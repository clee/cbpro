use core::pin::Pin;
use futures::{
    future::{BoxFuture, FutureExt},
    stream::{BoxStream, Stream, StreamExt},
    task::{Context, Poll},
};
use reqwest::{Client, Error, Response, Url};
use serde_json::Value;

enum State {
    Start,
    Stop,
}

type ResponseFuture<'a> = BoxFuture<'a, Result<Response, Error>>;
pub type Json<'a> = BoxStream<'a, Result<Value, Error>>;

pub(super) struct Paginate<'a> {
    in_flight: ResponseFuture<'a>,
    client: Client,
    url: Url,
    params: Vec<(&'a str, Option<&'a str>)>,
    state: State,
}

impl<'a> Paginate<'a> {
    pub(super) fn new(client: Client, url: Url, params: Vec<(&'a str, Option<&'a str>)>) -> Self {
        let mut query = Vec::new();

        for param in &params {
            if let (param, Some(value)) = param {
                query.push((param, value));
            }
        }

        Self {
            in_flight: client.get(url.clone()).query(&query).send().boxed(),
            client,
            url,
            params,
            state: State::Start,
        }
    }

    pub(super) fn json(self) -> Json<'a> {
        self.then(|x| async move { x?.json::<Value>().await })
            .boxed()
    }
}

impl<'a> Stream for Paginate<'a> {
    type Item = Result<Response, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let State::Stop = self.state {
            return Poll::Ready(None);
        }

        let res = match self.in_flight.poll_unpin(cx) {
            Poll::Ready(Err(e)) => {
                return Poll::Ready(Some(Err(e)));
            }
            Poll::Ready(Ok(res)) => res,
            Poll::Pending => return Poll::Pending,
        };

        if let (Some(after), None) = (res.headers().get("cb-after"), self.params[1].1) {
            let mut query: Vec<(&str, &str)> = Vec::new();
            query.push(("after", after.to_str().unwrap()));

            if let Some(limit) = self.params[0].1 {
                query.push(("limit", limit))
            };

            self.in_flight = self
                .client
                .get(self.url.clone())
                .query(&query)
                .send()
                .boxed();
        } else if let Some(before) = res.headers().get("cb-before") {
            let mut query: Vec<(&str, &str)> = Vec::new();
            query.push(("before", before.to_str().unwrap()));

            if let Some(limit) = self.params[0].1 {
                query.push(("limit", limit))
            };

            self.in_flight = self
                .client
                .get(self.url.clone())
                .query(&query)
                .send()
                .boxed();
        } else {
            self.state = State::Stop;
        }
        Poll::Ready(Some(Ok(res)))
    }
}

pub struct PaginateBuilder<'a> {
    client: Client,
    url: Url,
    params: Vec<(&'a str, Option<&'a str>)>,
}

impl<'a> PaginateBuilder<'a> {
    pub(super) fn new(client: Client, url: Url) -> Self {
        Self {
            client,
            url,
            params: vec![("limit", None), ("before", None), ("after", None)],
        }
    }

    pub fn limit(mut self, limit: &'a str) -> Self {
        self.params[0].1 = Some(limit);
        self
    }

    pub fn before(mut self, before: &'a str) -> Self {
        self.params[0].1 = None;
        self.params[1].1 = Some(before);
        self.params[2].1 = None;
        self
    }

    pub fn after(mut self, after: &'a str) -> Self {
        self.params[2].1 = Some(after);
        self.params[1].1 = None;
        self
    }

    pub fn paginate(self) -> Json<'a> {
        Paginate::new(self.client, self.url, self.params).json()
    }
}
