mod models;
mod handlers;

use std::env;
use dotenv::dotenv;
use serde_json::Value;
use hyper_tls::HttpsConnector;
use futures::{stream, StreamExt};
use hyper::{
    Client,
    Uri,
    Request,
    Method,
    Body,
    body::HttpBody as _
};

use handlers::parsing;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let id = env::var("UNIQUE_ID").expect("variabel UNIQUE_ID belum didefinisikan");

    let data_url_bbta3 = env::var("FETCH_URL_BBTA3").expect("variabel FETCH_URL_BBTA3 belum didefinisikan");
    let url_bbta3 = data_url_bbta3.parse::<Uri>()?;
    let data_url_bppt = env::var("FETCH_URL_BPPT").expect("variabel FETCH_URL_BBTA3 belum didefinisikan");
    let url_bppt = data_url_bppt.parse::<Uri>()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let stream = stream::iter(vec![url_bbta3, url_bppt]);

    stream.map(|url| {
        let req = Request::builder()
            .method(Method::GET)
            .uri(url)
            .header("content-type", "application/json")
            .header("Authorization", format!("{} {}", "bearer", id))
            .body(Body::from(""))
            .unwrap();

        let resp = client.request(req);

        resp
    }).then(|resp| async {
        let hasil = resp.await.unwrap();

        hasil
    }).for_each(|mut resp| async move {
        let mut obj = "".to_string();

        while let Some(chunk) = resp.body_mut().data().await {
            let chunk = chunk.unwrap();
            let chunk_str = std::str::from_utf8(&chunk).unwrap();

            obj.push_str(chunk_str);
        }

        let items: Value = serde_json::from_str(obj.as_str()).unwrap();

        match items.as_array() {
            None => {
                let coll = items["statuses"].as_array().expect("Gagal parsing Array statuses");

                parsing::berita(coll);
            },
            Some(coll) => {
                parsing::berita(coll);
            },
        }
    }).await;

    Ok(())
}
