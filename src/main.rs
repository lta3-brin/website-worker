mod models;

use std::env;
use dotenv::dotenv;
use serde_json::Value;
use std::fs::File;
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

use models::berita::Berita;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let berita:Vec<Berita> = vec![];
    let id = env::var("UNIQUE_ID").expect("variabel UNIQUE_ID belum didefinisikan");
    let data_url_bbta3 = env::var("FETCH_URL_BBTA3").expect("variabel FETCH_URL_BBTA3 belum didefinisikan");
    let url_bbta3 = data_url_bbta3.parse::<Uri>()?;

    let https = HttpsConnector::new();
    let client = Client::builder().build::<_, Body>(https);

    let stream = stream::iter(vec![url_bbta3]);

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

        let items: Vec<Value> = serde_json::from_str(obj.as_str()).unwrap();

        for item in items {
            let mut media = "".to_string();

            if item["entities"]["media"].to_string().is_empty() {
                media = "".to_string();
            } else {
                media = item["entities"]["media"]["media_url_https"].to_string()
            }

            let kabar = Berita {
                deskripsi: item["full_text"].to_string(),
                thumbnail: media,
                tanggal: item["created_at"].to_string(),
                kategori: item["user"]["name"].to_string()
            };

            println!("{:?}", kabar)
        }
    }).await;

    // serde_json::to_writer_pretty(File::create("uji.json").unwrap(), &o).unwrap();

    Ok(())
}
