//! The nation parser.

use crate::parsers::happenings::Event;
use crate::pretty_name;
use crate::shards::public_nation::PublicNationShard;
use crate::shards::NSRequest;
use crate::shards::world::{
    AccountCategory, BannerId, BulletinCategory, DispatchCategory, FactbookCategory, MetaCategory,
};
use crate::shards::world_assembly::WACouncil;
use either::{Either, Left, Right};
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
    // default shards from ?nation=
    // #[serde(rename = "$value", deserialize_with = "handle_name")]
    /// The name of the nation.
    ///
    // Note: this is the *only* field that is ever guaranteed to be filled in.
    // If the [`PublicNationShard::Name`] field was not requested,
    // this is obtained from the results of [`pretty_name`], which can
    //
    // [`PublicNationShard::Name`]:
    // crate::shards::public_nation_shards::PublicNationShard::Name
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
    tgcanrecruit: Option<bool>,
    tgcancampaign: Option<bool>,
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

#[derive(Debug, Deserialize)]
struct Deaths {
    #[serde(rename = "CAUSE")]
    causes: Vec<Cause>,
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

#[derive(Debug, Deserialize)]
struct Admirables {
    #[serde(rename = "ADMIRABLE")]
    traits: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Banners {
    #[serde(rename = "BANNER")]
    banners: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Census {
    #[serde(rename = "SCALE")]
    data: Vec<CensusData>,
}

/// A piece of World Census data about the nation.
//noinspection SpellCheckingInspection
#[derive(Clone, Debug, Deserialize)]
pub struct CensusData {
    /// The ID used for the data point. For example,
    #[serde(rename = "@id")]
    pub id: u8,
    /// The score of the nation on the Census scale.
    #[serde(rename = "SCORE")]
    pub score: Option<f64>,
    /// The placement the nation holds in the world ranking.
    #[serde(rename = "RANK")]
    pub world_rank: Option<u32>,
    /// The placement the nation holds in its region ranking.
    #[serde(rename = "RRANK")]
    pub region_rank: Option<u32>,
    /// Kind of like a percentile, but backwards:
    /// the nation is in the top x% of nations according to this category,
    /// with x being this field.
    /// Note that all percentiles are to the nearest whole except for <1%,
    /// which are to the nearest tenth.
    #[serde(rename = "PRANK")]
    pub percent_world_rank: Option<f64>,
    /// Like `percent_world_rank`, but only for the nation's region ranking.
    #[serde(rename = "PRRANK")]
    pub percent_region_rank: Option<f64>,
    /// When the nation was ranked.
    /// This usually corresponds to a time around the major
    /// (midnight Eastern Time) or minor (noon Eastern Time) game updates.
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: Option<NonZeroU64>,
}

#[derive(Debug, Deserialize)]
struct RawDispatchList {
    #[serde(rename = "DISPATCH", default)]
    dispatches: Vec<RawDispatch>,
}

#[derive(Debug, Deserialize)]
struct RawFactbookList {
    #[serde(rename = "FACTBOOK", default)]
    factbooks: Vec<RawDispatch>, // only containing factbooks!!
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
    #[serde(rename = "EVENT")]
    events: Vec<RawEvent>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(super) struct RawEvent {
    pub(crate) timestamp: u64,
    pub(crate) text: String,
}

#[derive(Debug, Deserialize)]
struct Legislation {
    #[serde(rename = "LAW")]
    laws: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Notables {
    #[serde(rename = "NOTABLE")]
    notables: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Policies {
    #[serde(rename = "POLICY")]
    policies: Vec<RawPolicy>,
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
    ///
    /// Requested using [`PublicNationShard::Founded`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub founded: Option<MaybeAncient>,
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
    /// the [`Left`] variant is filled with the custom leader's name;
    /// if not, the [`Right`] variant is filled with the default leader name.
    ///
    /// Requested using [`PublicNationShard::Leader`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub leader: Option<Either<String, String>>,
    /// The national capital.
    ///
    /// If there is a custom capital,
    /// the [`Left`] variant is filled with the custom capital name;
    /// if not, the [`Right`] variant is filled with the default capital name.
    ///
    /// Requested using [`PublicNationShard::Capital`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub capital: Option<Either<String, String>>,
    /// The national religion.
    ///
    /// If there is a custom religion,
    /// the [`Left`] variant is filled with the custom religion;
    /// if not, the [`Right`] variant is filled with the default religion.
    ///
    /// Requested using [`PublicNationShard::Religion`].
    /// [`NSRequest::new_nation_standard`] requests this field.
    pub religion: Option<Either<String, String>>,
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
    pub admirable: Option<String>,
    pub admirables: Option<Vec<String>>,
    pub animal_trait: Option<String>,
    pub banner: Option<BannerId>,
    pub banners: Option<Vec<BannerId>>,
    pub census: Option<Vec<CensusData>>,
    pub crime: Option<String>,
    pub dispatch_list: Option<Vec<Dispatch>>,
    pub factbook_list: Option<Vec<Dispatch>>,
    pub founded_time: Option<Option<NonZeroU64>>,
    pub ga_vote: Option<WAVote>,
    pub gdp: Option<u64>,
    pub govt_desc: Option<String>,
    pub happenings: Option<Vec<Event>>,
    pub income: Option<u32>,
    pub industry_desc: Option<String>,
    pub legislation: Option<Vec<String>>,
    pub notable: Option<String>,
    pub notables: Option<Vec<String>>,
    pub policies: Option<Vec<Policy>>,
    pub poorest: Option<u32>,
    pub regional_census: Option<NonZeroU16>,
    pub richest: Option<u32>,
    pub sc_vote: Option<WAVote>,
    pub sectors: Option<Sectors>,
    pub sensibilities: Option<String>,
    pub tg_can_recruit: Option<bool>,
    pub tg_can_campaign: Option<bool>,
    pub world_census: Option<NonZeroU32>,
}

#[derive(Clone, Debug)]
pub struct Dispatch {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub category: DispatchCategory,
    pub created: u64,
    pub edited: u64,
    pub views: u32,
    pub score: u32,
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
            edited: value.edited,
            views: value.views,
            score: value.score,
        })
    }
}

/// Describes a national policy.
#[derive(Debug)]
pub struct Policy {
    pub name: String,
    pub picture: BannerId,
    pub category: String,
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

#[derive(Debug)]
pub enum MaybeAncient {
    Recorded(String),
    Antiquity,
}

impl From<MaybeAncient> for Option<String> {
    fn from(value: MaybeAncient) -> Self {
        match value {
            MaybeAncient::Recorded(s) => Some(s),
            MaybeAncient::Antiquity => None,
        }
    }
}

impl From<String> for MaybeAncient {
    fn from(value: String) -> Self {
        if value.is_empty() {
            MaybeAncient::Antiquity
        } else {
            MaybeAncient::Recorded(value)
        }
    }
}

#[derive(Clone, Debug, Error)]
pub enum IntoNationError {
    #[error("deserialization failed")]
    DeserializationError {
        #[from]
        source: DeError,
        // backtrace: Backtrace,
    },
    #[error("could not find a nation name in response")]
    NoNameError,
    #[error("malformed WA status response: {0}")]
    MalformedWAStatusError(String),
    #[error("malformed dispatch category: {0}")]
    MalformedDispatchCategory(String),
    #[error("malformed wa vote: {bad_vote} in {council:?}")]
    MalformedWAVote {
        bad_vote: String,
        council: WACouncil,
    },
    #[error("malformed banner id: {0}")]
    MalformedBannerId(String),
}

#[derive(Clone, Debug)]
pub enum WAVote {
    For,
    Against,
    Undecided,
}

const DEFAULT_LEADER: &'static str = "Leader";
const DEFAULT_RELIGION: &'static str = "a major religion";

impl TryFrom<RawNation> for Nation {
    type Error = IntoNationError;

    fn try_from(value: RawNation) -> Result<Self, Self::Error> {
        let name = match value.name {
            Some(n) => Ok(n),
            None => match value.id {
                Some(i) => Ok(i),
                None => Err(IntoNationError::NoNameError),
            },
        }?;

        let happenings = value
            .happenings
            .map(|h| h.events.into_iter().map(Event::from).collect());

        let capital = value.capital.map(|c| {
            if c.is_empty() {
                Left(format!("{} City", &name))
            } else {
                Right(c)
            }
        });

        Ok(Self {
            name,
            kind: value.kind,
            full_name: value.fullname,
            motto: value.motto,
            category: value.category,
            wa_status: if let Some(s) = value.unstatus {
                match s.as_str() {
                    "WA Delegate" => Ok(Some(WAStatus::Delegate)),
                    "WA Member" => Ok(Some(WAStatus::Member)),
                    "Non-member" => Ok(Some(WAStatus::NonMember)),
                    other => Err(IntoNationError::MalformedWAStatusError(other.to_string())),
                }
            } else {
                Ok(None)
            }?,
            endorsements: value.endorsements.map(|e| {
                e.split(|c| c == ',')
                    .map(pretty_name)
                    .collect::<Vec<String>>()
            }),
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
            founded: value.founded.map(MaybeAncient::from),
            first_login: value.firstlogin,
            last_login: value.lastlogin,
            last_activity: value.lastactivity,
            influence: value.influence,
            freedom_scores: value.freedomscores,
            public_sector: value.publicsector,
            deaths: value.deaths.map(|d| d.causes),
            leader: value.leader.map(|l| {
                if l.is_empty() {
                    Left(DEFAULT_LEADER.to_string())
                } else {
                    Right(l)
                }
            }),
            capital,
            religion: value.religion.map(|r| {
                if r.is_empty() {
                    Left(DEFAULT_RELIGION.to_string())
                } else {
                    Right(r)
                }
            }),
            factbooks: value.factbooks,
            dispatches: value.dispatches,
            dbid: value.dbid,
            admirable: value.admirable,
            admirables: value.admirables.map(|a| a.traits),
            animal_trait: value.animaltrait,
            banner: value.banner.map(BannerId::try_from).transpose()?,
            banners: value
                .banners
                .map(|a| {
                    a.banners
                        .into_iter()
                        .map(BannerId::try_from)
                        .collect::<Result<Vec<BannerId>, IntoNationError>>()
                })
                .transpose()?,
            census: value.census.map(|c| c.data),
            crime: value.crime,
            dispatch_list: value
                .dispatchlist
                .map(|v| {
                    v.dispatches
                        .into_iter()
                        .map(Dispatch::try_from)
                        .collect::<Result<Vec<Dispatch>, IntoNationError>>()
                })
                .transpose()?,
            factbook_list: value
                .factbooklist
                .map(|v| {
                    v.factbooks
                        .into_iter()
                        .map(Dispatch::try_from)
                        .collect::<Result<Vec<Dispatch>, IntoNationError>>()
                })
                .transpose()?,
            founded_time: value.foundedtime.map(|t| match NonZeroU64::try_from(t) {
                Ok(x) => Some(x),
                Err(_) => None,
            }),
            ga_vote: value.gavote.map(|v| try_into_wa_vote(&v)).transpose()?,
            gdp: value.gdp,
            govt_desc: value.govtdesc,
            happenings,
            income: value.income,
            industry_desc: value.industrydesc,
            legislation: value.legislation.map(|l| l.laws),
            notable: value.notable,
            notables: value.notables.map(|n| n.notables),
            policies: value
                .policies
                .map(|v| {
                    v.policies
                        .into_iter()
                        .map(Policy::try_from)
                        .collect::<Result<Vec<Policy>, IntoNationError>>()
                })
                .transpose()?,
            poorest: value.poorest,
            regional_census: value.rcensus,
            richest: value.richest,
            sc_vote: value.scvote.map(|v| try_into_wa_vote(&v)).transpose()?,
            sectors: value.sectors,
            sensibilities: value.sensibilities,
            tg_can_recruit: value.tgcanrecruit,
            tg_can_campaign: value.tgcancampaign,
            world_census: value.wcensus,
        })
    }
}

impl Nation {
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
            other => Err(IntoNationError::MalformedDispatchCategory(format!(
                "Factbook:{other}"
            ))),
        }?)),
        "Bulletin" => Ok(DispatchCategory::Bulletin(match sub_category {
            "Policy" => Ok(BulletinCategory::Policy),
            "News" => Ok(BulletinCategory::News),
            "Opinion" => Ok(BulletinCategory::Opinion),
            "Campaign" => Ok(BulletinCategory::Campaign),
            other => Err(IntoNationError::MalformedDispatchCategory(format!(
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
            other => Err(IntoNationError::MalformedDispatchCategory(format!(
                "Account:{other}"
            ))),
        }?)),
        "Meta" => Ok(DispatchCategory::Meta(match sub_category {
            "Gameplay" => Ok(MetaCategory::Gameplay),
            "Reference" => Ok(MetaCategory::Reference),
            other => Err(IntoNationError::MalformedDispatchCategory(format!(
                "Meta:{other}"
            ))),
        }?)),
        other => Err(IntoNationError::MalformedDispatchCategory(
            other.to_string(),
        )),
    }
}

fn try_into_wa_vote(vote: &str) -> Result<WAVote, IntoNationError> {
    match vote {
        "FOR" => Ok(WAVote::For),
        "AGAINST" => Ok(WAVote::Against),
        "UNDECIDED" => Ok(WAVote::Undecided),
        other => Err(IntoNationError::MalformedWAVote {
            bad_vote: other.to_string(),
            council: WACouncil::SecurityCouncil,
        }),
    }
}
