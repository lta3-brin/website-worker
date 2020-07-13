use std::env;
use serde_json::Value;
use hyper_tls::HttpsConnector;
use hyper::{Client, Method, Request, Body, Uri, body::HttpBody as _};
use hyper::client::ResponseFuture;

use crate::handlers::parsing;
use crate::models::berita::Berita;

pub fn response(url: Uri) -> ResponseFuture {
    let id = env::var("UNIQUE_ID").expect("variabel UNIQUE_ID belum didefinisikan");
    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let req = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("content-type", "application/json")
        .header("Authorization", format!("{} {}", "bearer", id))
        .body(Body::from(""))
        .unwrap();

    let resp = client.request(req);

    resp
}

pub async fn process(resp: ResponseFuture) -> Vec<Berita> {
    let mut hasil = resp.await.unwrap();

    let mut obj = "".to_string();

    while let Some(chunk) = hasil.body_mut().data().await {
        let chunk = chunk.unwrap();
        let chunk_str = std::str::from_utf8(&chunk).unwrap();

        obj.push_str(chunk_str);
    }

    let items: Value = serde_json::from_str(obj.as_str()).unwrap();

    match items.as_array() {
        None => {
            if items["kind"].to_string().contains("youtube") {
                let coll = items["items"].as_array()
                    .expect("Gagal parsing Array video items");

                parsing::berita(coll)
            } else {
                let coll = items["statuses"].as_array()
                    .expect("Gagal parsing Array twitter statuses");

                parsing::berita(coll)
            }
        },
        Some(coll) => {
            parsing::berita(coll)
        },
    }
}
