# Library Client for Coinbase Pro
Async only support

[Documentation](https://docs.rs/cbpro)

## Usage
Add this in your `Cargo.toml`:
```toml
[dependencies]
cbpro = "0.5.0"
futures = "0.3.1"
serde_json = "1.0.44"
tokio = { version = "0.2.10", features = ["macros", "time"] }
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
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PublicClient::new(SANDBOX_URL);
    let mut pages = client.get_trades("BTC-USD").paginate()?;

    while let Some(json) = pages.try_next().await? {
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        tokio::time::delay_for(core::time::Duration::new(1, 0)).await;
    }
    Ok(())
}
```

### Async Websocket
```rust
use cbpro::websocket::{Channels, WebSocketFeed, SANDBOX_FEED_URL};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut feed = WebSocketFeed::connect(SANDBOX_FEED_URL).await?;
    feed.subscribe(&["BTC-USD"], &[Channels::LEVEL2]).await?;

    while let Some(value) = feed.try_next().await? {
        println!("{}", serde_json::to_string_pretty(&value).unwrap());
    }
    Ok(())
}
```

## Endpoints
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

## License

Licensed under

- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)