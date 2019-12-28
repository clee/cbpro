use cbpro::{WebSocketFeed, WEBSOCKET_FEED_URL, Channels};
use futures::TryStreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
/*     let secret = "WPMb2CIiMwjIWQfav5PmSzEwvSKXDH+U2bY8vfz01XKvbxJlXcFl+Ce81ynJ2YCdWwpv8m1krVDJGYx24vVrig==";
    let pass = "z7cfhsmemsj";
    let key = "a603d27ad9dbc41e1ac23486bced8165"; */

/*     let client = AuthenticatedClient::new(key, pass, secret, SANDBOX_URL);
    let data = client.list_accounts().json().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap()); */

    let mut feed = WebSocketFeed::connect(WEBSOCKET_FEED_URL).await?;
    feed.subscribe(&["BTC-USD"], &[Channels::TICKER]).await?;

    while let Some(value) = feed.try_next().await? {
        println!("{}", serde_json::to_string_pretty(&value).unwrap());
    }

    Ok(())
}