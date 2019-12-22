use core::pin::Pin;
use futures::{
    future::{BoxFuture, FutureExt},
    stream::{BoxStream, Stream, StreamExt},
    task::{Context, Poll},
};
use reqwest::{Error, Response, Client};
use crate::builder::{Paginate, apply_query, Params};
use reqwest::Request;
use serde_json::Value;

enum State {
    Start,
    Stop,
}

type ResponseFuture = BoxFuture<'static, Result<Response, Error>>;
pub type Pages<'a> = BoxStream<'a, Result<Value, Error>>;

pub(super) struct Paginated<T> {
    in_flight: ResponseFuture,
    client: Client,
    request: Request,
    query: T,
    state: State
}

impl<'a, T: Params<'a> + Paginate<'a> + Send + Unpin + 'a> Paginated<T> {
    pub(super) fn new(client: Client, request: Request, query: T) -> Self {
        Self {
            in_flight: client.execute(request.try_clone().unwrap()).boxed(),
            client,
            request,
            query,
            state: State::Start,
        }
    }

    pub(super) fn pages(self) -> Pages<'a> {
        self.then(|x| async move { x?.json::<Value>().await }).boxed()
    }
}

impl<'a, T: Params<'a> + Paginate<'a> + Send + Unpin+ 'a> Stream for Paginated<T> {
    type Item = Result<Response, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let State::Stop = self.state {
            return Poll::Ready(None);
        }

        let res = match self.as_mut().in_flight.poll_unpin(cx) {
            Poll::Ready(Err(e)) => {
                return Poll::Ready(Some(Err(e)));
            }
            Poll::Ready(Ok(res)) => res,
            Poll::Pending => return Poll::Pending,
        };

        if let (Some(after), None) = (res.headers().get("cb-after"), self.query.params().before) {
            self.as_mut().query.set_after(after.to_str().unwrap().parse().unwrap());
            let mut request = self.request.try_clone().unwrap();
            request.url_mut().set_query(None);

            apply_query(&mut request, self.query.params());
            self.as_mut().in_flight = self.client.execute(request).boxed()

        } else if let Some(before) = res.headers().get("cb-before") {
            self.as_mut().query.set_before(before.to_str().unwrap().parse().unwrap());
            let mut request = self.request.try_clone().unwrap();
            request.url_mut().set_query(None);

            apply_query(&mut request, self.query.params());
            self.as_mut().in_flight = self.client.execute(request).boxed()

        } else {
            self.as_mut().state = State::Stop;
        }
        Poll::Ready(Some(Ok(res)))
    }
}
