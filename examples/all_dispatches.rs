use crustacean_states::client::Client;
use crustacean_states::parsers::nation::Nation;
use crustacean_states::shards::nation::PublicNationRequest;
use crustacean_states::shards::nation::PublicNationShard::DispatchList;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let (client, mut client_state) = Client::new(user_agent)?.with_default_state();
    eprintln!("Made client!");

    let target_nation = "Testlandia";
    let request = PublicNationRequest::new(target_nation, &[DispatchList]);
    // eprintln!("{request}");
    let raw_response = client.get(request, &mut client_state).await?;
    let text = raw_response.text().await?;
    // eprintln!("{text}");
    let response = Nation::from_xml(&text)?;
    eprintln!("{:#?}", response.dispatch_list);

    Ok(())
}
