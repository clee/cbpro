use rustify::PublicClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = PublicClient::new();
    let data = client.get_products().await?;

    println!("{:#?}", data);
    Ok(())
}
