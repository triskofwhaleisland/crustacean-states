use crustacean_states::client::Client;
use crustacean_states::parsers::nation::Nation;
use crustacean_states::shards::nation::PublicNationRequest;
use crustacean_states::shards::nation::PublicNationShard::DispatchList;
use std::error::Error;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let begin1 = Instant::now();
    dotenv::dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let client = Client::new(user_agent);
    eprintln!("Made client!");

    let target_nation = "Testlandia";
    let request = PublicNationRequest::new(target_nation, &[DispatchList]);
    let end1 = Instant::now();
    // eprintln!("{request}");
    let text = client.get(request).await?.text().await?;
    let begin2 = Instant::now();
    // eprintln!("{text}");
    let response = Nation::from_xml(&text)?;
    let end2 = Instant::now();
    println!("{:#?}", response.dispatch_list);

    eprintln!("Creation time: {:?}", end1 - begin1);
    eprintln!("Request time: {:?}", begin2 - end1);
    eprintln!("Parsing time: {:?}", end2 - begin2);
    eprintln!("Total time: {:?}", end2 - begin1);

    Ok(())
}
