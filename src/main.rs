use std::env;
use dotenv::dotenv;
use hyper::{Client, Uri};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let data_url = env::var("FETCH_URL").expect("variabel FETCH_URL belum didefinisikan");
    let url = data_url.parse::<Uri>()?;
    let client = Client::new();
    let resp = client.get(url).await?;

    println!("Response: {:?}", resp.body());

    Ok(())
}
