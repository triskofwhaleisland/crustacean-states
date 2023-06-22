use crustacean_states::parsers::nation::Nation;
use crustacean_states::rate_limiter::client_request;
use crustacean_states::shards::public_nation_shards::PublicNationShard::*;
use crustacean_states::shards::NSRequest;
use reqwest::Client;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let target_name = "Aramos";
    let request = NSRequest::new_nation(
        target_name,
        vec![
            Admirable,
            Admirables,
            Animal,
            AnimalTrait,
            Answered,
            Banner,
            Banners,
            Capital,
            Category,
            Census {
                scale: None,
                modes: None,
            },
            Crime,
            Currency,
            DbId,
            Deaths,
            Demonym,
            Demonym2,
            Demonym2Plural,
            Dispatches,
            DispatchList,
            Endorsements,
            Factbooks,
            FactbookList,
            FirstLogin,
            Flag,
            Founded,
            FoundedTime,
            Freedom,
            FullName,
            GAVote,
            Gdp,
            Govt,
            GovtDesc,
            GovtPriority,
            Happenings,
            Income,
            IndustryDesc,
            Influence,
            LastActivity,
            LastLogin,
            Leader,
            Legislation,
            MajorIndustry,
            Motto,
            Name,
            Notable,
            Notables,
            Policies,
            Poorest,
            Population,
            PublicSector,
            RCensus,
            Region,
            Religion,
            Richest,
            SCVote,
            Sectors,
            Sensibilities,
            Tax,
            TGCanRecruit { from: None },
            TGCanCampaign { from: None },
            Type,
            WA,
            WABadges,
            WCensus,
        ],
    )
    .into_request();
    eprintln!("{request}");
    let client = Client::new();
    let raw_response = client_request(&client, &request).await?.text().await?;
    let response = Nation::from_xml(raw_response.as_str())?;
    println!("{response:?}");

    Ok(())
}
