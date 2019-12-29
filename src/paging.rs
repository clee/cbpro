use core::pin::Pin;
use futures::{
    future::{BoxFuture, FutureExt},
    stream::{BoxStream, Stream, StreamExt},
    task::{Context, Poll},
};
use reqwest::{Response, Client, Request};
use crate::builder::{Paginate, apply_query, Params};
use serde_json::Value;
use crate::error::{Error, CBError, Kind};

enum State {
    Start,
    Stop,
}

type ResponseFuture = BoxFuture<'static, Result<Response, reqwest::Error>>;
pub type Pages<'a> = BoxStream<'a, crate::error::Result<Value>>;

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
        self.then(|res| async move { 
            let resp = res?;
            if resp.status().is_success() {
                Ok(resp.json::<Value>().await?)
            } else {
                let error = CBError::new(resp.status().as_u16(), resp.text().await?);
                Err(error.into())
            }
             
        }).boxed()
    }
}

impl<'a, T: Params<'a> + Paginate<'a> + Send + Unpin+ 'a> Stream for Paginated<T> {
    type Item = crate::error::Result<Response>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if let State::Stop = self.state {
            return Poll::Ready(None);
        }

        let res = match self.as_mut().in_flight.poll_unpin(cx) {
            Poll::Ready(Err(e)) => {
                return Poll::Ready(Some(Err(Error::new(Kind::Client, Some(e)))));
            }
            Poll::Ready(Ok(res)) => res,
            Poll::Pending => return Poll::Pending,
        };

        if let (Some(after), None) = (res.headers().get("cb-after"), self.query.params().before) {
            self.as_mut().query.set_after(after.to_str()?.parse().unwrap());
            let mut request = self.request.try_clone().unwrap();
            request.url_mut().set_query(None);

            apply_query(&mut request, self.query.params())?;
            self.as_mut().in_flight = self.client.execute(request).boxed()

        } else if let Some(before) = res.headers().get("cb-before") {
            self.as_mut().query.set_before(before.to_str()?.parse().unwrap());
            let mut request = self.request.try_clone().unwrap();
            request.url_mut().set_query(None);

            apply_query(&mut request, self.query.params())?;
            self.as_mut().in_flight = self.client.execute(request).boxed()

        } else {
            self.as_mut().state = State::Stop;
        }
        Poll::Ready(Some(Ok(res)))
    }
}
