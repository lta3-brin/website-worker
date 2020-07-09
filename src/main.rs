use std::env;
use dotenv::dotenv;
use hyper_tls::HttpsConnector;
use hyper::{Client, Uri, Request, Method, Body, body::HttpBody as _};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let id = env::var("UNIQUE_ID").expect("variabel UNIQUE_ID belum didefinisikan");
    let data_url = env::var("FETCH_URL_BBTA3").expect("variabel FETCH_URL_BBTA3 belum didefinisikan");
    let url = data_url.parse::<Uri>()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);
    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("content-type", "application/json")
        .header("Authorization", format!("{} {}", "bearer", id))
        .body(Body::from(""))
        .unwrap();

    let mut resp = client.request(req).await?;

    println!("Response: {:?}", resp.status());

    // melakukan iterasi untuk StreamBody secara asynchronous
    while let Some(chunk) = resp.body_mut().data().await {
        let bin = chunk?;
        let data = std::str::from_utf8(&bin)?;
        println!("{}", data);
    }

    Ok(())
}
