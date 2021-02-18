use std::fs::File;

use crate::models::berita::Berita;
use crate::models::error::AppErrors;

pub fn save_content(konten: Vec<Berita>, path: &str) -> Result<(), AppErrors> {
    let mut koleksi = Vec::<Berita>::new();

    for item in konten {
        koleksi.push(item)
    }

    let writer = File::create(path)?;

    serde_json::to_writer_pretty(writer, &koleksi)?;

    Ok(())
}
