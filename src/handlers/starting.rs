use std::env;
use dotenv::dotenv;

use crate::models::error::AppErrors;
use crate::helpers::process::process_collection;

pub async fn app() -> Result<(), AppErrors> {
    dotenv().ok();

    let bbta3_url = env::var("FETCH_BBTA3_URL")?;
    let bppt_url = env::var("FETCH_BPPT_URL")?;
    let video_url = env::var("FETCH_VIDEO_URL")?;

    let koleksi = vec![bbta3_url, bppt_url, video_url];

    process_collection(koleksi).await
}
