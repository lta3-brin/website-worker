use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct Berita {
    pub deskripsi: Value,
    pub thumbnail: Value,
    pub tanggal: Value,
    pub kategori: Value
}
