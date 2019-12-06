use core::{future::Future, pin::Pin};
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

type ResponseFuture = BoxFuture<'static, Result<Response, Error>>;
pub type JsonStream = BoxStream<'static, Result<Value, Error>>;

pub(super) struct Paginate {
    in_flight: ResponseFuture,
    client: Client,
    url: Url,
    limit: String,
    state: State,
}

impl Paginate {
    pub(super) fn new(client: Client, url: Url, limit: String) -> Self {
        Self {
            in_flight: client.get(url.clone()).send().boxed(),
            client,
            url,
            limit,
            state: State::Start,
        }
    }

    pub(super) fn json(self) -> JsonStream {
        self.then(|x| async move { x?.json::<Value>().await })
            .boxed()
    }

    fn in_flight(self: Pin<&mut Self>) -> Pin<&mut ResponseFuture> {
        unsafe { Pin::map_unchecked_mut(self, |x| &mut x.in_flight) }
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
            let after = String::from(after.to_str().unwrap());
            self.in_flight = self
                .client
                .get(self.url.clone())
                .query(&[("limit", &self.limit), ("after", &after)])
                .send()
                .boxed();
        } else {
            self.state = State::Stop;
        }
        Poll::Ready(Some(Ok(res)))
    }
}