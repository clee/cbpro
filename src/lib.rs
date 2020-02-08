//! Coinbase pro async client.
//!
//! ## cbpro
//!
//! This crate provides an easy to use Coinbase Pro API wrapper. For private endpoints use [AuthenticatedClient](client/struct.AuthenticatedClient.html). 
//! For public endpoints use [PublicClient](client/struct.PublicClient.html) or [AuthenticatedClient::public](client/struct.AuthenticatedClient.html#method.public). 
//! All methods belonging to the public or private client will return [QueryBuilder<T>](builder/struct.QueryBuilder.html) which has split implementations per T.
//! 
//! The websocket can be found here: [WebSocketFeed](websocket/struct.WebSocketFeed.html).
//! For more details on Coinbase Pro go to [https://docs.pro.coinbase.com](https://docs.pro.coinbase.com).
//!
//! ## Examples
//!
//! ### Async Client
//! ```no_run
//! use cbpro::client::{PublicClient, SANDBOX_URL};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = PublicClient::new(SANDBOX_URL);
//!     let products = client.get_products().json::<serde_json::Value>().await?;
//!     println!("{}", serde_json::to_string_pretty(&products).unwrap());
//!     Ok(())
//! }
//! ```
//! ### Async Pagination
//! ```no_run
//! use cbpro::client::{PublicClient, SANDBOX_URL};
//! use futures::TryStreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = PublicClient::new(SANDBOX_URL);
//!     let mut pages = client
//!         .get_trades("BTC-USD")
//!         .paginate::<serde_json::Value>()?;
//!
//!     while let Some(json) = pages.try_next().await? {
//!         println!("{}", serde_json::to_string_pretty(&json).unwrap());
//!         tokio::time::delay_for(core::time::Duration::new(1, 0)).await;
//!     }
//!     Ok(())
//! }
//! ```
//! ### Async Websocket
//! ```no_run
//! use cbpro::websocket::{Channels, WebSocketFeed, SANDBOX_FEED_URL};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut feed = WebSocketFeed::connect(SANDBOX_FEED_URL).await?;
//!     feed.subscribe(&["BTC-USD"], &[Channels::LEVEL2]).await?;
//!
//!     while let Some(value) = feed.json::<serde_json::Value>().await? {
//!         println!("{}", serde_json::to_string_pretty(&value).unwrap());
//!     }
//!     Ok(())
//! }
//! ```
/// Builder and types representing optional methods
pub mod builder;
/// Public and private clients
pub mod client;
/// Errors of this crate
pub mod error;
mod paging;
/// Public and private websocket feed
pub mod websocket;

pub use self::client::{AuthenticatedClient, PublicClient};
pub use self::paging::Pages;
pub use self::websocket::WebSocketFeed;