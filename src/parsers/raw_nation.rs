use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};

use serde::Deserialize;

use crate::{
    models::dispatch::{
        AccountCategory, BulletinCategory, DispatchCategory, FactbookCategory, MetaCategory,
    },
    parsers::{
        happenings::Event,
        nation::{
            BannerId, Cause, FreedomScores, Freedoms, Government, IntoNationError, Nation, Policy,
            Sectors, StandardNation, WAStatus, WAVote,
        },
        CensusCurrentData, CensusData, CensusHistoricalData, DefaultOrCustom, Dispatch,
        MaybeRelativeTime, MaybeSystemTime, RawCensus, RawHappenings,
    },
    pretty_name,
};

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawNation {
    // default shards
    #[serde(rename = "@id")] // attribute: "id"
    id: Option<String>,
    name: Option<String>,
    #[serde(rename = "TYPE")] // why do they like this word so much :weary:
    kind: Option<String>,
    fullname: Option<String>,
    motto: Option<String>,
    category: Option<String>,
    unstatus: Option<String>,
    endorsements: Option<String>,
    issues_answered: Option<u32>,
    freedom: Option<RawFreedoms>,
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
    govt: Option<RawGovernment>,
    founded: Option<String>,
    firstlogin: Option<u64>,
    lastlogin: Option<u64>,
    lastactivity: Option<String>,
    influence: Option<String>,
    freedomscores: Option<RawFreedomScores>,
    publicsector: Option<f64>,
    deaths: Option<RawDeaths>,
    leader: Option<String>,
    capital: Option<String>,
    religion: Option<String>,
    factbooks: Option<u16>,
    dispatches: Option<u16>,
    dbid: Option<u32>,
    // END default
    admirable: Option<String>,
    admirables: Option<RawAdmirables>,
    animaltrait: Option<String>,
    banner: Option<String>,
    banners: Option<RawBanners>,
    census: Option<RawCensus>,
    crime: Option<String>,
    dispatchlist: Option<RawDispatchList>,
    factbooklist: Option<RawFactbookList>,
    foundedtime: Option<u64>,
    gavote: Option<String>,
    gdp: Option<u64>,
    govtdesc: Option<String>,
    happenings: Option<RawHappenings>,
    income: Option<u32>,
    industrydesc: Option<String>,
    legislation: Option<RawLegislation>,
    notable: Option<String>,
    notables: Option<RawNotables>,
    policies: Option<RawPolicies>,
    poorest: Option<u32>,
    rcensus: Option<NonZeroU16>,
    richest: Option<u32>,
    scvote: Option<String>,
    sectors: Option<RawSectors>,
    sensibilities: Option<String>,
    tgcanrecruit: Option<u8>,
    tgcancampaign: Option<u8>,
    wcensus: Option<NonZeroU32>,
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawStandardNation {
    name: String,
    #[serde(rename = "TYPE")]
    kind: String,
    fullname: String,
    motto: String,
    category: String,
    unstatus: String,
    endorsements: String,
    issues_answered: u32,
    freedom: RawFreedoms,
    region: String,
    population: u32,
    tax: f64,
    animal: String,
    currency: String,
    demonym: String,
    demonym2: String,
    demonym2plural: String,
    flag: String,
    majorindustry: String,
    govtpriority: String,
    govt: RawGovernment,
    founded: String,
    firstlogin: u64,
    lastlogin: u64,
    lastactivity: String,
    influence: String,
    freedomscores: RawFreedomScores,
    publicsector: f64,
    deaths: RawDeaths,
    leader: String,
    capital: String,
    religion: String,
    factbooks: u16,
    dispatches: u16,
    dbid: u32,
}

#[derive(Debug, Deserialize)]
struct RawDeaths {
    #[serde(rename = "CAUSE", default)]
    inner: Vec<RawCause>,
}

#[derive(Debug, Deserialize)]
struct RawAdmirables {
    #[serde(rename = "ADMIRABLE", default)]
    inner: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawBanners {
    #[serde(rename = "BANNER", default)]
    inner: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawDispatchList {
    #[serde(rename = "DISPATCH", default)]
    inner: Vec<RawDispatch>,
}

#[derive(Debug, Deserialize)]
struct RawFactbookList {
    #[serde(rename = "FACTBOOK", default)]
    inner: Vec<RawDispatch>, // only containing factbooks!
}

#[derive(Debug, Deserialize)]
struct RawLegislation {
    #[serde(rename = "LAW", default)]
    inner: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawNotables {
    #[serde(rename = "NOTABLE", default)]
    inner: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawPolicies {
    #[serde(rename = "POLICY", default)]
    inner: Vec<RawPolicy>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawPolicy {
    name: String,
    pic: String,
    cat: String,
    desc: String,
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

#[derive(Debug, Deserialize)]
struct RawCause {
    #[serde(rename = "@type")] // attribute: "type"
    kind: String,
    #[serde(rename = "$value")] // extract inner text
    frequency: f64,
}

impl From<RawCause> for Cause {
    fn from(value: RawCause) -> Self {
        let RawCause { kind, frequency } = value;
        Self { kind, frequency }
    }
}

#[derive(Debug, Deserialize)]
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

impl RawDispatch {
    fn dispatch_category(&self) -> Result<DispatchCategory, IntoNationError> {
        match self.category.as_str() {
            "Factbook" => Ok(DispatchCategory::Factbook(
                match self.subcategory.as_str() {
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
                    other => Err(IntoNationError::BadFieldError(
                        String::from("FactbookCategory"),
                        String::from(other),
                    )),
                }?,
            )),
            "Bulletin" => Ok(DispatchCategory::Bulletin(
                match self.subcategory.as_str() {
                    "Policy" => Ok(BulletinCategory::Policy),
                    "News" => Ok(BulletinCategory::News),
                    "Opinion" => Ok(BulletinCategory::Opinion),
                    "Campaign" => Ok(BulletinCategory::Campaign),
                    other => Err(IntoNationError::BadFieldError(
                        String::from("BulletinCategory"),
                        String::from(other),
                    )),
                }?,
            )),
            "Account" => Ok(DispatchCategory::Account(match self.subcategory.as_str() {
                "Military" => Ok(AccountCategory::Military),
                "Trade" => Ok(AccountCategory::Trade),
                "Sport" => Ok(AccountCategory::Sport),
                "Drama" => Ok(AccountCategory::Drama),
                "Diplomacy" => Ok(AccountCategory::Diplomacy),
                "Science" => Ok(AccountCategory::Science),
                "Culture" => Ok(AccountCategory::Culture),
                "Other" => Ok(AccountCategory::Other),
                other => Err(IntoNationError::BadFieldError(
                    String::from("AccountCategory"),
                    String::from(other),
                )),
            }?)),
            "Meta" => Ok(DispatchCategory::Meta(match self.subcategory.as_str() {
                "Gameplay" => Ok(MetaCategory::Gameplay),
                "Reference" => Ok(MetaCategory::Reference),
                other => Err(IntoNationError::BadFieldError(
                    String::from("MetaCategory"),
                    String::from(other),
                )),
            }?)),
            other => Err(IntoNationError::BadFieldError(
                String::from("DispatchCategory"),
                String::from(other),
            )),
        }
    }
}

impl TryFrom<RawDispatch> for Dispatch {
    type Error = IntoNationError;

    fn try_from(value: RawDispatch) -> Result<Self, Self::Error> {
        let category = value.dispatch_category()?;
        Ok(Dispatch {
            id: value.id,
            title: value.title,
            author: pretty_name(value.author),
            category,
            created: value.created,
            edited: NonZeroU64::try_from(value.edited).ok(), // field is 0 if never edited
            views: value.views,
            score: value.score,
        })
    }
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
struct RawFreedoms {
    #[serde(rename = "CIVILRIGHTS")]
    civil_rights: String,
    #[serde(rename = "ECONOMY")]
    economy: String,
    #[serde(rename = "POLITICALFREEDOM")]
    political_freedom: String,
}

impl TryFrom<RawFreedoms> for Freedoms {
    type Error = IntoNationError;
    fn try_from(value: RawFreedoms) -> Result<Self, Self::Error> {
        let RawFreedoms {
            civil_rights,
            economy,
            political_freedom,
        } = value;

        Ok(Self {
            civil_rights: civil_rights.try_into()?,
            economy: economy.try_into()?,
            political_freedom: political_freedom.try_into()?,
        })
    }
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
struct RawFreedomScores {
    #[serde(rename = "CIVILRIGHTS")]
    civil_rights: u8,
    #[serde(rename = "ECONOMY")]
    economy: u8,
    #[serde(rename = "POLITICALFREEDOM")]
    political_freedom: u8,
}

impl From<RawFreedomScores> for FreedomScores {
    fn from(value: RawFreedomScores) -> Self {
        let RawFreedomScores {
            civil_rights,
            economy,
            political_freedom,
        } = value;
        Self {
            civil_rights,
            economy,
            political_freedom,
        }
    }
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawGovernment {
    administration: f64,
    defence: f64,
    education: f64,
    environment: f64,
    healthcare: f64,
    commerce: f64,
    #[serde(rename = "INTERNATIONALAID")]
    international_aid: f64,
    #[serde(rename = "LAWANDORDER")]
    law_and_order: f64,
    #[serde(rename = "PUBLICTRANSPORT")]
    public_transport: f64,
    #[serde(rename = "SOCIALEQUALITY")]
    social_equality: f64,
    spirituality: f64,
    welfare: f64,
}

impl From<RawGovernment> for Government {
    fn from(value: RawGovernment) -> Self {
        let RawGovernment {
            administration,
            defence,
            education,
            environment,
            healthcare,
            commerce,
            international_aid,
            law_and_order,
            public_transport,
            social_equality,
            spirituality,
            welfare,
        } = value;
        Self {
            administration,
            defence,
            education,
            environment,
            healthcare,
            commerce,
            international_aid,
            law_and_order,
            public_transport,
            social_equality,
            spirituality,
            welfare,
        }
    }
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawSectors {
    #[serde(rename = "BLACKMARKET")]
    black_market: f64,
    government: f64,
    industry: f64,
    public: f64,
}

impl From<RawSectors> for Sectors {
    fn from(value: RawSectors) -> Self {
        let RawSectors {
            black_market,
            government,
            industry,
            public,
        } = value;
        Self {
            black_market,
            government,
            industry,
            public,
        }
    }
}

impl Nation {
    /// Converts the XML response from NationStates to a [`Nation`].
    pub fn from_xml(xml: &str) -> Result<Self, IntoNationError> {
        Self::try_from(quick_xml::de::from_str::<RawNation>(xml)?)
    }
}

impl TryFrom<RawNation> for Nation {
    type Error = IntoNationError;

    fn try_from(value: RawNation) -> Result<Self, Self::Error> {
        let name = match (value.name, value.id) {
            (Some(n), _) => Ok(n),
            (None, Some(i)) => Ok(pretty_name(i)),
            (None, None) => Err(IntoNationError::NoFieldError(String::from("name"))),
        }?;

        let happenings = value
            .happenings
            .map(|h| h.inner.into_iter().map(Event::from).collect());

        let wa_status = match value.unstatus {
            Some(s) => match s.as_str() {
                "WA Delegate" => Ok(Some(WAStatus::Delegate)),
                "WA Member" => Ok(Some(WAStatus::Member)),
                "Non-member" => Ok(Some(WAStatus::NonMember)),
                other => Err(IntoNationError::BadFieldError(
                    String::from("WAStatus"),
                    String::from(other),
                )),
            },
            None => Ok(None),
        }?;

        Ok(Self {
            name,
            kind: value.kind,
            full_name: value.fullname,
            motto: value.motto,
            category: value.category,
            wa_status,
            endorsements: value.endorsements.as_ref().map(|e| {
                (!e.is_empty())
                    .then(|| e.split(',').map(pretty_name).collect::<Vec<_>>())
                    .unwrap_or_default()
            }),
            issues_answered: value.issues_answered,
            freedom: value.freedom.map(Freedoms::try_from).transpose()?,
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
            government: value.govt.map(Government::from),
            founded: value.founded.map(MaybeRelativeTime::from),
            first_login: value.firstlogin,
            last_login: value.lastlogin,
            last_activity: value.lastactivity,
            influence: value.influence,
            freedom_scores: value.freedomscores.map(FreedomScores::from),
            public_sector: value.publicsector,
            deaths: value
                .deaths
                .map(|d| d.inner.into_iter().map(Cause::from).collect()),
            leader: value.leader.map(DefaultOrCustom::leader),
            capital: value.capital.map(DefaultOrCustom::capital),
            religion: value.religion.map(DefaultOrCustom::religion),
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
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            census: value
                .census
                .map(|c| match c.inner.first() {
                    Some(f) if f.timestamp.is_some() => Ok(CensusData::Historical(
                        c.inner
                            .into_iter()
                            .map(CensusHistoricalData::from)
                            .collect(),
                    )),
                    Some(_) => Ok(CensusData::Current(
                        c.inner.into_iter().map(CensusCurrentData::from).collect(),
                    )),
                    None => Err(IntoNationError::NoFieldError(String::from("census"))),
                })
                .transpose()?,
            crime: value.crime,
            dispatch_list: value
                .dispatchlist
                .map(|v| {
                    v.inner
                        .into_iter()
                        .map(Dispatch::try_from)
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            factbook_list: value
                .factbooklist
                .map(|v| {
                    v.inner
                        .into_iter()
                        .map(Dispatch::try_from)
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            founded_time: value.foundedtime.map(MaybeSystemTime::from),
            ga_vote: match wa_status {
                Some(WAStatus::NonMember) => None,
                _ => value.gavote.map(WAVote::try_from).transpose()?,
            },
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
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            poorest: value.poorest,
            regional_census: value.rcensus,
            richest: value.richest,
            sc_vote: match wa_status {
                Some(WAStatus::NonMember) => None,
                _ => value.scvote.map(WAVote::try_from).transpose()?,
            },
            sectors: value.sectors.map(Sectors::from),
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

impl StandardNation {
    /// Converts the XML response from NationStates to a [`Nation`].
    pub fn from_xml(xml: &str) -> Result<Self, IntoNationError> {
        Self::try_from(quick_xml::de::from_str::<RawStandardNation>(xml)?)
    }
}

impl TryFrom<RawStandardNation> for StandardNation {
    type Error = IntoNationError;

    fn try_from(value: RawStandardNation) -> Result<Self, Self::Error> {
        Ok(StandardNation {
            name: value.name,
            kind: value.kind,
            full_name: value.fullname,
            motto: value.motto,
            category: value.category,
            wa_status: match value.unstatus.as_str() {
                "WA Delegate" => Ok(WAStatus::Delegate),
                "WA Member" => Ok(WAStatus::Member),
                "Non-member" => Ok(WAStatus::NonMember),
                other => Err(IntoNationError::BadFieldError(
                    String::from("WAStatus"),
                    other.to_string(),
                )),
            }?,
            endorsements: (!value.endorsements.is_empty())
                .then(|| {
                    value
                        .endorsements
                        .split(',')
                        .map(pretty_name)
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default(),
            issues_answered: value.issues_answered,
            freedom: value.freedom.try_into()?,
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
            government: value.govt.into(),
            founded: value.founded.into(),
            first_login: value.firstlogin,
            last_login: value.lastlogin,
            last_activity: value.lastactivity,
            influence: value.influence,
            freedom_scores: value.freedomscores.into(),
            public_sector: value.publicsector,
            deaths: value.deaths.inner.into_iter().map(Cause::from).collect(),
            leader: DefaultOrCustom::leader(value.leader),
            capital: DefaultOrCustom::capital(value.capital),
            religion: DefaultOrCustom::religion(value.religion),
            factbooks: value.factbooks,
            dispatches: value.dispatches,
            dbid: value.dbid,
        })
    }
}
