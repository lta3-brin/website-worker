use std::env;
mod models;
mod handlers;
mod helpers;

use crate::models::error::AppErrors;
use crate::handlers::fetching::konten;


#[tokio::main]
async fn main() -> Result<(), AppErrors> {
    let token_berita = env::var("NEWS_API")?;
    // let bbta3_url = env::var("FETCH_BBTA3_URL")?;
    // let bppt_url = env::var("FETCH_BPPT_URL")?;
    let video_url = env::var("FETCH_VIDEO_URL")?;

    let data = konten(
        video_url.as_str(),
        token_berita.as_str()
    ).await?;

    println!("{:?}", data);

    Ok(())
}
