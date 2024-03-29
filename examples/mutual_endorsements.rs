use crustacean_states::{
    client::{Client, ClientError},
    parsers::nation::Nation,
    shards::nation::{PublicNationRequest, PublicNationShard::Endorsements},
};
use dotenvy::dotenv;
use std::error::Error;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let client = Client::new(user_agent);
    eprintln!("Made client!");
    let target = "Aramos";
    let request = PublicNationRequest::new_with_shards(target, [Endorsements]);
    eprintln!("{request:?}");
    let response = client.get(request).await?;
    let text = response.text().await?;
    let target_nation = Nation::from_xml(&text)?;
    let endorsements = target_nation.endorsements.unwrap();
    eprintln!("{endorsements:?}");
    let l = endorsements.len();
    let mut n = 0;
    for endorsed_nation in endorsements {
        let request = PublicNationRequest::new_with_shards(&endorsed_nation, vec![Endorsements]);
        eprintln!("{request:?}");
        let response = match client.get(request.clone()).await {
            Ok(r) => Ok(r),
            Err(ClientError::RateLimitedError(t)) => {
                tokio::time::sleep_until(Instant::from(t)).await;
                client.get(request).await
            }
            Err(e) => Err(e),
        }?;

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
