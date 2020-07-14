mod models;
mod helpers;
mod handlers;

use std::env;
use hyper::Uri;
use dotenv::dotenv;
use clap::{Arg, App};
use futures::{stream, StreamExt};

use models::berita::Berita;
use handlers::{
    saving,
    building
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let version = env::var("CARGO_PKG_VERSION").unwrap();
    let author = env::var("CARGO_PKG_AUTHORS").unwrap();
    let data_url_bbta3 = env::var("FETCH_URL_BBTA3").expect("variabel FETCH_URL_BBTA3 belum didefinisikan");
    let data_url_bppt = env::var("FETCH_URL_BPPT").expect("variabel FETCH_URL_BBTA3 belum didefinisikan");
    let data_url_video = env::var("FETCH_URL_VIDEO").expect("variabel FETCH_URL_VIDEO belum didefinisikan");

    let url_bbta3 = data_url_bbta3.parse::<Uri>()?;
    let url_bppt = data_url_bppt.parse::<Uri>()?;
    let url_video = data_url_video.parse::<Uri>()?;

    let app = App::new("Web Worker CLI BBTA3 BPPT")
        .version(version.as_str()).author(author.as_str())
        .about("CLI untuk fetch data Twitter dan Youtube API BBTA3 BPPT")
        .arg(Arg::with_name("path")
            .short("p")
            .long("path")
            .help("Tentukan lokasi beserta nama file json yang ingin disimpan. Misal, /var/www/media.json")
            .required(true)
            .takes_value(true));

    let matches = app.get_matches();

    let stream = stream::iter(vec![url_bbta3, url_bppt, url_video])
        .map(building::response)
        .then(building::process);

    let vector_berita = stream.collect::<Vec<Vec<Berita>>>().await;

    let path = matches.value_of("path").unwrap();

    saving::save_berita(vector_berita, path);

    Ok(())
}
