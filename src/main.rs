// use std::env;
use dotenv::dotenv;
use serde_json::Value;
use std::fs::File;
// use hyper_tls::HttpsConnector;
use futures::{stream, StreamExt};
use hyper::{
    Client,
    // Uri,
    // Request,
    // Method,
    // Body,
    body::HttpBody as _
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = "http://jsonplaceholder.typicode.com/posts/";
    let client = Client::new();

    let stream = stream::iter(1..=3);
    let stream = stream.map(|x| {
        let uri = format!("{}{}", url, x);

        let resp = client.get(uri.parse().unwrap());

        resp
    }).then(|resp| async {
        let mut hasil = resp.await.unwrap();

        let body_bin = hasil.body_mut().data().await.unwrap().unwrap();
        let body_str = std::str::from_utf8(&body_bin).unwrap();
        let body: Value = serde_json::from_str(&body_str).unwrap();

        body
    });

    let obj = stream.collect::<Vec<Value>>().await;

    serde_json::to_writer_pretty(File::create("uji.json").unwrap(), &obj).unwrap();

    Ok(())
}
