use serde_json::Value;
use crate::models::error::AppErrors;
use crate::helpers::error;
use crate::models::berita::Berita;
use crate::helpers::process::{process_video, process_berita};

pub async fn konten(url: &str, token: &str) -> Result<Vec<Berita>, AppErrors> {
    let client = reqwest::Client::new();
    let resp = client
        .get(url)
        .header("Authorization", format!("bearer {}", token))
        .send()
        .await?
        .json::<Value>()
        .await?;

    if !resp["kind"].is_null() {
        process_video(resp)
    } else if !resp["error"].is_null() {
        let pesan = resp["error"]["message"].as_str();

        error::simple_error("Video", pesan)
    } else if !resp["errors"].is_null() {
        let pesan = resp["errors"][0]["message"].as_str();

        error::simple_error("News", pesan)
    } else {
        process_berita(resp)
    }
}
