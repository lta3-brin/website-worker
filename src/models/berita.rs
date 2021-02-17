use serde::Serialize;


#[derive(Debug)]
pub enum Konten {
    Video,
    Berita
}

#[derive(Debug, Serialize)]
pub struct Berita {
    id: String,
    deskripsi: String,
    thumbnail: String,
    tanggal: String,
    kategori: String
}

impl Berita {
    pub fn new(
        id: String,
        deskripsi: String,
        thumbnail: String,
        tanggal: String,
        kategori: String) -> Self {
        Self {
            id,
            deskripsi,
            thumbnail,
            tanggal,
            kategori
        }
    }
}
