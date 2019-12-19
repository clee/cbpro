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

    //let orders = client.public().get_product_order_book("BTC-USD").level("1").json().await?;
    //println!("{}", serde_json::to_string_pretty(&orders).unwrap());

/*     let mut stream = client.public().get_trades("BTC-USD").paginate();
    
    while let Some(Ok(json)) = stream.next().await {
        println!("{}", serde_json::to_string_pretty(&json).unwrap());
        tokio_timer::delay_for(core::time::Duration::new(1, 0)).await;
    } */

    let data = client.list_accounts().json().await?;
    println!("{}", serde_json::to_string_pretty(&data).unwrap());

    Ok(())
}
