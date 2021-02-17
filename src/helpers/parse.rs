use std::ops::Add;
use serde_json::Value;
use chrono::prelude::{DateTime, NaiveDateTime, FixedOffset};

use crate::models::berita::{Berita, Konten};
use crate::models::error::AppErrors;
use crate::helpers::text::replace_text;


fn parse_to_string(item: Option<&str>) -> String {
    if let Some(itm) = item {
        String::from(itm)
    } else {
        String::from("NULL")
    }
}

fn parse_tanggal_berita(tanggal: String) -> Result<String, AppErrors> {
    let offset = FixedOffset::east(7 * 3600);
    let dt = DateTime::parse_from_str(
        tanggal.as_str(),
        "%a %b %d %H:%M:%S %z %Y"
    )?.add(offset).format("%A, %d %B %Y").to_string();

    Ok(replace_text(dt))
}

fn parse_tanggal_video(tanggal: String) -> Result<String, AppErrors> {
    let dt = NaiveDateTime::parse_from_str(
        tanggal.as_str(),
        "%Y-%m-%dT%H:%M:%SZ"
    )?.format("%A, %d %B %Y").to_string();

    Ok(replace_text(dt))
}

pub fn parse_to_konten(jenis: Konten, satuan: &Value) -> Result<Berita, AppErrors> {
    match jenis {
        Konten::Berita => {
            Ok(Berita::new(
                parse_to_string(satuan["id_str"].as_str()),
                parse_to_string(satuan["full_text"].as_str()),
                parse_to_string(
                    satuan["entities"]["media"][0]["media_url_https"].as_str()
                ),
                parse_tanggal_berita(
                    parse_to_string(satuan["created_at"].as_str())
                )?,
                parse_to_string(satuan["user"]["screen_name"].as_str())
            ))
        }
        Konten::Video => {
            Ok(Berita::new(
                parse_to_string(satuan["id"]["videoId"].as_str()),
                parse_to_string(satuan["snippet"]["description"].as_str()),
                parse_to_string(
                    satuan["snippet"]["thumbnails"]["default"]["url"].as_str()
                ),
                parse_tanggal_video(
                    parse_to_string(
                        satuan["snippet"]["publishedAt"].as_str()
                    )
                )?,
                parse_to_string(Some("VIDEO"))
            ))
        }
    }
}
