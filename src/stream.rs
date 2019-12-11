use core::pin::Pin;
use futures::{
    future::{BoxFuture, FutureExt},
    stream::{BoxStream, Stream, StreamExt},
    task::{Context, Poll},
};
use reqwest::{Error, Response};
use crate::builder::PaginateParams;
use reqwest::RequestBuilder;
use serde_json::Value;

enum State {
    Start,
    Stop,
}

type ResponseFuture = BoxFuture<'static, Result<Response, Error>>;
pub type Json = BoxStream<'static, Result<Value, Error>>;

pub(super) struct Paginate {
    in_flight: ResponseFuture,
    request_builder: RequestBuilder,
    params: PaginateParams,
    state: State
}

impl Paginate {
    pub(super) fn new(request_builder: RequestBuilder, params: PaginateParams) -> Self {

        Self {
            in_flight: request_builder.try_clone().unwrap().query(&params).send().boxed(),
            request_builder,
            params,
            state: State::Start,
        }
    }

    pub(super) fn json(self) -> Json {
        self.then(|x| async move { x?.json::<Value>().await })
            .boxed()
    }
}

impl Stream for Paginate {
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

        if let (Some(after), None) = (res.headers().get("cb-after"), &self.params.before) {
            self.params.after = Some(after.to_str().unwrap().to_string());

            self.in_flight = self.request_builder.try_clone().unwrap()
                .query(&self.params)
                .send()
                .boxed();
        } else if let Some(before) = res.headers().get("cb-before") {
            self.params.before = Some(before.to_str().unwrap().to_string());

            self.in_flight = self.request_builder.try_clone().unwrap()
                .query(&self.params)
                .send()
                .boxed();
        } else {
            self.state = State::Stop;
        }
        Poll::Ready(Some(Ok(res)))
    }
}