use crustacean_states::client::Client;
use crustacean_states::shards::region::RegionRequest;
use std::error::Error;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let begin1 = Instant::now();
    dotenv::dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let client = Client::new(user_agent);
    eprintln!("Made client!");

    let target_region = "Anteria";
    let request = RegionRequest::new(target_region);
    let end1 = Instant::now();
    let response = client.get(request);
    let response = response.await?;
    let begin2 = Instant::now();
    let response = response.text().await?;
    let end2 = Instant::now();
    println!("{response}");

    eprintln!("Creation time: {:?}", end1 - begin1);
    eprintln!("Request time: {:?}", begin2 - end1);
    eprintln!("Reading time: {:?}", end2 - begin2);
    eprintln!("Total time: {:?}", (end1 - begin1) + (end2 - begin2));

    Ok(())
}
