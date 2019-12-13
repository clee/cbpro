use cbpro::{AuthenticatedClient, SANDBOX_URL};
use futures::stream::StreamExt;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let secret = "Nxe70DU0b8Y6zeqkXULl4slEjKmWCYW88d8tku117Gx5ZZ+JThcSnKMGHJ99Scr21LNbCVGndJ1lNNw0lzSz6A==";
    let pass = "ycafwe00bgh";
    let key = "bec7e2ec5953b659e6d190f9d1461caa";

    let client = AuthenticatedClient::new(key, pass, secret, SANDBOX_URL);
    //let accounts = client.list_accounts().json().await?;
    let mut stream = client.public().get_trades("BTC-USD").before("7933782").paginate();

    while let Some(Ok(json)) = stream.next().await {
         println!("{}", serde_json::to_string_pretty(&json).unwrap());
         tokio_timer::delay_for(core::time::Duration::new(1, 0)).await;
    }
    //println!("{}", serde_json::to_string_pretty(&trades).unwrap());

    Ok(())
}