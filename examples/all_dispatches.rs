use crustacean_states::parsers::nation::Nation;
use crustacean_states::rate_limiter::client_request;
use crustacean_states::shards::public_nation_shards::PublicNationShard::DispatchList;
use crustacean_states::shards::NSRequest;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client = Client::new();
    eprintln!("Made client!");
    let target_nation = "Testlandia";
    let request = NSRequest::new_nation(target_nation, &[DispatchList]).to_string();
    eprintln!("{request}");
    let raw_response = client_request(&client, &request).await?;
    let text = raw_response.text().await?;
    let response: Nation = quick_xml::de::from_str(&text)?;
    eprintln!("{:#?}", response);

    Ok(())
}