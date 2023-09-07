use crustacean_states::shards::nation::PublicNationRequest;
use crustacean_states::shards::nation::PublicNationShard::*;
use crustacean_states::shards::{CensusCurrentMode::*, CensusModes, CensusScales, CensusShard};
use crustacean_states::{client::Client, parsers::nation::Nation};
use std::error::Error;
use tokio::time::Instant;
// use std::fs::{File, OpenOptions};
// use std::io::{Read, Write};
// use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let begin1 = Instant::now();
    dotenv::dotenv()?;
    let user_agent = std::env::var("USER_AGENT")?;
    let client = Client::new(user_agent);
    let target_name = "Aramos";
    let shards = [
        Admirable,
        Admirables,
        Animal,
        AnimalTrait,
        Answered,
        Banner,
        Banners,
        Capital,
        Category,
        Census(CensusShard::new(
            CensusScales::All,
            CensusModes::from([Score, Rank, PercentRank, RegionRank, PercentRegionRank]),
        )),
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

    let request = PublicNationRequest::new_with_shards(target_name, shards);
    let end1 = Instant::now();
    let raw_response = client.get(request).await?.text().await?;
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
    let begin2 = Instant::now();
    let response = Nation::from_xml(&raw_response)?;
    let end2 = Instant::now();
    println!("{response:#?}");

    eprintln!("Creation time: {:?}", end1 - begin1);
    eprintln!("Request time: {:?}", begin2 - end1);
    eprintln!("Parsing time: {:?}", end2 - begin2);
    eprintln!("Total time: {:?}", end2 - begin1);

    Ok(())
}
