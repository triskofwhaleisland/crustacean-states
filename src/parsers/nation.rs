//! The nation parser.

use crate::parsers::happenings::Event;
use crate::pretty_name;
use crate::shards::public_nation::PublicNationShard;
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
struct RawPolicy {
    #[serde(rename = "NAME")]
    name: String,
    #[serde(rename = "PIC")]
    picture: String,
    #[serde(rename = "CAT")]
    category: String,
    #[serde(rename = "DESC")]
    description: String,
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
    /// Gets the pre-title of the nation.
    ///
    /// Requested using
    /// [`PublicNationShard::Type`]
    /// (`type` is a reserved word in Rust, so `kind` is used in its place).
    pub kind: Option<String>,
    /// Gets the full name of the nation.
    ///
    /// Requested using [`PublicNationShard::FullName`].
    pub full_name: Option<String>,
    /// Gets the motto of the nation.
    ///
    /// Requested using [`PublicNationShard::Motto`].
    pub motto: Option<String>,
    /// Gets the category of the nation.
    /// Note that this is currently a `String` representation,
    /// but will eventually become its own type.
    ///
    /// Requested using [`PublicNationShard::Category`].
    pub category: Option<String>,
    /// Gets the WA status of the nation.
    ///
    /// Requested using [`PublicNationShard::WA`].
    pub wa_status: Option<WAStatus>,
    /// Gets a list of nations that endorse the nation.
    ///
    /// Requested using [`PublicNationShard::Endorsements`].
    pub endorsements: Option<Vec<String>>,
    /// Gets the number of issues answered by the nation.
    ///
    /// Requested using [`PublicNationShard::Answered`].
    pub issues_answered: Option<u32>,
    /// Gets the freedom statistics of the nation.
    ///
    /// Requested using [`PublicNationShard::Freedom`].
    pub freedom: Option<Freedoms>,
    /// Gets the region that the nation resides in.
    ///
    /// Requested using [`PublicNationShard::Region`].
    pub region: Option<String>,
    pub population: Option<u32>,
    pub tax: Option<f64>,
    pub animal: Option<String>,
    pub currency: Option<String>,
    pub demonym: Option<String>,
    pub demonym2: Option<String>,
    pub demonym2_plural: Option<String>,
    pub flag: Option<String>,
    pub major_industry: Option<String>,
    pub government_priority: Option<String>,
    pub government: Option<Government>,
    pub founded: Option<String>,
    pub first_login: Option<u64>,
    pub last_login: Option<u64>,
    pub last_activity: Option<String>,
    pub influence: Option<String>,
    pub freedom_scores: Option<FreedomScores>,
    pub public_sector: Option<f64>,
    pub deaths: Option<Vec<Cause>>,
    pub leader: Option<Either<String, String>>,
    pub capital: Option<Either<String, String>>,
    pub religion: Option<Either<String, String>>,
    pub factbooks: Option<u16>,
    pub dispatches: Option<u16>,
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
    pub founded_time: Option<u64>,
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
            picture: BannerId::try_from(value.picture)?,
            category: value.category,
            description: value.description,
        })
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

        let wa_status = if let Some(s) = value.unstatus {
            match s.as_str() {
                "WA Delegate" => Ok(Some(WAStatus::Delegate)),
                "WA Member" => Ok(Some(WAStatus::Member)),
                "Non-member" => Ok(Some(WAStatus::NonMember)),
                other => Err(IntoNationError::MalformedWAStatusError(other.to_string())),
            }
        } else {
            Ok(None)
        }?;

        let endorsements = value.endorsements.map(|e| {
            e.split(|c| c == ',')
                .map(pretty_name)
                .collect::<Vec<String>>()
        });

        let capital = value.capital.map(|c| {
            if c.is_empty() {
                Right(format!("{name} City"))
            } else {
                Left(c)
            }
        });
        let leader = value.leader.map(|l| {
            if l.is_empty() {
                Right(DEFAULT_LEADER.to_string())
            } else {
                Left(l)
            }
        });
        let religion = value.religion.map(|r| {
            if r.is_empty() {
                Right(DEFAULT_RELIGION.to_string())
            } else {
                Left(r)
            }
        });

        let deaths = value.deaths.map(|d| d.causes);
        let admirables = value.admirables.map(|a| a.traits);
        let banner = value.banner.map(BannerId::try_from).transpose()?;
        let banners = value
            .banners
            .map(|a| {
                a.banners
                    .into_iter()
                    .map(BannerId::try_from)
                    .collect::<Result<Vec<BannerId>, IntoNationError>>()
            })
            .transpose()?;
        let census = value.census.map(|c| c.data);
        let legislation = value.legislation.map(|l| l.laws);
        let notables = value.notables.map(|n| n.notables);
        let policies = value
            .policies
            .map(|v| {
                v.policies
                    .into_iter()
                    .map(Policy::try_from)
                    .collect::<Result<Vec<Policy>, IntoNationError>>()
            })
            .transpose()?;

        let dispatch_list = value
            .dispatchlist
            .map(|v| {
                v.dispatches
                    .into_iter()
                    .map(Dispatch::try_from)
                    .collect::<Result<Vec<Dispatch>, IntoNationError>>()
            })
            .transpose()?;
        let factbook_list = value
            .factbooklist
            .map(|v| {
                v.factbooks
                    .into_iter()
                    .map(Dispatch::try_from)
                    .collect::<Result<Vec<Dispatch>, IntoNationError>>()
            })
            .transpose()?;

        let ga_vote = value.gavote.map(|v| try_into_wa_vote(&v)).transpose()?;
        let sc_vote = value.scvote.map(|v| try_into_wa_vote(&v)).transpose()?;

        let happenings = value
            .happenings
            .map(|h| h.events.into_iter().map(Event::from).collect());

        Ok(Self {
            name,
            kind: value.kind,
            full_name: value.fullname,
            motto: value.motto,
            category: value.category,
            wa_status,
            endorsements,
            issues_answered: value.issues_answered,
            freedom: value.freedom,
            region: value.region,
            population: value.population,
            tax: value.tax,
            animal: value.animal,
            currency: value.currency,
            demonym: value.demonym,
            demonym2: value.demonym2,
            demonym2_plural: value.demonym2plural,
            flag: value.flag,
            major_industry: value.majorindustry,
            government_priority: value.govtpriority,
            government: value.govt,
            founded: value.founded,
            first_login: value.firstlogin,
            last_login: value.lastlogin,
            last_activity: value.lastactivity,
            influence: value.influence,
            freedom_scores: value.freedomscores,
            public_sector: value.publicsector,
            deaths,
            leader,
            capital,
            religion,
            factbooks: value.factbooks,
            dispatches: value.dispatches,
            dbid: value.dbid,
            admirable: value.admirable,
            admirables,
            animal_trait: value.animaltrait,
            banner,
            banners,
            census,
            crime: value.crime,
            dispatch_list,
            factbook_list,
            founded_time: value.foundedtime,
            ga_vote,
            gdp: value.gdp,
            govt_desc: value.govtdesc,
            happenings,
            income: value.income,
            industry_desc: value.industrydesc,
            legislation,
            notable: value.notable,
            notables,
            policies,
            poorest: value.poorest,
            regional_census: value.rcensus,
            richest: value.richest,
            sc_vote,
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
