use crate::models::error::AppErrors;
use crate::models::berita::Berita;


pub fn simple_error(kategori: &str, pesan: Option<&str>) -> Result<Vec<Berita>, AppErrors> {
    if let Some(psn) = pesan {
        Err(AppErrors::FetchContentErr(
            format!("{} Error: {}", kategori, psn.to_string())
        ))
    } else {
        let berita = Vec::<Berita>::new();

        Ok(berita)
    }
}
