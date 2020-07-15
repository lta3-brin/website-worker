use std::ops::Add;
use chrono::prelude::{DateTime, NaiveDateTime, FixedOffset};

fn replace_text(date: String) -> String {
    date.replace("January", "Januari")
        .replace("February", "Februari")
        .replace("March", "Maret")
        .replace("May", "Mei")
        .replace("June", "Juni")
        .replace("July", "Juli")
        .replace("August", "Agustus")
        .replace("October", "Oktober")
        .replace("December", "Desember")
        .replace("Monday", "Senin")
        .replace("Tuesday", "Selasa")
        .replace("Wednesday", "Rabu")
        .replace("Thursday", "Kamis")
        .replace("Friday", "Jum'at")
        .replace("Saturday", "Sabtu")
        .replace("Sunday", "Ahad")
}

pub fn build_date(tanggal: &str) -> String {
    let offset = FixedOffset::east(7 * 3600);
    let dt = DateTime::parse_from_str(tanggal, "%a %b %d %H:%M:%S %z %Y")
        .expect("Formatting date gagal dilakukan");

    let dt = dt.add(offset).format("%A, %d %B %Y").to_string();

    replace_text(dt)
}

pub fn build_date_yt(tanggal: &str) -> String {
    let dt = NaiveDateTime::parse_from_str(tanggal, "%Y-%m-%dT%H:%M:%SZ")
        .expect("Formatting date youtube gagal dilakukan");

    let dt = dt.format("%A, %d %B %Y").to_string();

    replace_text(dt)
}