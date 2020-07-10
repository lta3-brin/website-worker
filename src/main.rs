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

// const PARALLEL_REQUESTS: usize = 2;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let url = "http://jsonplaceholder.typicode.com/posts/";
    let client = Client::new();

    let stream = stream::iter(1..=3);
    stream.map(|x| {
        let uri = format!("{}{}", url, x);

        let resp = client.get(uri.parse().unwrap());

        resp
    }).then(|resp| async {
        let hasil = resp.await.unwrap();

        hasil
    }).for_each(|mut body| async move {
        while let Some(chunk) = body.body_mut().data().await {
            let chunk = chunk.unwrap();
            let obj_str = std::str::from_utf8(&chunk).unwrap();
            let obj: Value = serde_json::from_str(&obj_str).unwrap();

            serde_json::to_writer_pretty(File::create("uji.json").unwrap(), &obj)
                .expect("Tidak dapat membuat file json")
        }
    }).await;

    Ok(())
}
