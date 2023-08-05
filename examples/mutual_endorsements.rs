use crustacean_states::shards::nation::PublicNationRequest;
use crustacean_states::{
    client::{Client, ClientError},
    parsers::nation::Nation,
    shards::nation::PublicNationShard::Endorsements,
};
use std::error::Error;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let (client, mut client_state) = Client::new(user_agent)?.with_default_state();
    eprintln!("Made client!");
    let target = "Aramos";
    let request = PublicNationRequest::new(target, &[Endorsements]);
    eprintln!("{request:?}");
    let response = client.get(request, &mut client_state).await?;
    let text = response.text().await?;
    let target_nation = Nation::from_xml(&text)?;
    let endorsements = target_nation.endorsements.unwrap();
    eprintln!("{endorsements:?}");
    let l = endorsements.len();
    let mut n = 0;
    for endorsed_nation in endorsements {
        let request = PublicNationRequest::new(&endorsed_nation, &[Endorsements]);
        eprintln!("{request:?}");
        let response = match client.get(request.clone(), &mut client_state).await {
            Ok(r) => Ok(r),
            Err(ClientError::RateLimitedError(t)) => {
                tokio::time::sleep_until(Instant::from(t)).await;
                client.get(request, &mut client_state).await
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
