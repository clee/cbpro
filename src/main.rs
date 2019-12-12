use cbpro::{AuthenticatedClient, SANDBOX_URL};
use serde_json::to_string_pretty;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let secret = "Nxe70DU0b8Y6zeqkXULl4slEjKmWCYW88d8tku117Gx5ZZ+JThcSnKMGHJ99Scr21LNbCVGndJ1lNNw0lzSz6A==";
    let pass = "ycafwe00bgh";
    let key = "bec7e2ec5953b659e6d190f9d1461caa";

    let client = AuthenticatedClient::new(key, pass, secret, SANDBOX_URL);
    let accounts = client.list_accounts().json().await?;

    println!("{}", to_string_pretty(&accounts).unwrap());

    Ok(())
}