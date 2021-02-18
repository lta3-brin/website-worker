use std::env;
use serde_json::Value;

use crate::models::{
    error::AppErrors,
    berita::{Berita, Konten},
};
use crate::helpers::{
    save::save_content,
    parse::parse_to_konten,
};
use crate::handlers::fetching::konten;

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

pub async fn process_collection(collection: Vec<String>) -> Result<(), AppErrors> {
    let path = env::var("SAVE_PATH")?;
    let mut berita = Vec::<Vec<Berita>>::new();

    for satuan in collection {
        let token_berita = env::var("NEWS_API")?;
        let handle = tokio::spawn(async move {
            let data = konten(
                satuan.as_str(),
                &token_berita
            ).await;

            data
        });

        let result = handle.await??;
        berita.push(result);
    }

    let mut konten = Vec::<Berita>::new();
    for brt in berita {
        for b in brt {
            konten.push(b);
        }
    }

    Ok(save_content(konten, &path)?)
}
