use core::pin::Pin;
use futures::{
    future::FutureExt,
    stream::{LocalBoxStream, Stream, StreamExt},
    task::{Context, Poll},
};
use actix_web::client::Client;
use awc::{SendClientRequest, ClientResponse, error::SendRequestError};
use actix_http::{encoding::Decoder, Payload, PayloadStream};
use crate::builder::{Paginate, Params, sign_request};
use crate::client::Auth;
use serde_json::Value;

enum State {
    Start,
    Stop,
}

pub type Pages<'a> = LocalBoxStream<'a, actix_web::Result<Value>>;

pub(super) struct Paginated<'a, T> {
    in_flight: SendClientRequest,
    client: Client,
    url: String,
    query: T,
    auth: Option<Auth<'a>>,
    state: State
}

impl<'a, T: Params<'a> + Paginate<'a> + Send + Unpin + 'a> Paginated<'a, T> {
    pub(super) fn new(url: String, query: T, auth: Option<Auth<'a>>) -> Self {
        let client = Client::default();
        let request = client.get(url.clone()).query(query.params()).unwrap();

        Self {
            in_flight: sign_request(request, None, auth).send(),
            client,
            query,
            url,
            auth,
            state: State::Start,
        }
    }

    pub(super) fn pages(self) -> Pages<'a> {
        self.then(|x| async move { Ok(x?.json::<Value>().await?) }).boxed_local()
    }
}

impl<'a, T: Params<'a> + Paginate<'a> + Send + Unpin + 'a> Stream for Paginated<'a, T> {
    type Item = Result<ClientResponse<Decoder<Payload<PayloadStream>>>, SendRequestError>;

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
            let request = self.client.get(self.url.clone()).query(self.query.params()).unwrap();
            self.as_mut().in_flight = sign_request(request, None, self.auth).send();

        } else if let Some(before) = res.headers().get("cb-before") {
            self.as_mut().query.set_before(before.to_str().unwrap().parse().unwrap());
            let request = self.client.get(self.url.clone()).query(self.query.params()).unwrap();
            self.as_mut().in_flight = sign_request(request, None, self.auth).send();

        } else {
            self.as_mut().state = State::Stop;
        }
        Poll::Ready(Some(Ok(res)))
    }
}
