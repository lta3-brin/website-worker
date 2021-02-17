use serde_json::Value;

use crate::models::berita::{Berita, Konten};
use crate::models::error::AppErrors;
use crate::helpers::parse::parse_to_konten;

pub fn process_video(response: Value) -> Result<Vec<Berita>, AppErrors> {
    if let Some(koleksi) = response.as_object() {
        let mut kl = Vec::<Berita>::new();

        if let Some(koleksi) = koleksi["items"].as_array() {
            for satuan in koleksi {
                kl.push(
                    parse_to_konten(Konten::Video, satuan)?
                );
            }

            Ok(kl)
        } else {
            Err(AppErrors::FetchContentErr(
                String::from("Video bukan dalam bentuk array.")
            ))
        }
    } else {
        Err(AppErrors::FetchContentErr(
            String::from("Video bukan dalam bentuk objek.")
        ))
    }
}

pub fn process_berita(response: Value) -> Result<Vec<Berita>, AppErrors> {
    if response.is_array() {
        if let Some(koleksi) = response.as_array() {
            let mut kl = Vec::<Berita>::new();

            for satuan in koleksi {
                kl.push(
                    parse_to_konten(Konten::Berita, satuan)?
                );
            }

            Ok(kl)
        } else {
            Err(AppErrors::FetchContentErr(
                String::from("Berita BBTA3 bukan dalam bentuk array.")
            ))
        }
    } else {
        if let Some(koleksi) = response.as_object() {
            let mut kl = Vec::<Berita>::new();

            if let Some(koleksi) = koleksi["statuses"].as_array() {
                for satuan in koleksi {
                    kl.push(
                        parse_to_konten(Konten::Berita, satuan)?
                    );
                }

                Ok(kl)
            } else {
                Err(AppErrors::FetchContentErr(
                    String::from("Berita BPPT bukan dalam bentuk array.")
                ))
            }
        } else {
            Err(AppErrors::FetchContentErr(
                String::from("Berita BPPT bukan dalam bentuk objek.")
            ))
        }
    }
}
