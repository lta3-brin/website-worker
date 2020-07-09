use hyper::{Client, Uri};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "http://jsonplaceholder.typicode.com/users".parse::<Uri>()?;
    let client = Client::new();
    let resp = client.get(url).await?;

    println!("Response: {:?}", resp.body());

    Ok(())
}
