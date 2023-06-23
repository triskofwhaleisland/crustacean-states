use crustacean_states::parsers::nation::Nation;
use crustacean_states::request::client_request;
use crustacean_states::shards::public_nation::PublicNationShard::DispatchList;
use crustacean_states::shards::NSRequest;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let client = reqwest::ClientBuilder::new()
        .user_agent(user_agent)
        .build()?;
    eprintln!("Made client!");

    let target_nation = "Testlandia";
    let request = NSRequest::new_nation(target_nation, vec![DispatchList]).into_request();
    // eprintln!("{request}");
    let raw_response = client_request(&client, &request).await?;
    let text = raw_response.text().await?;
    // eprintln!("{text}");
    let response = Nation::from_xml(&text)?;
    eprintln!("{:#?}", response.dispatch_list);

    Ok(())
}
