# Coinbase pro client for Rust
Supports latest Future and Stream traits

## Features
- fully async private and public API
- async websocket-feed support

## Examples
Cargo.toml:
```toml
[dependencies]
cbpro = "0.1.0"
```

### Async Client
```rust
use cbpro::client::{PublicClient, SANDBOX_URL};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PublicClient::new(SANDBOX_URL);
    let products = client.get_products().json().await?;
    println!("{}", serde_json::to_string_pretty(&products).unwrap());
    Ok(())
}
```

### Async Pagination
```rust
use cbpro::client::{PublicClient, SANDBOX_URL};
use futures::stream::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PublicClient::new(SANDBOX_URL);
    let mut stream = client.get_trades("BTC-USD").paginate()?;

    while let Some(json) = stream.try_next().await? {
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        tokio_timer::delay_for(core::time::Duration::new(1, 0)).await;
    }
    Ok(())
}
```

### Async Websocket
```rust
use cbpro::websocket::{Channels, WebSocketFeed, WEBSOCKET_FEED_URL};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut feed = WebSocketFeed::connect(WEBSOCKET_FEED_URL).await?;
    feed.subscribe(&["BTC-USD"], &[Channels::LEVEL2]).await?;

    while let Some(value) = feed.try_next().await? {
        println!("{}", serde_json::to_string_pretty(&value).unwrap());
    }
    Ok(())
}
```

## Api supported:
- [] SYNC
- [x] ASYNC
- [x] Websocket-Feed

## API
- [x] Requests
- [x] Pagination
- [x] Types
- [x] Private
  - [x] Authentication
  - [x] Accounts
  - [x] Orders
  - [x] Fills
  - [x] Deposits
  - [x] Withdrawals
  - [x] Payment Methods
  - [x] Coinbase Accounts
  - [x] Reports
  - [x] User Account
- [x] Market Data
  - [x] Products
  - [x] Currencies
  - [x] Time
- [x] Websocket Feed
  - [x] heartbeat
  - [x] ticker
  - [x] level2
  - [x] user
  - [x] matches
  - [x] full

## FIX API
by request