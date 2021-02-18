mod models;
mod handlers;
mod helpers;

use crate::models::error::AppErrors;
use crate::handlers::starting;


#[tokio::main]
async fn main() -> Result<(), AppErrors> {
    starting::app().await
}
