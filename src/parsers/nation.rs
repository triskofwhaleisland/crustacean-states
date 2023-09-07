//! The nation parser module.

use crate::{
    parsers::happenings::Event,
    parsers::{CensusData, DefaultOrCustom, Dispatch, MaybeRelativeTime, MaybeSystemTime},
    shards::wa::WACouncil,
};
use quick_xml::DeError;
use std::{
    fmt::{Debug, Display, Formatter},
    num::{NonZeroU16, NonZeroU32},
    str::FromStr,
};
use thiserror::Error;

/// The status of a nation in the World Assembly.
#[derive(Debug)]
pub enum WAStatus {
    /// The nation is delegate of a region.
    Delegate,
    /// The nation is simply a member.
    Member,
    /// The nation is not part of the World Assembly.
    NonMember,
}

/// Describes a nation's government spending as percentages.
/// Each field represents a category.
/// All fields *should* add up to 100.0,
/// but expect it to not be exact due to floating-point arithmetic and on-site rounding error.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct Government {
    pub administration: f64,
    pub defence: f64,
    pub education: f64,
    pub environment: f64,
    pub healthcare: f64,
    pub commerce: f64,
    pub international_aid: f64,
    pub law_and_order: f64,
    pub public_transport: f64,
    pub social_equality: f64,
    pub spirituality: f64,
    pub welfare: f64,
}

/// Describes national freedoms as explained on-site.
///
/// Note:
/// in a future release,
/// the fields in this struct will be converted from `String`s to enum variants.
#[derive(Debug)]
#[allow(missing_docs)]
pub struct Freedoms {
    // TODO make enum
    pub civil_rights: String,
    // TODO make enum
    pub economy: String,
    // TODO make enum
    pub political_freedom: String,
}

/// Gives a score out of 100 for the three types of national freedom.
// TODO restrict type from 0 to 100
#[derive(Debug)]
#[allow(missing_docs)]
pub struct FreedomScores {
    pub civil_rights: u8,
    pub economy: u8,
    pub political_freedom: u8,
}

/// Causes of death in a nation.
/// Note: at some point, the field `kind` in this struct will be converted to enum variants.
#[derive(Debug)]
pub struct Cause {
    /// The way in which citizens die.
    // TODO make enum
    pub kind: String,
    /// How common this cause of death is, to the nearest tenth of a percent.
    pub frequency: f64,
}

/// A breakdown of the relative economic power of each economic sector.
#[derive(Debug)]
#[allow(missing_docs)] // TODO learn economics so I can explain this :P
pub struct Sectors {
    pub black_market: f64,
    pub government: f64,
    pub industry: f64,
    pub public: f64,
}

/// A nation, with every piece of information you could ask for!
///
/// Note that aside from the `name` field, every field is an `Option`.
/// This is because,
/// depending on the [`PublicNationShard`]s used to make the request,
/// only certain fields will be returned.
#[derive(Debug)]
#[non_exhaustive]
pub struct Nation {
    /// The name of the nation.
    /// This is the only field that is guaranteed to be filled in.
    /// Note that because of limitations to the way the name is sent by NationStates,
    /// it may not be capitalized properly by the "pretty name" function.
    /// The only way to get the accurate capitalization is to request [`PublicNationShard::Name`].
    pub name: String,
    /// The pre-title of the nation.
    /// (`type` is a reserved word in Rust, so `kind` is used in its place.)
    ///
    /// Requested using
    /// [`PublicNationShard::Type`](crate::shards::nation::PublicNationShard::Type).
    pub kind: Option<String>,
    /// The full name of the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::FullName`](crate::shards::nation::PublicNationShard::FullName).
    pub full_name: Option<String>,
    /// The motto of the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Motto`](crate::shards::nation::PublicNationShard::Motto).
    pub motto: Option<String>,
    /// The category of the nation.
    /// Note that this is currently a `String` representation,
    /// but will eventually become its own type.
    ///
    /// Requested using
    /// [`PublicNationShard::Category`](crate::shards::nation::PublicNationShard::Category).
    pub category: Option<String>,
    /// The WA status of the nation.
    ///
    /// Requested using [`PublicNationShard::WA`](crate::shards::nation::PublicNationShard::WA).
    pub wa_status: Option<WAStatus>,
    /// A list of nations that endorse the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Endorsements`](crate::shards::nation::PublicNationShard::Endorsements).
    pub endorsements: Option<Vec<String>>,
    /// The number of issues answered by the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Answered`](crate::shards::nation::PublicNationShard::Answered).
    pub issues_answered: Option<u32>,
    /// The freedom statistics of the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Freedom`](crate::shards::nation::PublicNationShard::Freedom).
    pub freedom: Option<Freedoms>,
    /// The region that the nation resides in.
    ///
    /// Requested using
    /// [`PublicNationShard::Region`](crate::shards::nation::PublicNationShard::Region).
    pub region: Option<String>,
    /// The population of the nation in millions of people.
    ///
    /// Requested using
    /// [`PublicNationShard::Population`](crate::shards::nation::PublicNationShard::Population).
    pub population: Option<u32>,
    /// The effective tax rate of the nation.
    ///
    /// Requested using [`PublicNationShard::Tax`](crate::shards::nation::PublicNationShard::Tax).
    pub tax: Option<f64>,
    /// The national animal.
    ///
    /// Requested using
    /// [`PublicNationShard::Animal`](crate::shards::nation::PublicNationShard::Animal).
    pub animal: Option<String>,
    /// The national currency.
    ///
    /// Requested using
    /// [`PublicNationShard::Currency`](crate::shards::nation::PublicNationShard::Currency).
    pub currency: Option<String>,
    /// The adjective used to describe a citizen of the nation.
    /// (An example would be: I am **French**.)
    ///
    /// Requested using
    /// [`PublicNationShard::Demonym`](crate::shards::nation::PublicNationShard::Demonym).
    pub demonym_adjective: Option<String>,
    /// The singular noun used to describe a citizen of the nation.
    /// (An example would be: I am a **Frenchman**.)
    ///
    /// Requested using
    /// [`PublicNationShard::Demonym2`](crate::shards::nation::PublicNationShard::Demonym2).
    pub demonym_singular: Option<String>,
    /// The plural noun used to describe a citizen of the nation.
    /// (An example would be: They are (some) **Frenchmen**.)
    ///
    /// Requested using
    /// [`PublicNationShard::Demonym2Plural`](crate::shards::nation::PublicNationShard::Demonym2Plural).
    pub demonym_plural: Option<String>,
    /// The URL to the flag of the nation.
    ///
    /// Requested using [`PublicNationShard::Flag`](crate::shards::nation::PublicNationShard::Flag).
    pub flag: Option<String>,
    /// The largest industry in the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::MajorIndustry`](crate::shards::nation::PublicNationShard::MajorIndustry).
    pub major_industry: Option<String>,
    /// The financial sector where the government spends the most money.
    ///
    /// Requested using
    /// [`PublicNationShard::GovtPriority`](crate::shards::nation::PublicNationShard::GovtPriority).
    pub government_priority: Option<String>,
    /// The nation's government spending as percentages in various financial areas.
    ///
    /// Requested using [`PublicNationShard::Govt`](crate::shards::nation::PublicNationShard::Govt).
    pub government: Option<Government>,
    /// When the nation was founded as a relative timestamp.
    /// Note: NationStates did not track this at the beginning.
    /// For this reason, some nations are considered "founded in antiquity",
    /// which is represented by [`MaybeRelativeTime::Antiquity`]
    /// A nation founded more recently would be [`MaybeRelativeTime::Recorded`].
    ///
    /// Requested using
    /// [`PublicNationShard::Founded`](crate::shards::nation::PublicNationShard::Founded).
    pub founded: Option<MaybeRelativeTime>,
    /// The Unix timestamp of when the nation first logged in.
    ///
    /// Requested using
    /// [`PublicNationShard::FirstLogin`](crate::shards::nation::PublicNationShard::FirstLogin).
    pub first_login: Option<u64>,
    /// The Unix timestamp of when the nation most recently logged in.
    ///
    /// Requested using
    /// [`PublicNationShard::LastLogin`](crate::shards::nation::PublicNationShard::LastLogin).
    pub last_login: Option<u64>,
    /// When the nation was last active as a relative timestamp.
    ///
    /// Requested using
    /// [`PublicNationShard::LastActivity`](crate::shards::nation::PublicNationShard::LastActivity).
    pub last_activity: Option<String>,
    /// The influence of the nation in its region using qualitative descriptors.
    /// Note that this is currently a `String` representation,
    /// but will shift to an enum in the future.
    ///
    /// Requested using
    /// [`PublicNationShard::Influence`](crate::shards::nation::PublicNationShard::Influence).
    pub influence: Option<String>,
    /// The economy, political freedoms, and civil rights within the country,
    /// described using a quantitative scale.
    ///
    /// Requested using
    /// [`PublicNationShard::FreedomScores`](crate::shards::nation::PublicNationShard::FreedomScores).
    pub freedom_scores: Option<FreedomScores>,
    /// The percentage of the economy controlled or funded by the government and the public.
    ///
    /// Requested using
    /// [`PublicNationShard::PublicSector`](crate::shards::nation::PublicNationShard::PublicSector).
    pub public_sector: Option<f64>,
    /// The national statistics on deaths.
    ///
    /// Requested using
    /// [`PublicNationShard::Deaths`](crate::shards::nation::PublicNationShard::Deaths).
    pub deaths: Option<Vec<Cause>>,
    /// The national leader.
    ///
    /// If there is a custom leader,
    /// the [`DefaultOrCustom::Custom`] variant is filled with the custom leader's name;
    /// if not, the [`DefaultOrCustom::Default`] variant is filled with the default leader name.
    ///
    /// Requested using
    /// [`PublicNationShard::Leader`](crate::shards::nation::PublicNationShard::Leader).
    pub leader: Option<DefaultOrCustom>,
    /// The national capital.
    ///
    /// If there is a custom capital,
    /// the [`DefaultOrCustom::Custom`] variant is filled with the custom capital name;
    /// if not, the [`DefaultOrCustom::Default`] variant is filled with the default capital name.
    ///
    /// Requested using
    /// [`PublicNationShard::Capital`](crate::shards::nation::PublicNationShard::Capital).
    pub capital: Option<DefaultOrCustom>,
    /// The national religion.
    ///
    /// If there is a custom religion,
    /// the [`DefaultOrCustom::Custom`] variant is filled with the custom religion;
    /// if not, the [`DefaultOrCustom::Default`] variant is filled with the default religion.
    ///
    /// Requested using
    /// [`PublicNationShard::Religion`](crate::shards::nation::PublicNationShard::Religion).
    pub religion: Option<DefaultOrCustom>,
    /// The number of factbooks the nation has published.
    ///
    /// Requested using
    /// [`PublicNationShard::Factbooks`](crate::shards::nation::PublicNationShard::Factbooks).
    pub factbooks: Option<u16>,
    /// The number of dispatches the nation has published.
    ///
    /// Requested using
    /// [`PublicNationShard::Dispatches`](crate::shards::nation::PublicNationShard::Dispatches).
    pub dispatches: Option<u16>,
    /// The ID of the nation in the NationStates database.
    /// Note that earlier nations update first.
    ///
    /// Requested using [`PublicNationShard::DbId`](crate::shards::nation::PublicNationShard::DbId).
    pub dbid: Option<u32>,
    // END default
    /// A randomly-selected compliment for the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Admirable`](crate::shards::nation::PublicNationShard::Admirable).
    pub admirable: Option<String>,
    /// All possible compliments for the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Admirables`](crate::shards::nation::PublicNationShard::Admirables).
    pub admirables: Option<Vec<String>>,
    /// Describes the national animal on the nation's page.
    ///
    /// Requested using
    /// [`PublicNationShard::AnimalTrait`](crate::shards::nation::PublicNationShard::AnimalTrait).
    pub animal_trait: Option<String>,
    /// One Rift banner code that should be displayed for this nation:
    /// the nation's primary banner, if one is set; otherwise, a randomly chosen eligible banner.
    ///
    /// Requested using
    /// [`PublicNationShard::Banner`](crate::shards::nation::PublicNationShard::Banner).
    pub banner: Option<BannerId>,
    /// A list of Rift banners that should be displayed:
    /// the nation's primary banner (if any) is always listed first,
    /// with the remainder in random order.
    /// Banner codes can be converted into image URLs by prepending `/images/banners/`
    /// and appending `.jpg`.
    ///
    /// Requested using
    /// [`PublicNationShard::Banners`](crate::shards::nation::PublicNationShard::Banners).
    pub banners: Option<Vec<BannerId>>,
    /// Information on the nation's score and ranking on the World Census.
    /// If current data was requested (the default),
    /// the resulting data will be found in the [`CensusData::Current`] variant,
    /// but if historical data was requested,
    /// the resulting data wil be found in the [`CensusData::Historical`] variant.
    ///
    /// Requested and configured
    /// using [`PublicNationShard::Census`](crate::shards::nation::PublicNationShard::Census).
    pub census: Option<CensusData>,
    /// Describes crime in the nation on its nation page.
    ///
    /// Requested using
    /// [`PublicNationShard::Crime`](crate::shards::nation::PublicNationShard::Crime).
    pub crime: Option<String>,
    /// The list of all dispatches published by this nation.
    ///
    /// Requested using
    /// [`PublicNationShard::DispatchList`](crate::shards::nation::PublicNationShard::DispatchList).
    pub dispatch_list: Option<Vec<Dispatch>>,
    /// The list of all factbooks published by this nation.
    /// Note that because factbooks are a subset of dispatches,
    /// this field will contain a list of dispatches,
    /// but those dispatches will always be factbooks.
    ///
    /// Requested using
    /// [`PublicNationShard::FactbookList`](crate::shards::nation::PublicNationShard::FactbookList).
    pub factbook_list: Option<Vec<Dispatch>>,
    /// The Unix timestamp of when the nation was founded.
    /// Note: NationStates did not track this at the beginning.
    /// For this reason, some nations are considered "founded in antiquity",
    /// which is represented by [`MaybeSystemTime::Antiquity`].
    /// A nation founded more recently would be [`MaybeSystemTime::Recorded`].
    ///
    /// Requested using
    /// [`PublicNationShard::FoundedTime`](crate::shards::nation::PublicNationShard::FoundedTime).
    pub founded_time: Option<MaybeSystemTime>,
    /// The vote of the nation in the General Assembly.
    ///
    /// Note:
    /// if the nation is not in the World Assembly,
    /// but the [`PublicNationShard::WA`] shard was not requested,
    /// the field will erroneously be `Some(`[`WAVote::Undecided`]`)`.
    ///
    /// Requested using
    /// [`PublicNationShard::GAVote`](crate::shards::nation::PublicNationShard::GAVote).
    /// Recommended to request with [`PublicNationShard::WA`].
    ///
    /// [`PublicNationShard::WA`]: crate::shards::nation::PublicNationShard::WA
    pub ga_vote: Option<WAVote>,
    /// The GDP of the nation in its national currency.
    ///
    /// Requested using [`PublicNationShard::Gdp`](crate::shards::nation::PublicNationShard::Gdp).
    pub gdp: Option<u64>,
    /// The description of the nation's government found on its nation page.
    ///
    /// Requested using
    /// [`PublicNationShard::GovtDesc`](crate::shards::nation::PublicNationShard::GovtDesc).
    pub govt_desc: Option<String>,
    /// The 10 most recent [`Event`]s in the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Happenings`](crate::shards::nation::PublicNationShard::Happenings).
    pub happenings: Option<Vec<Event>>,
    /// The average income in the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Income`](crate::shards::nation::PublicNationShard::Income).
    pub income: Option<u32>,
    /// The description of the nation's industry found on its nation page.
    ///
    /// Requested using
    /// [`PublicNationShard::IndustryDesc`](crate::shards::nation::PublicNationShard::IndustryDesc).
    pub industry_desc: Option<String>,
    /// The list of (joke) descriptions of laws in the nation found on its nation page.
    ///
    /// Requested using
    /// [`PublicNationShard::Legislation`](crate::shards::nation::PublicNationShard::Legislation).
    pub legislation: Option<Vec<String>>,
    /// Notable facts about the nation, randomly selected by the API.
    ///
    /// Requested using
    /// [`PublicNationShard::Notable`](crate::shards::nation::PublicNationShard::Notable).
    pub notable: Option<String>,
    /// All possible notable facts about the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Notables`](crate::shards::nation::PublicNationShard::Notables).
    pub notables: Option<Vec<String>>,
    /// The list of policies the nation has in place.
    ///
    /// Requested using
    /// [`PublicNationShard::Policies`](crate::shards::nation::PublicNationShard::Policies).
    pub policies: Option<Vec<Policy>>,
    /// The average income of the poorest 10% in the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Poorest`](crate::shards::nation::PublicNationShard::Poorest).
    pub poorest: Option<u32>,
    /// The region rank on today's featured World Census scale.
    ///
    /// Requested using
    /// [`PublicNationShard::RCensus`](crate::shards::nation::PublicNationShard::RCensus).
    pub regional_census: Option<NonZeroU16>,
    /// The average income of the richest 10% in the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Richest`](crate::shards::nation::PublicNationShard::Richest).
    pub richest: Option<u32>,
    /// The vote of the nation in the Security Council.
    ///
    /// Note:
    /// if the nation is not in the World Assembly,
    /// and the [`PublicNationShard::WA`] shard was not requested,
    /// the field will erroneously be `Some(`[`WAVote::Undecided`]`)`.
    ///
    /// Requested using
    /// [`PublicNationShard::SCVote`](crate::shards::nation::PublicNationShard::SCVote).
    /// Recommended to request with [`PublicNationShard::WA`].
    ///
    /// [`PublicNationShard::WA`]: crate::shards::nation::PublicNationShard::WA
    pub sc_vote: Option<WAVote>,
    /// Describes the nation's economy as percentages controlled or funded by various sectors.
    ///
    /// Requested using
    /// [`PublicNationShard::Sectors`](crate::shards::nation::PublicNationShard::Sectors).
    pub sectors: Option<Sectors>,
    /// The adjectives that describe the nation's population on its nation page.
    ///
    /// Requested using
    /// [`PublicNationShard::Sensibilities`](crate::shards::nation::PublicNationShard::Sensibilities).
    pub sensibilities: Option<String>,
    /// Whether a recruitment telegram can be sent to the nation or not.
    ///
    /// Requested and configured using
    /// [`PublicNationShard::TGCanRecruit`](crate::shards::nation::PublicNationShard::TGCanRecruit).
    pub tg_can_recruit: Option<bool>,
    /// Whether a campaign telegram can be sent to the nation or not.
    ///
    /// Requested and configured using
    /// [`PublicNationShard::TGCanCampaign`](crate::shards::nation::PublicNationShard::TGCanCampaign).
    pub tg_can_campaign: Option<bool>,
    /// The world rank on today's featured World Census scale.
    ///
    /// Requested using
    /// [`PublicNationShard::WCensus`](crate::shards::nation::PublicNationShard::WCensus).
    pub world_census: Option<NonZeroU32>,
}

/// A nation given by the standard version of the public nation API.
///
/// This struct aims to have parity with [`Nation`],
/// but also has benefits for parsing this specific case:
/// fields are not wrapped in the [`Option`] type,
/// and only the fields required for the struct are provided.
/// This should speed up parsing and create ease of use.
#[derive(Debug)]
#[non_exhaustive]
pub struct StandardNation {
    /// The name of the nation.
    pub name: String,
    /// The pre-title of the nation.
    /// (`type` is a reserved word in Rust, so `kind` is used in its place.)
    pub kind: String,
    /// The full name of the nation.
    pub full_name: String,
    /// The motto of the nation.
    pub motto: String,
    /// The category of the nation.
    /// Note that this is currently a `String` representation,
    /// but will eventually become its own type.
    pub category: String,
    /// The WA status of the nation.
    pub wa_status: WAStatus,
    /// A list of nations that endorse the nation.
    pub endorsements: Vec<String>,
    /// The number of issues answered by the nation.
    pub issues_answered: u32,
    /// The freedom statistics of the nation.
    pub freedom: Freedoms,
    /// The region that the nation resides in.
    pub region: String,
    /// The population of the nation in millions of people.
    pub population: u32,
    /// The effective tax rate of the nation.
    pub tax: f64,
    /// The national animal.
    pub animal: String,
    /// The national currency.
    pub currency: String,
    /// The adjective used to describe a citizen of the nation.
    /// (An example would be: I am **French**.)
    pub demonym_adjective: String,
    /// The singular noun used to describe a citizen of the nation.
    /// (An example would be: I am a **Frenchman**.)
    pub demonym_singular: String,
    /// The plural noun used to describe a citizen of the nation.
    /// (An example would be: They are (some) **Frenchmen**.)
    pub demonym_plural: String,
    /// The URL to the flag of the nation.
    pub flag: String,
    /// The largest industry in the nation.
    pub major_industry: String,
    /// The financial sector where the government spends the most money.
    pub government_priority: String,
    /// The nation's government spending as percentages in various financial areas.
    pub government: Government,
    /// When the nation was founded as a relative timestamp.
    ///
    /// Note: NationStates did not track this at the beginning.
    /// For this reason, some nations are considered "founded in antiquity",
    /// which is represented by [`MaybeRelativeTime::Antiquity`]
    /// A nation founded more recently would be [`MaybeRelativeTime::Recorded`].
    pub founded: MaybeRelativeTime,
    /// The Unix timestamp of when the nation first logged in.
    pub first_login: u64,
    /// The Unix timestamp of when the nation most recently logged in.
    pub last_login: u64,
    /// When the nation was last active as a relative timestamp.
    pub last_activity: String,
    /// The influence of the nation in its region using qualitative descriptors.
    /// Note that this is currently a `String` representation,
    /// but will shift to an enum in the future.
    pub influence: String,
    /// The economy, political freedoms, and civil rights within the country,
    /// described using a quantitative scale.
    pub freedom_scores: FreedomScores,
    /// The percentage of the economy controlled or funded by the government and the public.
    pub public_sector: f64,
    /// The national statistics on deaths.
    pub deaths: Vec<Cause>,
    /// The national leader.
    ///
    /// If there is a custom leader,
    /// the [`DefaultOrCustom::Custom`] variant is filled with the custom leader's name;
    /// if not, the [`DefaultOrCustom::Default`] variant is filled with the default leader name.
    pub leader: DefaultOrCustom,
    /// The national capital.
    ///
    /// If there is a custom capital,
    /// the [`DefaultOrCustom::Custom`] variant is filled with the custom capital name;
    /// if not, the [`DefaultOrCustom::Default`] variant is filled with the default capital name.
    pub capital: DefaultOrCustom,
    /// The national religion.
    ///
    /// If there is a custom religion,
    /// the [`DefaultOrCustom::Custom`] variant is filled with the custom religion;
    /// if not, the [`DefaultOrCustom::Default`] variant is filled with the default religion.
    pub religion: DefaultOrCustom,
    /// The number of factbooks the nation has published.
    pub factbooks: u16,
    /// The number of dispatches the nation has published.
    pub dispatches: u16,
    /// The ID of the nation in the NationStates database.
    /// Note that earlier nations update first.
    pub dbid: u32,
}

/// Describes a national policy.
#[derive(Debug)]
pub struct Policy {
    /// The name of the policy.
    pub name: String,
    /// The banner that is associated with the policy.
    pub picture: BannerId,
    /// The category the policy belongs to. Note: this field will eventually be converted into an `enum`.
    // TODO PolicyCategory
    pub category: String,
    /// The description of the policy.
    pub description: String,
}

/// Represents any one of the errors
/// that can go wrong between deserialization and creating the Nation struct.
#[derive(Debug, Error)]
pub enum IntoNationError {
    /// A string could not be parsed as a banner ID.
    #[error("malformed banner id: {0}")]
    BadBannerId(String),
    /// A `u8` could not be parsed as a `bool` because it was not `0` or `1`.
    #[error("boolean cannot be derived from {0}")]
    BadBooleanError(u8),
    /// A `String` could not be parsed as a [`DispatchCategory`][crate::dispatch::DispatchCategory].
    #[error("malformed dispatch category: {0}")]
    BadDispatchCategory(String),
    /// A `String` could not be parsed as a [`WAStatus`].
    #[error("malformed WA status response: {0}")]
    BadWAStatusError(String),
    /// A `String` could not be parsed as a [`WAVote`].
    #[error("malformed WA vote: {bad_vote} in {council:?}")]
    BadWAVote {
        /// The problematic content.
        bad_vote: String,
        /// The council that the vote was supposedly for.
        council: WACouncil,
    },
    /// Something bad happened in deserialization.
    #[error("deserialization failed")]
    DeserializationError {
        /// The error source. Look here for what went wrong.
        #[from]
        source: DeError,
    },
    /// There was neither an `id` attribute in the `<NATION>` root tag nor a `<NAME>` tag.
    #[error("could not find a nation name in response")]
    NoNameError,
    /// No census data was created for this nation.
    #[error("could not find any census data in response")]
    NoCensusDataError,
}

/// Describes a nation's vote in the World Assembly.
#[derive(Debug)]
pub enum WAVote {
    /// The nation votes for the proposed resolution.
    For,
    /// The nation votes against the proposed resolution.
    Against,
    /// The nation has not voted on the proposed resolution.
    ///
    /// This is the default response that the game provides,
    /// even if the nation is not in the World Assembly.
    /// See the documentation for [`PublicNationShard::GAVote`] or [`PublicNationShard::SCVote`]
    /// for more details.
    Undecided,
}

impl TryFrom<String> for WAVote {
    type Error = IntoNationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "FOR" => Ok(WAVote::For),
            "AGAINST" => Ok(WAVote::Against),
            "UNDECIDED" => Ok(WAVote::Undecided),
            other => Err(IntoNationError::BadWAVote {
                bad_vote: other.to_string(),
                council: Default::default(),
            }),
        }
    }
}

/// The ID of a banner. WIP. TODO make banner id categories
#[derive(Clone, Debug)]
pub struct BannerId {
    pub(crate) category: String,
    pub(crate) number: u16,
}

impl Display for BannerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.category.to_ascii_lowercase(), self.number)
    }
}

impl BannerId {
    fn new(category: impl ToString, number: u16) -> Self {
        Self {
            category: category.to_string(),
            number,
        }
    }
}

impl TryFrom<String> for BannerId {
    type Error = IntoNationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split_index = value.chars().position(|c| c.is_ascii_digit());
        if split_index.is_none() || split_index == Some(0) {
            return Err(IntoNationError::BadBannerId(value));
        }
        let (cat, num) = value.split_at(split_index.unwrap());
        let num = u16::from_str(num).map_err(|_| IntoNationError::BadBannerId(value.clone()))?;
        Ok(BannerId::new(cat, num))
    }
}
