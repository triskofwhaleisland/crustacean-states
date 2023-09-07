use crustacean_states::{
    client::Client,
    parsers::nation::Nation,
    shards::{
        nation::{PublicNationRequest, PublicNationShard as PNS},
        CensusCurrentMode as CCM, CensusModes, CensusScales, CensusShard,
    },
};
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
        PNS::Admirable,
        PNS::Admirables,
        PNS::Animal,
        PNS::AnimalTrait,
        PNS::Answered,
        PNS::Banner,
        PNS::Banners,
        PNS::Capital,
        PNS::Category,
        PNS::Census(CensusShard::new(
            CensusScales::All,
            CensusModes::from([
                CCM::Score,
                CCM::Rank,
                CCM::PercentRank,
                CCM::RegionRank,
                CCM::PercentRegionRank,
            ]),
        )),
        PNS::Crime,
        PNS::Currency,
        PNS::DbId,
        PNS::Deaths,
        PNS::Demonym,
        PNS::Demonym2,
        PNS::Demonym2Plural,
        PNS::Dispatches,
        PNS::DispatchList,
        PNS::Endorsements,
        PNS::Factbooks,
        PNS::FactbookList,
        PNS::FirstLogin,
        PNS::Flag,
        PNS::Founded,
        PNS::FoundedTime,
        PNS::Freedom,
        PNS::FreedomScores,
        PNS::FullName,
        PNS::GAVote,
        PNS::Gdp,
        PNS::Govt,
        PNS::GovtDesc,
        PNS::GovtPriority,
        PNS::Happenings,
        PNS::Income,
        PNS::IndustryDesc,
        PNS::Influence,
        PNS::LastActivity,
        PNS::LastLogin,
        PNS::Leader,
        PNS::Legislation,
        PNS::MajorIndustry,
        PNS::Motto,
        PNS::Name,
        PNS::Notable,
        PNS::Notables,
        PNS::Policies,
        PNS::Poorest,
        PNS::Population,
        PNS::PublicSector,
        PNS::RCensus,
        PNS::Region,
        PNS::Religion,
        PNS::Richest,
        PNS::SCVote,
        PNS::Sectors,
        PNS::Sensibilities,
        PNS::Tax,
        PNS::TGCanRecruit { from: None },
        PNS::TGCanCampaign { from: None },
        PNS::Type,
        PNS::WA,
        PNS::WABadges,
        PNS::WCensus,
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
