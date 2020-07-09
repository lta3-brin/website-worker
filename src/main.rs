use std::env;
use dotenv::dotenv;
use hyper::{Client, Uri, body::HttpBody as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let data_url = env::var("FETCH_URL").expect("variabel FETCH_URL belum didefinisikan");
    let url = data_url.parse::<Uri>()?;
    let client = Client::new();
    let mut resp = client.get(url).await?;

    println!("Response: {:?}", resp.status());

    // melakukan iterasi untuk StreamBody secara asynchronous
    while let Some(chunk) = resp.body_mut().data().await {
        let bin = chunk?;
        let data = std::str::from_utf8(&bin)?;
        println!("{}", data);
    }

    Ok(())
}
