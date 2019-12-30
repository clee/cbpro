//! Coinbase pro client with latest Future and Stream traits support.
//!
//! ## cbpro
//!
//! This crate provides an easy to use Coinbase Pro binding interface.
//! For private endpoints use [AuthenticatedClient](client/struct.AuthenticatedClient.html). For public endpoints use [PublicClient](client/struct.PublicClient.html) or [AuthenticatedClient::public](client/struct.AuthenticatedClient.html#method.public).
//! All methods beloging to the public or private client will return [QueryBuilder<'a, T>](builder/struct.QueryBuilder.html) which has split implementations per T. 
//! The final result of any operation be it methods from client or websocket-feed will resolve to [serde_json::Value](https://docs.serde.rs/serde_json/enum.Value.html).
//!
//! The public feed endpoint is also available via [WebSocketFeed::connect](websocket/struct.WebSocketFeed.html#method.connect) or the private endpoint at [WebSocketFeed::connect_auth](websocket/struct.WebSocketFeed.html#method.connect_auth).
//!
//! For more details on Coinbase Pro go to: [https://docs.pro.coinbase.com](https://docs.pro.coinbase.com).
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
//!     let products = client.get_products().json().await?;
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
//!     let mut pages = client.get_trades("BTC-USD").paginate()?;
//!
//!     while let Some(json) = pages.try_next().await? {
//!         println!("{}", serde_json::to_string_pretty(&json).unwrap());
//!         tokio_timer::delay_for(core::time::Duration::new(1, 0)).await;
//!     }
//!     Ok(())
//! }
//! ```
//! ### Async Websocket
//! ```no_run
//! use cbpro::websocket::{Channels, WebSocketFeed, SANDBOX_FEED_URL};
//! use futures::TryStreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let mut feed = WebSocketFeed::connect(SANDBOX_FEED_URL).await?;
//!     feed.subscribe(&["BTC-USD"], &[Channels::LEVEL2]).await?;
//!
//!     while let Some(value) = feed.try_next().await? {
//!         println!("{}", serde_json::to_string_pretty(&value).unwrap());
//!     }
//!     Ok(())
//! }
//! ```
/// Builder and types representing optional methods
pub mod builder;
/// Public and private clients
pub mod client;
/// All possible errors
pub mod error;
mod paging;
/// Public and Private websocket feed
pub mod websocket;

pub use self::client::{AuthenticatedClient, PublicClient};
pub use self::paging::Pages;
pub use self::websocket::WebSocketFeed;