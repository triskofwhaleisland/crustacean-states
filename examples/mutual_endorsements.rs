use crustacean_states::shards::NSRequest;
use crustacean_states::{
    parsers::nation::Nation,
    request::{client_request, RateLimits},
    shards::public_nation::PublicNationShard::Endorsements,
};
use std::error::Error;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let client = reqwest::ClientBuilder::new()
        .user_agent(user_agent)
        .build()?;
    eprintln!("Made client!");
    let target = "Aramos";
    let request = NSRequest::new_nation(target.to_string(), vec![Endorsements]).into_request();
    eprintln!("{request}");
    let response = client_request(&client, &request).await?;
    let text = response.text().await?;
    let target_nation = Nation::from_xml(&text)?;
    let l = target_nation.endorsements.as_ref().unwrap().len();
    let mut n = 0;
    for endorsed_nation in target_nation.endorsements.unwrap() {
        let request = NSRequest::new_nation(endorsed_nation, vec![Endorsements]).into_request();
        eprintln!("{request}");
        let mut response = client_request(&client, &request).await?;
        if response.status().is_client_error() {
            let rate_limiter = RateLimits::new(response.headers())?;
            let wait_time = rate_limiter.retry_after.unwrap();
            eprintln!("Waiting for {} seconds to comply with API.", wait_time);
            tokio::time::sleep(Duration::from_secs(wait_time as u64)).await;
            response = client_request(&client, &request).await?;
        }
        let text = response.text().await?;
        let nation = Nation::from_xml(&text)?;
        if nation.endorsements.unwrap().contains(&target.to_string()) {
            n += 1;
            continue;
        }
    }
    println!(
        "{} is endorsing {} of the {} nations that endorse it.",
        target, n, l
    );

    Ok(())
}
