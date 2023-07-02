//! The nation parser module.

use crate::parsers::happenings::Event;
use crate::parsers::{
    CensusCurrentData, CensusData, CensusHistoricalData, DefaultOrCustom, Dispatch,
    MaybeRelativeTime, MaybeSystemTime, RawEvent,
};
use crate::pretty_name;
#[allow(unused_imports)] // needed for docs
use crate::shards::nation::PublicNationShard;
use crate::shards::wa::WACouncil;
use crate::shards::world::{
    AccountCategory, BannerId, BulletinCategory, DispatchCategory, FactbookCategory, MetaCategory,
};
#[allow(unused_imports)] // needed for docs
use crate::shards::NSRequest;
use quick_xml::DeError;
use serde::Deserialize;
use std::fmt::Debug;
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use thiserror::Error;

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
/// The Rust representation of a nation, as interpreted from a response to a request.
struct RawNation {
    // default shards
    #[serde(rename = "@id")]
    id: Option<String>,
    name: Option<String>,
    #[serde(rename = "TYPE")]
    kind: Option<String>,
    fullname: Option<String>,
    motto: Option<String>,
    category: Option<String>,
    unstatus: Option<String>,
    endorsements: Option<String>,
    issues_answered: Option<u32>,
    freedom: Option<Freedoms>,
    region: Option<String>,
    population: Option<u32>,
    tax: Option<f64>,
    animal: Option<String>,
    currency: Option<String>,
    demonym: Option<String>,
    demonym2: Option<String>,
    demonym2plural: Option<String>,
    flag: Option<String>,
    majorindustry: Option<String>,
    govtpriority: Option<String>,
    govt: Option<Government>,
    founded: Option<String>,
    firstlogin: Option<u64>,
    lastlogin: Option<u64>,
    lastactivity: Option<String>,
    influence: Option<String>,
    freedomscores: Option<FreedomScores>,
    publicsector: Option<f64>,
    deaths: Option<Deaths>,
    leader: Option<String>,
    capital: Option<String>,
    religion: Option<String>,
    factbooks: Option<u16>,
    dispatches: Option<u16>,
    dbid: Option<u32>,
    // END default
    admirable: Option<String>,
    admirables: Option<Admirables>,
    animaltrait: Option<String>,
    banner: Option<String>,
    banners: Option<Banners>,
    census: Option<Census>,
    crime: Option<String>,
    dispatchlist: Option<RawDispatchList>,
    factbooklist: Option<RawFactbookList>,
    foundedtime: Option<u64>,
    gavote: Option<String>,
    gdp: Option<u64>,
    govtdesc: Option<String>,
    happenings: Option<Happenings>,
    income: Option<u32>,
    industrydesc: Option<String>,
    legislation: Option<Legislation>,
    notable: Option<String>,
    notables: Option<Notables>,
    policies: Option<Policies>,
    poorest: Option<u32>,
    rcensus: Option<NonZeroU16>,
    richest: Option<u32>,
    scvote: Option<String>,
    sectors: Option<Sectors>,
    sensibilities: Option<String>,
    tgcanrecruit: Option<u8>,
    tgcancampaign: Option<u8>,
    wcensus: Option<NonZeroU32>,
}

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
//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Government {
    #[serde(rename = "ADMINISTRATION")]
    pub administration: f64,
    #[serde(rename = "DEFENCE")]
    pub defence: f64,
    #[serde(rename = "EDUCATION")]
    pub education: f64,
    #[serde(rename = "ENVIRONMENT")]
    pub environment: f64,
    #[serde(rename = "HEALTHCARE")]
    pub healthcare: f64,
    #[serde(rename = "COMMERCE")]
    pub commerce: f64,
    #[serde(rename = "INTERNATIONALAID")]
    pub international_aid: f64,
    #[serde(rename = "LAWANDORDER")]
    pub law_and_order: f64,
    #[serde(rename = "PUBLICTRANSPORT")]
    pub public_transport: f64,
    #[serde(rename = "SOCIALEQUALITY")]
    pub social_equality: f64,
    #[serde(rename = "SPIRITUALITY")]
    pub spirituality: f64,
    #[serde(rename = "WELFARE")]
    pub welfare: f64,
}

#[derive(Debug, Deserialize)]
struct Deaths {
    #[serde(rename = "CAUSE", default)]
    inner: Vec<Cause>,
}

#[derive(Debug, Deserialize)]
struct Admirables {
    #[serde(rename = "ADMIRABLE", default)]
    inner: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Banners {
    #[serde(rename = "BANNER", default)]
    inner: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Census {
    #[serde(rename = "SCALE", default)]
    inner: Vec<RawCensusData>,
}

#[derive(Debug, Deserialize)]
struct RawDispatchList {
    #[serde(rename = "DISPATCH", default)]
    inner: Vec<RawDispatch>,
}

#[derive(Debug, Deserialize)]
struct RawFactbookList {
    #[serde(rename = "FACTBOOK", default)]
    inner: Vec<RawDispatch>, // only containing factbooks!!
}

/// Describes national freedoms as explained on-site.
///
/// Note:
/// in a future release,
/// the fields in this struct will be converted from `String`s to enum variants.
//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct Freedoms {
    #[serde(rename = "CIVILRIGHTS")]
    pub civil_rights: String,
    #[serde(rename = "ECONOMY")]
    pub economy: String,
    #[serde(rename = "POLITICALFREEDOM")]
    pub political_freedom: String,
}

/// Gives a score out of 100 for the three types of national freedom.
//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[allow(missing_docs)]
pub struct FreedomScores {
    #[serde(rename = "CIVILRIGHTS")]
    pub civil_rights: u8,
    #[serde(rename = "ECONOMY")]
    pub economy: u8,
    #[serde(rename = "POLITICALFREEDOM")]
    pub political_freedom: u8,
}

/// Causes of death in a nation.
/// Note: at some point, the field `kind` in this struct will be converted to enum variants.
#[derive(Clone, Debug, Deserialize)]
pub struct Cause {
    /// The way in which citizens die.
    #[serde(rename = "@type")]
    pub kind: String,
    /// How common this cause of death is, to the nearest tenth of a percent.
    #[serde(rename = "$value")]
    pub frequency: f64,
}

//noinspection SpellCheckingInspection
#[derive(Clone, Debug, Deserialize)]
struct RawCensusData {
    #[serde(rename = "@id")]
    id: u8,
    #[serde(rename = "SCORE")]
    score: Option<f64>,
    #[serde(rename = "RANK")]
    world_rank: Option<NonZeroU32>,
    #[serde(rename = "RRANK")]
    region_rank: Option<NonZeroU32>,
    #[serde(rename = "PRANK")]
    percent_world_rank: Option<f64>,
    #[serde(rename = "PRRANK")]
    percent_region_rank: Option<f64>,
    #[serde(rename = "TIMESTAMP")]
    timestamp: Option<NonZeroU64>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawDispatch {
    #[serde(rename = "@id")]
    id: u32,
    title: String,
    author: String,
    category: String,
    subcategory: String,
    created: u64,
    edited: u64,
    views: u32,
    score: u32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct Happenings {
    #[serde(rename = "EVENT", default)]
    inner: Vec<RawEvent>,
}

#[derive(Debug, Deserialize)]
struct Legislation {
    #[serde(rename = "LAW", default)]
    inner: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Notables {
    #[serde(rename = "NOTABLE", default)]
    inner: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Policies {
    #[serde(rename = "POLICY", default)]
    inner: Vec<RawPolicy>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawPolicy {
    name: String,
    pic: String,
    cat: String,
    desc: String,
}

/// A breakdown of the relative economic power of each economic sector.
//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[allow(missing_docs)] // TODO learn economics so I can explain this
pub struct Sectors {
    #[serde(rename = "BLACKMARKET")]
    pub black_market: f64,
    pub government: f64,
    pub industry: f64,
    pub public: f64,
}

/// A nation, with every piece of information you could ask for!
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
    /// [`PublicNationShard::Type`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub kind: Option<String>,
    /// The full name of the nation.
    ///
    /// Requested using [`PublicNationShard::FullName`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub full_name: Option<String>,
    /// The motto of the nation.
    ///
    /// Requested using [`PublicNationShard::Motto`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub motto: Option<String>,
    /// The category of the nation.
    /// Note that this is currently a `String` representation,
    /// but will eventually become its own type.
    ///
    /// Requested using [`PublicNationShard::Category`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub category: Option<String>,
    /// The WA status of the nation.
    ///
    /// Requested using [`PublicNationShard::WA`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub wa_status: Option<WAStatus>,
    /// A list of nations that endorse the nation.
    ///
    /// Requested using [`PublicNationShard::Endorsements`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub endorsements: Option<Vec<String>>,
    /// The number of issues answered by the nation.
    ///
    /// Requested using [`PublicNationShard::Answered`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub issues_answered: Option<u32>,
    /// The freedom statistics of the nation.
    ///
    /// Requested using [`PublicNationShard::Freedom`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub freedom: Option<Freedoms>,
    /// The region that the nation resides in.
    ///
    /// Requested using [`PublicNationShard::Region`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub region: Option<String>,
    /// The population of the nation in millions of people.
    ///
    /// Requested using [`PublicNationShard::Population`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub population: Option<u32>,
    /// The effective tax rate of the nation.
    ///
    /// Requested using [`PublicNationShard::Tax`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub tax: Option<f64>,
    /// The national animal.
    ///
    /// Requested using [`PublicNationShard::Animal`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub animal: Option<String>,
    /// The national currency.
    ///
    /// Requested using [`PublicNationShard::Currency`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub currency: Option<String>,
    /// The adjective used to describe a citizen of the nation.
    /// (An example would be: I am **French**.)
    ///
    /// Requested using [`PublicNationShard::Demonym`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub demonym_adjective: Option<String>,
    /// The singular noun used to describe a citizen of the nation.
    /// (An example would be: I am a **Frenchman**.)
    ///
    /// Requested using [`PublicNationShard::Demonym2`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub demonym_singular: Option<String>,
    /// The plural noun used to describe a citizen of the nation.
    /// (An example would be: They are (some) **Frenchmen**.)
    ///
    /// Requested using [`PublicNationShard::Demonym2Plural`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub demonym_plural: Option<String>,
    /// The URL to the flag of the nation.
    ///
    /// Requested using [`PublicNationShard::Flag`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub flag: Option<String>,
    /// The largest industry in the nation.
    ///
    /// Requested using [`PublicNationShard::MajorIndustry`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub major_industry: Option<String>,
    /// The financial sector where the government spends the most money.
    ///
    /// Requested using [`PublicNationShard::GovtPriority`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub government_priority: Option<String>,
    /// The nation's government spending as percentages in various financial areas.
    ///
    /// Requested using [`PublicNationShard::Govt`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub government: Option<Government>,
    /// When the nation was founded as a relative timestamp.
    /// Note: NationStates did not track this at the beginning.
    /// For this reason, some nations are considered "founded in antiquity",
    /// which is represented by [`MaybeRelativeTime::Antiquity`]
    /// A nation founded more recently would be [`MaybeRelativeTime::Recorded`].
    ///
    /// Requested using [`PublicNationShard::Founded`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub founded: Option<MaybeRelativeTime>,
    /// The Unix timestamp of when the nation first logged in.
    ///
    /// Requested using [`PublicNationShard::FirstLogin`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub first_login: Option<u64>,
    /// The Unix timestamp of when the nation most recently logged in.
    ///
    /// Requested using [`PublicNationShard::LastLogin`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub last_login: Option<u64>,
    /// When the nation was last active as a relative timestamp.
    ///
    /// Requested using [`PublicNationShard::LastActivity`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub last_activity: Option<String>,
    /// The influence of the nation in its region using qualitative descriptors.
    /// Note that this is currently a `String` representation,
    /// but will shift to an enum in the future.
    ///
    /// Requested using [`PublicNationShard::Influence`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub influence: Option<String>,
    /// The economy, political freedoms, and civil rights within the country,
    /// described using a quantitative scale.
    ///
    /// Requested using [`PublicNationShard::FreedomScores`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub freedom_scores: Option<FreedomScores>,
    /// The percentage of the economy controlled or funded by the government and the public.
    ///
    /// Requested using [`PublicNationShard::PublicSector`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub public_sector: Option<f64>,
    /// The national statistics on deaths.
    ///
    /// Requested using [`PublicNationShard::Deaths`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub deaths: Option<Vec<Cause>>,
    /// The national leader.
    ///
    /// If there is a custom leader,
    /// the [`DefaultOrCustom::Custom`] variant is filled with the custom leader's name;
    /// if not, the [`DefaultOrCustom::Default`] variant is filled with the default leader name.
    ///
    /// Requested using [`PublicNationShard::Leader`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub leader: Option<DefaultOrCustom>,
    /// The national capital.
    ///
    /// If there is a custom capital,
    /// the [`DefaultOrCustom::Custom`] variant is filled with the custom capital name;
    /// if not, the [`DefaultOrCustom::Default`] variant is filled with the default capital name.
    ///
    /// Requested using [`PublicNationShard::Capital`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub capital: Option<DefaultOrCustom>,
    /// The national religion.
    ///
    /// If there is a custom religion,
    /// the [`DefaultOrCustom::Custom`] variant is filled with the custom religion;
    /// if not, the [`DefaultOrCustom::Default`] variant is filled with the default religion.
    ///
    /// Requested using [`PublicNationShard::Religion`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub religion: Option<DefaultOrCustom>,
    /// The number of factbooks the nation has published.
    ///
    /// Requested using [`PublicNationShard::Factbooks`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub factbooks: Option<u16>,
    /// The number of dispatches the nation has published.
    ///
    /// Requested using [`PublicNationShard::Dispatches`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub dispatches: Option<u16>,
    /// The ID of the nation in the NationStates database.
    /// Note that earlier nations update first.
    ///
    /// Requested using [`PublicNationShard::DbId`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub dbid: Option<u32>,
    // END default
    /// A randomly-selected compliment for the nation.
    ///
    /// Requested using [`PublicNationShard::Admirable`].
    pub admirable: Option<String>,
    /// All possible compliments for the nation.
    ///
    /// Requested using [`PublicNationShard::Admirables`].
    pub admirables: Option<Vec<String>>,
    /// Describes the national animal on the nation's page.
    ///
    /// Requested using [`PublicNationShard::AnimalTrait`].
    pub animal_trait: Option<String>,
    /// One Rift banner code that should be displayed for this nation:
    /// the nation's primary banner, if one is set; otherwise, a randomly chosen eligible banner.
    ///
    /// Requested using [`PublicNationShard::Banner`].
    pub banner: Option<BannerId>,
    /// A list of Rift banners that should be displayed:
    /// the nation's primary banner (if any) is always listed first,
    /// with the remainder in random order.
    /// Banner codes can be converted into image URLs by prepending `/images/banners/`
    /// and appending `.jpg`.
    ///
    /// Requested using [`PublicNationShard::Banners`].
    pub banners: Option<Vec<BannerId>>,
    /// Information on the nation's score and ranking on the World Census.
    /// If current data was requested (the default),
    /// the resulting data will be found in the [`CensusData::Current`] variant,
    /// but if historical data was requested,
    /// the resulting data wil be found in the [`CensusData::Historical`] variant.
    ///
    /// Requested and configured using [`PublicNationShard::Census`].
    pub census: Option<CensusData>,
    /// Describes crime in the nation on its nation page.
    ///
    /// Requested using [`PublicNationShard::Crime`].
    pub crime: Option<String>,
    /// The list of all dispatches published by this nation.
    ///
    /// Requested using [`PublicNationShard::DispatchList`].
    pub dispatch_list: Option<Vec<Dispatch>>,
    /// The list of all factbooks published by this nation.
    /// Note that because factbooks are a subset of dispatches,
    /// this field will contain a list of dispatches,
    /// but those dispatches will always be factbooks.
    ///
    /// Requested using [`PublicNationShard::FactbookList`].
    pub factbook_list: Option<Vec<Dispatch>>,
    /// The Unix timestamp of when the nation was founded.
    /// Note: NationStates did not track this at the beginning.
    /// For this reason, some nations are considered "founded in antiquity",
    /// which is represented by [`MaybeSystemTime::Antiquity`].
    /// A nation founded more recently would be [`MaybeSystemTime::Recorded`].
    ///
    /// Requested using [`PublicNationShard::FoundedTime`].
    pub founded_time: Option<MaybeSystemTime>,
    /// The vote of the nation in the General Assembly.
    ///
    /// Note:
    /// if the nation is not in the World Assembly,
    /// but the [`PublicNationShard::WA`] shard was not requested,
    /// the field will erroneously be `Some(`[`WAVote::Undecided`]`)`.
    ///
    /// Requested using [`PublicNationShard::GAVote`].
    /// Recommended to request with [`PublicNationShard::WA`].
    pub ga_vote: Option<WAVote>,
    /// The GDP of the nation in its national currency.
    ///
    /// Requested using [`PublicNationShard::Gdp`].
    pub gdp: Option<u64>,
    /// The description of the nation's government found on its nation page.
    ///
    /// Requested using [`PublicNationShard::GovtDesc`].
    pub govt_desc: Option<String>,
    /// The 10 most recent [`Event`]s in the nation.
    ///
    /// Requested using [`PublicNationShard::Happenings`].
    pub happenings: Option<Vec<Event>>,
    /// The average income in the nation.
    ///
    /// Requested using [`PublicNationShard::Income`].
    pub income: Option<u32>,
    /// The description of the nation's industry found on its nation page.
    ///
    /// Requested using [`PublicNationShard::IndustryDesc`].
    pub industry_desc: Option<String>,
    /// The list of (joke) descriptions of laws in the nation found on its nation page.
    ///
    /// Requested using [`PublicNationShard::Legislation`].
    pub legislation: Option<Vec<String>>,
    /// Notable facts about the nation, randomly selected by the API.
    ///
    /// Requested using [`PublicNationShard::Notable`].
    pub notable: Option<String>,
    /// All possible notable facts about the nation.
    ///
    /// Requested using [`PublicNationShard::Notables`].
    pub notables: Option<Vec<String>>,
    /// The list of policies the nation has in place.
    ///
    /// Requested using [`PublicNationShard::Policies`].
    pub policies: Option<Vec<Policy>>,
    /// The average income of the poorest 10% in the nation.
    ///
    /// Requested using [`PublicNationShard::Poorest`].
    pub poorest: Option<u32>,
    /// The region rank on today's featured World Census scale.
    ///
    /// Requested using [`PublicNationShard::RCensus`].
    pub regional_census: Option<NonZeroU16>,
    /// The average income of the richest 10% in the nation.
    ///
    /// Requested using [`PublicNationShard::Richest`].
    pub richest: Option<u32>,
    /// The vote of the nation in the Security Council.
    ///
    /// Note:
    /// if the nation is not in the World Assembly,
    /// and the [`PublicNationShard::WA`] shard was not requested,
    /// the field will erroneously be `Some(`[`WAVote::Undecided`]`)`.
    ///
    /// Requested using [`PublicNationShard::SCVote`].
    /// Recommended to request with [`PublicNationShard::WA`].
    pub sc_vote: Option<WAVote>,
    /// Describes the nation's economy as percentages controlled or funded by various sectors.
    ///
    /// Requested using [`PublicNationShard::Sectors`].
    pub sectors: Option<Sectors>,
    /// The adjectives that describe the nation's population on its nation page.
    ///
    /// Requested using [`PublicNationShard::Sensibilities`].
    pub sensibilities: Option<String>,
    /// Whether a recruitment telegram can be sent to the nation or not.
    ///
    /// Requested and configured using [`PublicNationShard::TGCanRecruit`].
    pub tg_can_recruit: Option<bool>,
    /// Whether a campaign telegram can be sent to the nation or not.
    ///
    /// Requested and configured using [`PublicNationShard::TGCanCampaign`].
    pub tg_can_campaign: Option<bool>,
    /// The world rank on today's featured World Census scale.
    ///
    /// Requested using [`PublicNationShard::WCensus`].
    pub world_census: Option<NonZeroU32>,
}

impl From<RawCensusData> for CensusCurrentData {
    fn from(value: RawCensusData) -> Self {
        let RawCensusData {
            id,
            score,
            world_rank,
            region_rank,
            percent_world_rank,
            percent_region_rank,
            ..
        } = value;
        Self {
            id,
            score,
            world_rank,
            region_rank,
            percent_world_rank,
            percent_region_rank,
        }
    }
}

impl From<RawCensusData> for CensusHistoricalData {
    fn from(value: RawCensusData) -> Self {
        let RawCensusData {
            id,
            timestamp,
            score,
            ..
        } = value;
        Self {
            id,
            timestamp,
            score,
        }
    }
}

impl TryFrom<RawDispatch> for Dispatch {
    type Error = IntoNationError;

    fn try_from(value: RawDispatch) -> Result<Self, Self::Error> {
        Ok(Dispatch {
            id: value.id,
            title: value.title,
            author: pretty_name(value.author),
            category: try_into_dispatch_category(&value.category, &value.subcategory)?,
            created: value.created,
            edited: NonZeroU64::try_from(value.edited).ok(), // field is 0 if never edited
            views: value.views,
            score: value.score,
        })
    }
}

/// Describes a national policy.
#[derive(Debug)]
pub struct Policy {
    /// The name of the policy.
    pub name: String,
    /// The banner that is associated with the policy.
    pub picture: BannerId,
    /// The category the policy belongs to. Note: this field will eventually be converted into an `enum`.
    pub category: String,
    /// The description of the policy.
    pub description: String,
}

impl TryFrom<RawPolicy> for Policy {
    type Error = IntoNationError;

    fn try_from(value: RawPolicy) -> Result<Self, Self::Error> {
        Ok(Self {
            name: value.name,
            picture: BannerId::try_from(value.pic)?,
            category: value.cat,
            description: value.desc,
        })
    }
}

/// Represents any one of the errors
/// that can go wrong between deserialization and creating the Nation struct.
#[derive(Clone, Debug, Error)]
pub enum IntoNationError {
    /// A string could not be parsed as a banner ID.
    #[error("malformed banner id: {0}")]
    BadBannerId(String),
    /// A `u8` could not be parsed as a `bool` because it was not `0` or `1`.
    #[error("boolean cannot be derived from {0}")]
    BadBooleanError(u8),
    /// A `String` could not be parsed as a [`DispatchCategory`].
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
#[derive(Clone, Debug)]
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
                council: WACouncil::SecurityCouncil,
            }),
        }
    }
}

const DEFAULT_LEADER: &str = "Leader";
const DEFAULT_RELIGION: &str = "a major religion";

impl TryFrom<RawNation> for Nation {
    type Error = IntoNationError;

    fn try_from(value: RawNation) -> Result<Self, Self::Error> {
        let name = match (value.name, value.id) {
            (Some(n), _) => Ok(n),
            (None, Some(i)) => Ok(i),
            (None, None) => Err(IntoNationError::NoNameError),
        }?;

        let happenings = value
            .happenings
            .map(|h| h.inner.into_iter().map(Event::from).collect());

        let capital = value.capital.map(|c| {
            if c.is_empty() {
                DefaultOrCustom::Default(format!("{} City", &name))
            } else {
                DefaultOrCustom::Custom(c)
            }
        });

        let wa_status = match value.unstatus {
            Some(s) => match s.as_str() {
                "WA Delegate" => Ok(Some(WAStatus::Delegate)),
                "WA Member" => Ok(Some(WAStatus::Member)),
                "Non-member" => Ok(Some(WAStatus::NonMember)),
                other => Err(IntoNationError::BadWAStatusError(other.to_string())),
            },
            None => Ok(None),
        }?;

        let ga_vote = match wa_status {
            Some(WAStatus::NonMember) => None,
            _ => value.gavote.map(WAVote::try_from).transpose()?,
        };
        let sc_vote = match wa_status {
            Some(WAStatus::NonMember) => None,
            _ => value.scvote.map(WAVote::try_from).transpose()?,
        };

        Ok(Self {
            name,
            kind: value.kind,
            full_name: value.fullname,
            motto: value.motto,
            category: value.category,
            wa_status,
            endorsements: value
                .endorsements
                .map(|e| e.split(',').map(pretty_name).collect::<Vec<String>>()),
            issues_answered: value.issues_answered,
            freedom: value.freedom,
            region: value.region,
            population: value.population,
            tax: value.tax,
            animal: value.animal,
            currency: value.currency,
            demonym_adjective: value.demonym,
            demonym_singular: value.demonym2,
            demonym_plural: value.demonym2plural,
            flag: value.flag,
            major_industry: value.majorindustry,
            government_priority: value.govtpriority,
            government: value.govt,
            founded: value.founded.map(MaybeRelativeTime::from),
            first_login: value.firstlogin,
            last_login: value.lastlogin,
            last_activity: value.lastactivity,
            influence: value.influence,
            freedom_scores: value.freedomscores,
            public_sector: value.publicsector,
            deaths: value.deaths.map(|d| d.inner),
            leader: value.leader.map(|l| {
                if l.is_empty() {
                    DefaultOrCustom::Default(DEFAULT_LEADER.to_string())
                } else {
                    DefaultOrCustom::Custom(l)
                }
            }),
            capital,
            religion: value.religion.map(|r| {
                if r.is_empty() {
                    DefaultOrCustom::Default(DEFAULT_RELIGION.to_string())
                } else {
                    DefaultOrCustom::Custom(r)
                }
            }),
            factbooks: value.factbooks,
            dispatches: value.dispatches,
            dbid: value.dbid,
            admirable: value.admirable,
            admirables: value.admirables.map(|a| a.inner),
            animal_trait: value.animaltrait,
            banner: value.banner.map(BannerId::try_from).transpose()?,
            banners: value
                .banners
                .map(|a| {
                    a.inner
                        .into_iter()
                        .map(BannerId::try_from)
                        .collect::<Result<Vec<BannerId>, IntoNationError>>()
                })
                .transpose()?,
            census: value
                .census
                .map(|c| match c.inner.first() {
                    Some(f) if f.timestamp.is_some() => Ok(CensusData::Historical(
                        c.inner
                            .into_iter()
                            .map(CensusHistoricalData::from)
                            .collect::<Vec<_>>(),
                    )),
                    Some(_) => Ok(CensusData::Current(
                        c.inner
                            .into_iter()
                            .map(CensusCurrentData::from)
                            .collect::<Vec<_>>(),
                    )),
                    None => Err(IntoNationError::NoCensusDataError),
                })
                .transpose()?,
            crime: value.crime,
            dispatch_list: value
                .dispatchlist
                .map(|v| {
                    v.inner
                        .into_iter()
                        .map(Dispatch::try_from)
                        .collect::<Result<Vec<Dispatch>, IntoNationError>>()
                })
                .transpose()?,
            factbook_list: value
                .factbooklist
                .map(|v| {
                    v.inner
                        .into_iter()
                        .map(Dispatch::try_from)
                        .collect::<Result<Vec<Dispatch>, IntoNationError>>()
                })
                .transpose()?,
            founded_time: value.foundedtime.map(MaybeSystemTime::from),
            ga_vote,
            gdp: value.gdp,
            govt_desc: value.govtdesc,
            happenings,
            income: value.income,
            industry_desc: value.industrydesc,
            legislation: value.legislation.map(|l| l.inner),
            notable: value.notable,
            // .map(|n| {
            //     eprintln!("{n}");
            //     let (first, back) = n.split_once(", ").unwrap();
            //     let (second, third) = back.split_once(" and ").unwrap();
            //     [first.to_string(), second.to_string(), third.to_string()]
            // })
            notables: value.notables.map(|n| n.inner),
            policies: value
                .policies
                .map(|v| {
                    v.inner
                        .into_iter()
                        .map(Policy::try_from)
                        .collect::<Result<Vec<Policy>, IntoNationError>>()
                })
                .transpose()?,
            poorest: value.poorest,
            regional_census: value.rcensus,
            richest: value.richest,
            sc_vote,
            sectors: value.sectors,
            sensibilities: value.sensibilities,
            // .map(|s| {
            //     let v = s.split(", ").collect::<Vec<_>>();
            //     [v[0].to_string(), v[1].to_string()]
            // })
            tg_can_recruit: value
                .tgcanrecruit
                .map(|x| match x {
                    0 => Ok(false),
                    1 => Ok(true),
                    e => Err(IntoNationError::BadBooleanError(e)),
                })
                .transpose()?,
            tg_can_campaign: value
                .tgcancampaign
                .map(|x| match x {
                    0 => Ok(false),
                    1 => Ok(true),
                    e => Err(IntoNationError::BadBooleanError(e)),
                })
                .transpose()?,
            world_census: value.wcensus,
        })
    }
}

impl Nation {
    /// Converts the XML response from NationStates to a [`Nation`].
    pub fn from_xml(xml: &str) -> Result<Self, IntoNationError> {
        Self::try_from(quick_xml::de::from_str::<RawNation>(xml)?)
    }
}

fn try_into_dispatch_category(
    main_category: &str,
    sub_category: &str,
) -> Result<DispatchCategory, IntoNationError> {
    match main_category {
        "Factbook" => Ok(DispatchCategory::Factbook(match sub_category {
            "Overview" => Ok(FactbookCategory::Overview),
            "History" => Ok(FactbookCategory::History),
            "Geography" => Ok(FactbookCategory::Geography),
            "Culture" => Ok(FactbookCategory::Culture),
            "Politics" => Ok(FactbookCategory::Politics),
            "Legislation" => Ok(FactbookCategory::Legislation),
            "Religion" => Ok(FactbookCategory::Religion),
            "Military" => Ok(FactbookCategory::Military),
            "Economy" => Ok(FactbookCategory::Economy),
            "International" => Ok(FactbookCategory::International),
            "Trivia" => Ok(FactbookCategory::Trivia),
            "Miscellaneous" => Ok(FactbookCategory::Miscellaneous),
            other => Err(IntoNationError::BadDispatchCategory(format!(
                "Factbook:{other}"
            ))),
        }?)),
        "Bulletin" => Ok(DispatchCategory::Bulletin(match sub_category {
            "Policy" => Ok(BulletinCategory::Policy),
            "News" => Ok(BulletinCategory::News),
            "Opinion" => Ok(BulletinCategory::Opinion),
            "Campaign" => Ok(BulletinCategory::Campaign),
            other => Err(IntoNationError::BadDispatchCategory(format!(
                "Bulletin:{other}"
            ))),
        }?)),
        "Account" => Ok(DispatchCategory::Account(match sub_category {
            "Military" => Ok(AccountCategory::Military),
            "Trade" => Ok(AccountCategory::Trade),
            "Sport" => Ok(AccountCategory::Sport),
            "Drama" => Ok(AccountCategory::Drama),
            "Diplomacy" => Ok(AccountCategory::Diplomacy),
            "Science" => Ok(AccountCategory::Science),
            "Culture" => Ok(AccountCategory::Culture),
            "Other" => Ok(AccountCategory::Other),
            other => Err(IntoNationError::BadDispatchCategory(format!(
                "Account:{other}"
            ))),
        }?)),
        "Meta" => Ok(DispatchCategory::Meta(match sub_category {
            "Gameplay" => Ok(MetaCategory::Gameplay),
            "Reference" => Ok(MetaCategory::Reference),
            other => Err(IntoNationError::BadDispatchCategory(format!(
                "Meta:{other}"
            ))),
        }?)),
        other => Err(IntoNationError::BadDispatchCategory(other.to_string())),
    }
}
