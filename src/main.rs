use cbpro::{PublicClient, SANDBOX_URL};
//use futures::future::FutureExt;
use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PublicClient::new(SANDBOX_URL);

    while let Some(Ok(_)) = client.get_trades("BTC-USD", 100).next().await {
        //println!("{:?}", resp)
    }

/*     let resp = client.get_trades2("BTC-USD", "100").await;
    println!("{:?}", resp.unwrap());

    
    let resp = client.get_trades2("BTC-USD", "100").await;
    println!("{:?}", resp.unwrap()); */

    Ok(())
}

/* fn main() {
    let secret = "Nxe70DU0b8Y6zeqkXULl4slEjKmWCYW88d8tku117Gx5ZZ+JThcSnKMGHJ99Scr21LNbCVGndJ1lNNw0lzSz6A==";
    let passphrase = "ycafwe00bgh";
    let key = "bec7e2ec5953b659e6d190f9d1461caa";

    let client = PublicClient::new();
} */
