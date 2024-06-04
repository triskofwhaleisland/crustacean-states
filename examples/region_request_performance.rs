use crustacean_states::{client::Client, shards::region::RegionRequest};
use dotenvy::dotenv;
use std::error::Error;
use tokio::time::Instant;
use crustacean_states::parsers::region::Region;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();
    dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let client = Client::new(user_agent);
    eprintln!("Made client!");

    let target_region = "Anteria";
    let request = RegionRequest::new(target_region);
    let request_struct_made = Instant::now();
    let response = client.get(request);
    let response = response.await?;
    let api_responded = Instant::now();
    let response = response.bytes().await?;
    let text_gathered = Instant::now();
    println!("{response:?}");
    println!("\n---\n");
    let region = Region::from_xml(&response)?;
    println!("{}", region.inner);
    let response_struct_made = Instant::now();

    eprintln!("Creation time: {:?}", request_struct_made - start);
    eprintln!("Request time: {:?}", api_responded - request_struct_made);
    eprintln!("Reading time: {:?}", text_gathered - api_responded);
    eprintln!("Deserializing time: {:?}", response_struct_made - text_gathered);
    eprintln!("Total time: {:?}", response_struct_made - start);

    Ok(())
}
