use crustacean_states::shards::nation::PublicNationRequest;
use crustacean_states::shards::nation::PublicNationShard::*;
use crustacean_states::shards::{CensusCurrentMode::*, CensusModes, CensusScales};
use crustacean_states::{client::Client, parsers::nation::Nation};
use std::error::Error;
// use std::fs::{File, OpenOptions};
// use std::io::{Read, Write};
// use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let (client, mut client_state) = Client::new(user_agent)?.with_default_state();
    let target_name = "Aramos";
    let shards = vec![
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
            scale: Some(CensusScales::All),
            modes: Some(CensusModes::Current(vec![
                Score,
                Rank,
                PercentRank,
                RegionRank,
                PercentRegionRank,
            ])),
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
        FreedomScores,
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
    ];

    let request = PublicNationRequest::new(target_name, &shards);
    let raw_response = client.get(request, &mut client_state).await?.text().await?;
    // if !Path::exists("response.xml".as_ref()) {
    //     File::create("response.xml")?;
    // }
    // OpenOptions::new()
    //     .write(true)
    //     .open("response.xml")?
    //     .write_all(&raw_response.into_bytes())?;
    // let mut contents: Vec<u8> = Vec::new();
    // File::open("response.xml")?.read_to_end(&mut contents)?;
    // let raw_response = std::str::from_utf8(&contents)?;
    let response = Nation::from_xml(&raw_response)?;
    println!("{response:#?}");

    Ok(())
}
