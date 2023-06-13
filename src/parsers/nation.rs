use crate::pretty_name;
use crate::shards::world_shards::{
    AccountCategory, BulletinCategory, DispatchCategory, FactbookCategory, MetaCategory,
};
use quick_xml::DeError;
use serde::{Deserialize, Deserializer};
use std::fmt::Debug;
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use thiserror::Error;

#[derive(Debug, Deserialize)]
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
    #[serde(rename = "NAME")]
    name: Option<String>,
    #[serde(rename = "TYPE")]
    kind: Option<String>,
    #[serde(rename = "FULLNAME")]
    full_name: Option<String>,
    #[serde(rename = "MOTTO")]
    motto: Option<String>,
    #[serde(rename = "CATEGORY")]
    category: Option<String>,
    #[serde(rename = "UNSTATUS")] // deserialize_with = "handle_wa_status"
    wa_status: Option<String>,
    #[serde(rename = "ENDORSEMENTS")] // deserialize_with = "unwrap_endorsement_list"
    endorsements: Option<String>,
    #[serde(rename = "ISSUES_ANSWERED")]
    issues_answered: Option<u32>,
    #[serde(rename = "FREEDOM")]
    freedom: Option<Freedoms>,
    #[serde(rename = "REGION")]
    region: Option<String>,
    #[serde(rename = "POPULATION")]
    population: Option<u32>,
    #[serde(rename = "TAX")]
    tax: Option<f32>,
    #[serde(rename = "ANIMAL")]
    animal: Option<String>,
    #[serde(rename = "CURRENCY")]
    currency: Option<String>,
    #[serde(rename = "DEMONYM")]
    demonym: Option<String>,
    #[serde(rename = "DEMONYM2")]
    demonym2: Option<String>,
    #[serde(rename = "DEMONYM2PLURAL")]
    demonym2_plural: Option<String>,
    #[serde(rename = "FLAG")]
    flag: Option<String>,
    #[serde(rename = "MAJORINDUSTRY")]
    major_industry: Option<String>,
    #[serde(rename = "GOVTPRIORITY")]
    government_priority: Option<String>,
    #[serde(rename = "GOVT")]
    government: Option<Government>,
    #[serde(rename = "FOUNDED")]
    founded: Option<String>,
    #[serde(rename = "FIRSTLOGIN")]
    first_login: Option<u64>,
    #[serde(rename = "LASTLOGIN")]
    last_login: Option<u64>,
    #[serde(rename = "LASTACTIVITY")]
    last_activity: Option<String>,
    #[serde(rename = "INFLUENCE")]
    influence: Option<String>,
    #[serde(rename = "FREEDOMSCORES")]
    freedom_scores: Option<FreedomScores>,
    #[serde(rename = "PUBLICSECTOR")]
    public_sector: Option<f32>,
    #[serde(rename = "DEATHS")]
    deaths: Option<Deaths>,
    #[serde(rename = "LEADER")]
    leader: Option<String>,
    #[serde(rename = "CAPITAL")]
    capital: Option<String>,
    #[serde(rename = "RELIGION")]
    religion: Option<String>,
    #[serde(rename = "FACTBOOKS")]
    factbooks: Option<u16>,
    #[serde(rename = "DISPATCHES")]
    dispatches: Option<u16>,
    #[serde(rename = "DBID")]
    dbid: Option<u32>,
    // END default
    #[serde(rename = "ADMIRABLE")]
    admirable: Option<String>,
    #[serde(rename = "ADMIRABLES")]
    admirables: Option<Vec<String>>,
    #[serde(rename = "ANIMALTRAIT")]
    animal_trait: Option<String>,
    #[serde(rename = "BANNER")]
    banner: Option<String>, // TODO: Option<BannerID>
    #[serde(rename = "BANNERS")]
    banners: Option<Vec<String>>, // TODO: Option<Vec<BannerID>>
    #[serde(rename = "CENSUS")]
    census: Option<Vec<CensusData>>,
    #[serde(rename = "CRIME")]
    crime: Option<String>,
    #[serde(rename = "DISPATCHLIST")]
    dispatch_list: Option<RawDispatchList>,
    // #[serde(rename = "FACTBOOKLIST")]
    // factbook_list: Option<Vec<Factbook>>,
    #[serde(rename = "FOUNDEDTIME")]
    founded_time: Option<u64>,
    // #[serde(rename = "GAVOTE")]
    // ga_vote: Option<WAVote>,
    #[serde(rename = "GDP")]
    gdp: Option<u64>,
    #[serde(rename = "GOVTDESC")]
    govt_desc: Option<String>,
    // happenings: Option<Vec<Event>>,
    // income: Option<u32>,
    industry_desc: Option<String>,
    legislation: Option<String>,
    notable: Option<String>,
    notables: Option<Vec<String>>,
    // policies: Option<Vec<Policy>>,
    poorest: Option<u32>,
    regional_census: Option<NonZeroU16>,
    richest: Option<u32>,
    // sc_vote: Option<WAVote>,
    // sectors: Option<GovernmentSectors>,
    sensibilities: Option<String>,
    tg_can_recruit: Option<bool>,
    tg_can_campaign: Option<bool>,
    world_census: Option<NonZeroU32>,
}

#[derive(Debug)]
pub enum WAStatus {
    Delegate,
    Member,
    NonMember,
}

#[derive(Debug, Deserialize)]
pub struct Government {
    #[serde(rename = "ADMINISTRATION")]
    pub administration: f32,
    #[serde(rename = "DEFENCE")]
    pub defence: f32,
    #[serde(rename = "EDUCATION")]
    pub education: f32,
    #[serde(rename = "ENVIRONMENT")]
    pub environment: f32,
    #[serde(rename = "HEALTHCARE")]
    pub healthcare: f32,
    #[serde(rename = "COMMERCE")]
    pub commerce: f32,
    #[serde(rename = "INTERNATIONALAID")]
    pub international_aid: f32,
    #[serde(rename = "LAWANDORDER")]
    pub law_and_order: f32,
    #[serde(rename = "PUBLICTRANSPORT")]
    pub public_transport: f32,
    #[serde(rename = "SOCIALEQUALITY")]
    pub social_equality: f32,
    #[serde(rename = "SPIRITUALITY")]
    pub spirituality: f32,
    #[serde(rename = "WELFARE")]
    pub welfare: f32,
}

#[derive(Debug, Deserialize)]
pub struct Freedoms {
    #[serde(rename = "CIVILRIGHTS")]
    pub civil_rights: String,
    #[serde(rename = "ECONOMY")]
    pub economy: String,
    #[serde(rename = "POLITICALFREEDOM")]
    pub political_freedom: String,
}

#[derive(Debug, Deserialize)]
pub struct FreedomScores {
    #[serde(rename = "CIVILRIGHTS")]
    pub civil_rights: u8,
    #[serde(rename = "ECONOMY")]
    pub economy: u8,
    #[serde(rename = "POLITICALFREEDOM")]
    pub political_freedom: u8,
}

#[derive(Debug, Deserialize)]
pub struct Deaths {
    #[serde(rename = "CAUSE")]
    pub causes: Vec<Cause>,
}

#[derive(Debug, Deserialize)]
pub struct Cause {
    #[serde(rename = "@type")]
    pub kind: String,
    #[serde(rename = "$value")]
    pub frequency: f32,
}

#[derive(Debug, Deserialize)]
pub struct Admirables {
    #[serde(rename = "ADMIRABLES", deserialize_with = "unwrap_list")]
    pub traits: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct Census {
    #[serde(rename = "SCALE")]
    pub data: Vec<CensusData>,
}

#[derive(Debug, Deserialize)]
pub struct CensusData {
    #[serde(rename = "@id")]
    pub id: u8,
    #[serde(rename = "SCORE")]
    pub score: Option<f64>,
    #[serde(rename = "RANK")]
    pub world_rank: Option<u32>,
    #[serde(rename = "RRANK")]
    pub region_rank: Option<u32>,
    #[serde(rename = "PRANK")]
    pub percent_world_rank: Option<u8>,
    #[serde(rename = "PRRANK")]
    pub percent_region_rank: Option<u8>,
    #[serde(rename = "TIMESTAMP")]
    pub timestamp: Option<NonZeroU64>,
}

#[derive(Debug, Deserialize)]
pub struct RawDispatchList {
    #[serde(rename = "DISPATCH")]
    pub dispatches: Vec<RawDispatch>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct RawDispatch {
    #[serde(rename = "@id")]
    pub id: u32,
    pub title: String,
    pub author: String,
    pub category: String,
    pub subcategory: String,
    pub created: u64,
    pub edited: u64,
    pub views: u32,
    pub score: u32,
}

fn unwrap_list<'de, D>(deserializer: D) -> Result<Option<Vec<String>>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct List {
        inner: Option<Vec<String>>,
    }
    Ok(List::deserialize(deserializer)?.inner)
}

#[derive(Debug)]
pub struct Nation {
    pub name: String,
    pub kind: Option<String>,
    pub full_name: Option<String>,
    pub motto: Option<String>,
    pub category: Option<String>,
    // deserialize_with = "handle_wa_status"
    pub wa_status: Option<WAStatus>,
    // deserialize_with = "unwrap_endorsement_list"
    pub endorsements: Option<Vec<String>>,
    pub issues_answered: Option<u32>,
    pub freedom: Option<Freedoms>,
    pub region: Option<String>,
    pub population: Option<u32>,
    pub tax: Option<f32>,
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
    pub public_sector: Option<f32>,
    pub deaths: Option<Deaths>,
    pub leader: Option<String>,
    pub capital: Option<String>,
    pub religion: Option<String>,
    pub factbooks: Option<u16>,
    pub dispatches: Option<u16>,
    pub dbid: Option<u32>,
    // END default
    pub admirable: Option<String>,
    pub admirables: Option<Vec<String>>,
    pub animal_trait: Option<String>,
    pub banner: Option<String>,       // TODO: Option<BannerID>
    pub banners: Option<Vec<String>>, // TODO: Option<Vec<BannerID>>
    pub census: Option<Vec<CensusData>>,
    pub crime: Option<String>,
    pub dispatch_list: Option<Vec<Dispatch>>,
    // #[serde(rename = "FACTBOOKLIST")]
    // factbook_list: Option<Vec<Factbook>>,
    pub founded_time: Option<u64>,
    // #[serde(rename = "GAVOTE")]
    // ga_vote: Option<WAVote>,
    pub gdp: Option<u64>,
    pub govt_desc: Option<String>,
    // happenings: Option<Vec<Event>>,
    // income: Option<u32>,
    pub industry_desc: Option<String>,
    pub legislation: Option<String>,
    pub notable: Option<String>,
    pub notables: Option<Vec<String>>,
    // policies: Option<Vec<Policy>>,
    pub poorest: Option<u32>,
    pub regional_census: Option<NonZeroU16>,
    pub richest: Option<u32>,
    // sc_vote: Option<WAVote>,
    // sectors: Option<GovernmentSectors>,
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
}

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

        let wa_status = if let Some(s) = value.wa_status {
            match s.as_str() {
                "WA Delegate" => Ok(Some(WAStatus::Delegate)),
                "WA Member" => Ok(Some(WAStatus::Member)),
                "Non-member" => Ok(Some(WAStatus::NonMember)),
                other => Err(IntoNationError::MalformedWAStatusError(other.to_string())),
            }
        } else {
            Ok(None)
        }?;

        let endorsements: Option<Vec<String>> = value
            .endorsements
            .map(|e| e.split(|c| c == ',').map(pretty_name).collect());

        let dispatch_list: Option<Vec<Dispatch>> = if let Some(l) = value.dispatch_list {
            Some(
                l.dispatches
                    .iter()
                    .map(|d| {
                        Ok(Dispatch {
                            id: d.id,
                            title: d.title.clone(),
                            author: d.author.clone(),
                            category: main_and_sub_to_rust(&d.category, &d.subcategory)?,
                            created: d.created,
                            edited: d.edited,
                            views: d.views,
                            score: d.score,
                        })
                    })
                    // collect can take an Iterator<Item=Result<T, E>>
                    // and transform it into a Result<Iterator<Item=T>, E>,
                    // and we can propagate that afterward
                    .collect::<Result<Vec<Dispatch>, IntoNationError>>()?,
            )
        } else {
            None
        };

        Ok(Self {
            name,
            kind: value.kind,
            full_name: value.full_name,
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
            demonym2_plural: value.demonym2_plural,
            flag: value.flag,
            major_industry: value.major_industry,
            government_priority: value.government_priority,
            government: value.government,
            founded: value.founded,
            first_login: value.first_login,
            last_login: value.last_login,
            last_activity: value.last_activity,
            influence: value.influence,
            freedom_scores: value.freedom_scores,
            public_sector: value.public_sector,
            deaths: value.deaths,
            leader: value.leader,
            capital: value.capital,
            religion: value.religion,
            factbooks: value.factbooks,
            dispatches: value.dispatches,
            dbid: value.dbid,
            admirable: value.admirable,
            admirables: value.admirables,
            animal_trait: value.animal_trait,
            banner: value.banner,
            banners: value.banners,
            census: value.census,
            crime: value.crime,
            dispatch_list,
            founded_time: value.founded_time,
            gdp: value.gdp,
            govt_desc: value.govt_desc,
            industry_desc: value.industry_desc,
            legislation: value.legislation,
            notable: value.notable,
            notables: value.notables,
            poorest: value.poorest,
            regional_census: value.regional_census,
            richest: value.richest,
            sensibilities: value.sensibilities,
            tg_can_recruit: value.tg_can_recruit,
            tg_can_campaign: value.tg_can_campaign,
            world_census: value.world_census,
        })
    }
}

impl Nation {
    pub fn from_xml<'a>(xml: impl Into<&'a str>) -> Result<Self, IntoNationError> {
        Self::try_from(quick_xml::de::from_str::<RawNation>(xml.into())?)
    }
}

fn main_and_sub_to_rust(
    main_category: &str,
    sub_category: &str,
) -> Result<DispatchCategory, IntoNationError> {
    match main_category {
        "Factbook" => Ok(DispatchCategory::Factbook(Some(match sub_category {
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
        }?))),
        "Bulletin" => Ok(DispatchCategory::Bulletin(Some(match sub_category {
            "Policy" => Ok(BulletinCategory::Policy),
            "News" => Ok(BulletinCategory::News),
            "Opinion" => Ok(BulletinCategory::Opinion),
            "Campaign" => Ok(BulletinCategory::Campaign),
            other => Err(IntoNationError::MalformedDispatchCategory(format!(
                "Bulletin:{other}"
            ))),
        }?))),
        "Account" => Ok(DispatchCategory::Account(Some(match sub_category {
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
        }?))),
        "Meta" => Ok(DispatchCategory::Meta(Some(match sub_category {
            "Gameplay" => Ok(MetaCategory::Gameplay),
            "Reference" => Ok(MetaCategory::Reference),
            other => Err(IntoNationError::MalformedDispatchCategory(format!(
                "Meta:{other}"
            ))),
        }?))),
        other => Err(IntoNationError::MalformedDispatchCategory(
            other.to_string(),
        )),
    }
}
