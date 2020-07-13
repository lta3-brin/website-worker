mod models;
mod helpers;
mod handlers;

use std::env;
use hyper::Uri;
use dotenv::dotenv;
use futures::{stream, StreamExt};

use models::berita::Berita;
use handlers::{
    saving,
    building
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let data_url_bbta3 = env::var("FETCH_URL_BBTA3").expect("variabel FETCH_URL_BBTA3 belum didefinisikan");
    let data_url_bppt = env::var("FETCH_URL_BPPT").expect("variabel FETCH_URL_BBTA3 belum didefinisikan");

    let url_bbta3 = data_url_bbta3.parse::<Uri>()?;
    let url_bppt = data_url_bppt.parse::<Uri>()?;

    let stream = stream::iter(vec![url_bbta3, url_bppt])
        .map(building::response)
        .then(building::process);

    let vector_berita = stream.collect::<Vec<Vec<Berita>>>().await;

    saving::save_berita(vector_berita);

    Ok(())
}
