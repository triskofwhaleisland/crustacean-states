use crustacean_states::{
    client::{Client, ClientError},
    parsers::nation::Nation,
    shards::{public_nation::PublicNationShard::Endorsements, NSRequest},
};
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let mut client = Client::new(user_agent)?;
    eprintln!("Made client!");
    let target = "Aramos";
    let request = Url::from(NSRequest::new_nation(
        target.to_string(),
        vec![Endorsements],
    ));
    eprintln!("{request:?}");
    let response = client.get(request.as_str()).await?;
    let text = response.text().await?;
    let target_nation = Nation::from_xml(&text)?;
    let l = target_nation.endorsements.as_ref().unwrap().len();
    let mut n = 0;
    for endorsed_nation in target_nation.endorsements.unwrap() {
        let request = Url::from(NSRequest::new_nation(endorsed_nation, vec![Endorsements]));
        eprintln!("{request:?}");
        let response = match client.get(request.as_str()).await {
            Ok(r) => Ok(r),
            Err(ClientError::RateLimitedError(t)) => {
                tokio::time::sleep_until(t).await;
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
