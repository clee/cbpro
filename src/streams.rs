use core::{
    future::Future, 
    pin::Pin
};
use futures::{
    stream::Stream,
    task::{Context, Poll},
};
use reqwest::{Client, Error, Method, Response, Url};
use serde_json::Value;


pub enum State {
    Start,
    Stop,
}

pub struct Paginate {
    in_flight: ResponseFuture,

    client: Client,
    method: Method,
    url: Url,

    after: String,
    limit: String,

    state: State,
}

impl Paginate {
    pub fn new(
        in_flight: ResponseFuture,
        client: Client,
        method: Method,
        url: Url,
        after: String,
        limit: String
    ) -> Self {
        Self {
            in_flight,
            client,
            method,
            url,
            after,
            limit,
            state: State::Start,
        }
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
            println!("cb-after {:?}", after);

            self.after = String::from(after.to_str().unwrap());

            self.in_flight = ResponseFuture::new(Box::new(
                self.client.request(self.method.clone(), self.url.clone()).query(&[("limit", &self.limit), ("after", &self.after)]).send(),
            ));

        } else {
            self.state = State::Stop;
        }

        Poll::Ready(Some(Ok(res)))
    }
}

pub struct ResponseFuture {
    inner: Pin<Box<dyn Future<Output = Result<Response, Error>> + Send>>,
}

impl ResponseFuture {
    pub fn new(fut: Box<dyn Future<Output = Result<Response, Error>> + Send>) -> Self {
        Self { inner: fut.into() }
    }
}

impl Future for ResponseFuture {
    type Output = Result<Response, Error>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.inner).poll(cx)
    }
}

pub struct JsonStream {
    inner: Pin<Box<dyn Stream<Item = Result<Value, Error>> + Send>>,
}

impl JsonStream {
    pub fn new(stream: Box<dyn Stream<Item = Result<Value, Error>> + Send>) -> Self {
        Self { inner: stream.into() }
    }
}

impl Stream for JsonStream {
    type Item = Result<Value, Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }
}
