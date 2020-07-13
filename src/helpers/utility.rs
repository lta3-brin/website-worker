use chrono::prelude::NaiveDate;

pub fn build_date(tanggal: &str) -> String {
    let dt = NaiveDate::parse_from_str(tanggal, "%a %b %d %H:%M:%S %z %Y")
        .expect("Formatting date gagal dilakukan");

    let date = dt.format("%A, %d %B %Y").to_string();
    let date = date
        .replace("January", "Januari")
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
        .replace("Sunday", "Ahad");

    date
}