use serde_json::{json, Value};

use crate::models::berita::Berita;
use crate::helpers::utility;

pub fn berita(items: &Vec<Value>) -> Vec<Berita> {
    let mut berita:Vec<Berita> = Vec::new();

    for item in items {
        let kabar = Berita {
            id: item["id_str"].to_owned(),
            deskripsi: item["full_text"].to_owned(),
            thumbnail: item["entities"]["media"][0]["media_url_https"].to_owned(),
            tanggal: json!(utility::build_date(item["created_at"].as_str().unwrap())),
            kategori: item["user"]["screen_name"].to_owned()
        };

        berita.push(kabar);
    }

    berita
}
