use reqwest::RequestBuilder;
use serde::Serialize;
use reqwest::Error;
use serde_json::Value;
use crate::stream::{Paginate, Json};

pub struct ArgBuilder<T: Serialize> {
    request_builder: RequestBuilder,
    serializable: T
}

impl<T: Serialize> ArgBuilder<T> {
    pub(super) fn new(request_builder: reqwest::RequestBuilder, serializable: T) -> Self {
        Self {
            request_builder,
            serializable,
        }
    }

    pub async fn json(self) -> Result<Value, Error> {
        self.request_builder.query(&self.serializable).send().await?.json().await
    }
}

impl ArgBuilder<ProductOrderBookParams> {
    pub fn level(mut self, value: u32) -> Self {
        self.serializable.level = Some(value.to_string());
        self
    }
}

impl ArgBuilder<PaginatedParams> {
    pub fn limit(mut self, limit: u32) -> Self {
        self.serializable.limit = Some(limit.to_string());
        self
    }

    pub fn before(mut self, before: u32) -> Self {
        self.serializable.before = Some(before.to_string());
        self.serializable.after = None;
        self
    }

    pub fn after(mut self, after: u32) -> Self {
        self.serializable.after = Some(after.to_string());
        self.serializable.before = None;
        self
    }

    pub fn paginate(self) -> Json {
        Paginate::new(self.request_builder, self.serializable).json()
    }
}

#[derive(Serialize)]
pub struct NoOptionalParams;

#[derive(Serialize)]
pub struct ProductOrderBookParams {
    pub level: Option<String>
}

#[derive(Serialize)]
pub struct PaginatedParams {
    pub limit: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>
}