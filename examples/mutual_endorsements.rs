use crustacean_states::shards::NSRequest;
use crustacean_states::{
    parsers::nation::Nation,
    rate_limiter::{client_request, RateLimits},
    shards::public_nation_shards::PublicNationShard::Endorsements,
};
use dotenv::dotenv;
use reqwest::Client;
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();
    let client = Client::new();
    eprintln!("Made client!");
    let nation = "Aramos";
    let request = NSRequest::new_nation_standard(nation.to_string()).to_string();
    let response = client_request(&client, &request).await?;
    let text = response.text().await?;
    let target = Nation::from_xml(&*text)?;
    let l = target.endorsements.clone().unwrap().len();
    let mut n = 0;
    for endorsed_nation in target.endorsements.unwrap() {
        let request = NSRequest::new_nation(endorsed_nation, &[Endorsements]).to_string();
        let mut response = client_request(&client, &request).await?;
        if response.status().is_client_error() {
            let rate_limiter = RateLimits::new(response.headers())?;
            let wait_time = rate_limiter.retry_after.unwrap();
            eprintln!("Waiting for {} seconds to comply with API.", wait_time);
            sleep(Duration::from_secs(wait_time as u64)).await;
            response = client_request(&client, &request).await?;
        }
        let text = response.text().await?;
        let nation = Nation::from_xml(&*text)?;
        if nation.endorsements.unwrap().contains(&"Aramos".to_string()) {
            n += 1;
            continue;
        }
    }
    eprintln!(
        "You are endorsing {} of the {} nations that endorse you.",
        n, l
    );

    Ok(())
}
