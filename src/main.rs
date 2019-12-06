use cbpro::{MarketData, SANDBOX_URL};
//use futures::future::FutureExt;
/* use futures::stream::StreamExt;
use tokio_timer::delay_for;
use core::time::Duration; */
use serde_json::to_string_pretty;
use chrono::offset::Utc;
use chrono::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MarketData::new(SANDBOX_URL);
/*     let mut stream = client.get_trades("BTC-USD", 100);

    while let Some(Ok(json)) = stream.next().await {
        println!("{}", to_string_pretty(&json).unwrap());
        delay_for(Duration::new(1, 0)).await;
    } */

/*     let end = Utc::now();
    let start = end - Duration::hours(5);

    let rates = client.get_historic_rates("BTC-USD", start, end , 3600).await?;
    println!("{}", to_string_pretty(&rates).unwrap()); */

    let stats = client.get_time().await?;
    println!("{}", to_string_pretty(&stats).unwrap());

    Ok(())
}

/* fn main() {
    let secret = "Nxe70DU0b8Y6zeqkXULl4slEjKmWCYW88d8tku117Gx5ZZ+JThcSnKMGHJ99Scr21LNbCVGndJ1lNNw0lzSz6A==";
    let passphrase = "ycafwe00bgh";
    let key = "bec7e2ec5953b659e6d190f9d1461caa";

    let client = MarketData::new();
} */
