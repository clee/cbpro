use core::pin::Pin;
use futures::{
    future::{BoxFuture, FutureExt},
    stream::{BoxStream, Stream, StreamExt},
    task::{Context, Poll},
};
use reqwest::{Error, Response, Client};
use crate::builder::{PaginateQuery, apply_query};
use reqwest::Request;
use serde_json::Value;

enum State {
    Start,
    Stop,
}

type ResponseFuture = BoxFuture<'static, Result<Response, Error>>;
pub type Pages = BoxStream<'static, Result<Value, Error>>;

pub(super) struct Paginate {
    in_flight: ResponseFuture,
    client: Client,
    request: Request,
    query: PaginateQuery,
    state: State
}

impl Paginate {
    pub(super) fn new(client: Client, request: Request, query: PaginateQuery) -> Self {
        Self {
            in_flight: client.execute(request.try_clone().unwrap()).boxed(),
            client,
            request,
            query,
            state: State::Start,
        }
    }

    pub(super) fn pages(self) -> Pages {
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

        if let (Some(after), None) = (res.headers().get("cb-after"), &self.query.before) {
            self.query.after = Some(after.to_str().unwrap().to_string());
            let mut request = self.request.try_clone().unwrap();
            request.url_mut().set_query(None);

            apply_query(&mut request, &self.query);
            self.in_flight = self.client.execute(request).boxed()

        } else if let Some(before) = res.headers().get("cb-before") {
            self.query.before = Some(before.to_str().unwrap().to_string());
            let mut request = self.request.try_clone().unwrap();
            request.url_mut().set_query(None);

            apply_query(&mut request, &self.query);
            self.in_flight = self.client.execute(request).boxed()

        } else {
            self.state = State::Stop;
        }
        Poll::Ready(Some(Ok(res)))
    }
}