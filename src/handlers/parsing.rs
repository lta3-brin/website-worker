use serde_json::Value;

use crate::models::berita::Berita;

pub fn berita(items: &Vec<Value>) -> Vec<Berita> {
    let mut berita:Vec<Berita> = Vec::new();

    for item in items {
        let kabar = Berita {
            deskripsi: item["full_text"].to_owned(),
            thumbnail: item["entities"]["media"][0]["media_url_https"].to_owned(),
            tanggal: item["created_at"].to_owned(),
            kategori: item["user"]["screen_name"].to_owned()
        };

        berita.push(kabar);
    }

    berita
}
