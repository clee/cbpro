use cbpro::{AuthenticatedClient, SANDBOX_URL};
//use futures::stream::StreamExt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let secret = "WPMb2CIiMwjIWQfav5PmSzEwvSKXDH+U2bY8vfz01XKvbxJlXcFl+Ce81ynJ2YCdWwpv8m1krVDJGYx24vVrig==";
    let pass = "z7cfhsmemsj";
    let key = "a603d27ad9dbc41e1ac23486bced8165";

    let client = AuthenticatedClient::new(key, pass, secret, SANDBOX_URL);
    //let accounts = client.list_accounts().json().await?;
/*     let order = client
        .place_market_order("BTC-USD", "buy", 10.00)
        .json()
        .await?;
    println!("{}", serde_json::to_string_pretty(&order).unwrap());
     */
    let orders = client.list_orders(&["all"]).json().await?;
    println!("{}", serde_json::to_string_pretty(&orders).unwrap());

    Ok(())
}
