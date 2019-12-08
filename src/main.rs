use cbpro::{PublicClient, SANDBOX_URL};
//use futures::future::FutureExt;
use futures::stream::StreamExt;
use tokio_timer::delay_for;
use core::time::Duration;
use serde_json::to_string_pretty;
/* use chrono::offset::Utc;
use chrono::Duration; */

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

/*     let mut query: Vec<(&str, &str)> = Vec::new();
    let mut params = vec![("limit", Some("heloo")), ("before", Some("wtf")), ("after", Some("great"))];
    println!("{:?}", params);

    while let Some((param, Some(value))) = params.pop() {
        query.push((param, value));
    }
    println!("{:?}", query); */

    let client = PublicClient::new(SANDBOX_URL);
    let mut stream = client.get_trades("BTC-USD").limit("2").before("7833638").paginate();

    while let Some(Ok(json)) = stream.next().await {
        println!("{}", to_string_pretty(&json).unwrap());
        delay_for(Duration::new(5, 0)).await;
    }

/*     let end = Utc::now();
    let start = end - Duration::hours(5);

    let rates = client.get_historic_rates("BTC-USD", start, end , 3600).await?;
    println!("{}", to_string_pretty(&rates).unwrap()); */

/*     let stats = client.get_products().await?;
    println!("{}", to_string_pretty(&stats).unwrap()); */

    Ok(())
}

/* fn main() {
    let secret = "Nxe70DU0b8Y6zeqkXULl4slEjKmWCYW88d8tku117Gx5ZZ+JThcSnKMGHJ99Scr21LNbCVGndJ1lNNw0lzSz6A==";
    let passphrase = "ycafwe00bgh";
    let key = "bec7e2ec5953b659e6d190f9d1461caa";

    let client = PublicClient::new();
} */
