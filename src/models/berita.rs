use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Berita {
    pub deskripsi: String,
    pub thumbnail: String,
    pub tanggal: String,
    pub kategori: String
}
