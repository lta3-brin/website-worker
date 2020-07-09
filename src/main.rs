// use std::env;
use dotenv::dotenv;
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
use std::fs::OpenOptions;
use std::io::Write;

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
    }).for_each(|mut body| async move{
        while let Some(chunk) = body.body_mut().data().await {
            let chunk = chunk.unwrap();

            let mut file = OpenOptions::new().append(true).create(true).open("coba.txt").expect("unable to create a file");
            file.write_all(&chunk).expect("unable to write");
        }
    }).await;

    Ok(())
}
