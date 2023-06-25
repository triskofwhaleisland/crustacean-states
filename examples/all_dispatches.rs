use crustacean_states::client::Client;
use crustacean_states::parsers::nation::Nation;
use crustacean_states::shards::public_nation::PublicNationShard::DispatchList;
use crustacean_states::shards::NSRequest;
use std::error::Error;
use url::Url;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let mut client = Client::new(user_agent)?;
    eprintln!("Made client!");

    let target_nation = "Testlandia";
    let request = Url::from(NSRequest::new_nation(target_nation, vec![DispatchList]));
    // eprintln!("{request}");
    let raw_response = client.get(request).await?;
    let text = raw_response.text().await?;
    // eprintln!("{text}");
    let response = Nation::from_xml(&text)?;
    eprintln!("{:#?}", response.dispatch_list);

    Ok(())
}
