use crate::stream::{Json, Paginate};
use chrono::{offset::TimeZone, DateTime};
use reqwest::Error;
use reqwest::RequestBuilder;
use serde::Serialize;
use serde_json::Value;

#[derive(Serialize)]
pub struct NoOptionalParams;

#[derive(Serialize)]
pub struct ProductOrderBookParams {
    pub level: Option<String>,
}

#[derive(Serialize)]
pub struct HistoricRatesParams {
    pub start: Option<String>,
    pub end: Option<String>,
    pub granularity: Option<String>,
}

#[derive(Serialize)]
pub struct PaginateParams {
    pub limit: Option<String>,
    pub before: Option<String>,
    pub after: Option<String>,
}

pub struct ArgBuilder<T: Serialize> {
    request_builder: RequestBuilder,
    serializable: T,
}

impl<T: Serialize> ArgBuilder<T> {
    pub(super) fn new(request_builder: reqwest::RequestBuilder, serializable: T) -> Self {
        Self {
            request_builder,
            serializable,
        }
    }

    pub async fn json(self) -> Result<Value, Error> {
        self.request_builder
            .query(&self.serializable)
            .send()
            .await?
            .json()
            .await
    }
}

impl ArgBuilder<ProductOrderBookParams> {
    pub fn level(mut self, value: u32) -> Self {
        self.serializable.level = Some(value.to_string());
        self
    }
}

impl ArgBuilder<PaginateParams> {
    pub fn limit(mut self, value: u32) -> Self {
        self.serializable.limit = Some(value.to_string());
        self
    }

    pub fn before(mut self, value: &str) -> Self {
        self.serializable.before = Some(value.to_string());
        self.serializable.after = None;
        self
    }

    pub fn after(mut self, value: &str) -> Self {
        self.serializable.after = Some(value.to_string());
        self.serializable.before = None;
        self
    }

    pub fn paginate(self) -> Json {
        Paginate::new(self.request_builder, self.serializable).json()
    }
}

impl ArgBuilder<HistoricRatesParams> {
    pub fn range<Tz: TimeZone>(mut self, start: DateTime<Tz>, end: DateTime<Tz>) -> Self
    where
        Tz::Offset: core::fmt::Display,
    {
        self.serializable.start = Some(start.to_rfc3339());
        self.serializable.end = Some(end.to_rfc3339());
        self
    }
}
