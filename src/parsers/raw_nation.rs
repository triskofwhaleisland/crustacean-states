use crate::parsers::nation::GovernmentCategory;
use crate::parsers::region::RegionName;
use crate::{
    models::dispatch::{
        AccountCategory, BulletinCategory, DispatchCategory, FactbookCategory, MetaCategory,
    },
    parsers::{
        happenings::Event,
        into_datetime,
        nation::{
            BannerId, Cause, Endorsements, FreedomScores, Freedoms, Government, IntoNationError,
            Nation, NationName, Policy, Sectors, StandardNation, WAStatus, WAVote,
        },
        CensusData, DefaultOrCustom, Dispatch, MaybeRelativeTime, MaybeSystemTime, RawCensus,
        RawHappenings,
    },
};
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};

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
    firstlogin: Option<i64>,
    lastlogin: Option<i64>,
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
    foundedtime: Option<i64>,
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
    firstlogin: i64,
    lastlogin: i64,
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

impl TryFrom<RawBanners> for Vec<BannerId> {
    type Error = IntoNationError;
    fn try_from(value: RawBanners) -> Result<Self, Self::Error> {
        value
            .inner
            .into_iter()
            .map(BannerId::try_from)
            .collect::<Result<Vec<_>, _>>()
    }
}

#[derive(Debug, Deserialize)]
struct RawDispatchList {
    #[serde(rename = "DISPATCH", default)]
    inner: Vec<RawDispatch>,
}

impl TryFrom<RawDispatchList> for Vec<Dispatch> {
    type Error = IntoNationError;

    fn try_from(value: RawDispatchList) -> Result<Self, Self::Error> {
        value
            .inner
            .into_iter()
            .map(Dispatch::try_from)
            .collect::<Result<Vec<_>, _>>()
    }
}

#[derive(Debug, Deserialize)]
struct RawFactbookList {
    #[serde(rename = "FACTBOOK", default)]
    inner: Vec<RawDispatch>, // only containing factbooks!
}

impl TryFrom<RawFactbookList> for Vec<Dispatch> {
    type Error = IntoNationError;

    fn try_from(value: RawFactbookList) -> Result<Self, Self::Error> {
        value
            .inner
            .into_iter()
            .map(Dispatch::try_from)
            .collect::<Result<Vec<_>, _>>()
    }
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

impl TryFrom<RawPolicies> for Vec<Policy> {
    type Error = IntoNationError;

    fn try_from(value: RawPolicies) -> Result<Self, Self::Error> {
        value
            .inner
            .into_iter()
            .map(Policy::try_from)
            .collect::<Result<Vec<_>, _>>()
    }
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
                    _ => Err(IntoNationError::BadFieldError(
                        "FactbookCategory",
                        self.subcategory.clone(),
                    )),
                }?,
            )),
            "Bulletin" => Ok(DispatchCategory::Bulletin(
                match self.subcategory.as_str() {
                    "Policy" => Ok(BulletinCategory::Policy),
                    "News" => Ok(BulletinCategory::News),
                    "Opinion" => Ok(BulletinCategory::Opinion),
                    "Campaign" => Ok(BulletinCategory::Campaign),
                    _ => Err(IntoNationError::BadFieldError(
                        "BulletinCategory",
                        self.subcategory.clone(),
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
                _ => Err(IntoNationError::BadFieldError(
                    "AccountCategory",
                    self.subcategory.clone(),
                )),
            }?)),
            "Meta" => Ok(DispatchCategory::Meta(match self.subcategory.as_str() {
                "Gameplay" => Ok(MetaCategory::Gameplay),
                "Reference" => Ok(MetaCategory::Reference),
                _ => Err(IntoNationError::BadFieldError(
                    "MetaCategory",
                    self.subcategory.clone(),
                )),
            }?)),
            _ => Err(IntoNationError::BadFieldError(
                "DispatchCategory",
                self.category.clone(),
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
            author: value.author,
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

fn into_datetime_or_bad_field(
    t: i64,
    field: &'static str,
) -> Result<DateTime<Utc>, IntoNationError> {
    into_datetime(t).ok_or(IntoNationError::BadFieldError(field, t.to_string()))
}

fn try_into_bool(x: u8) -> Result<bool, IntoNationError> {
    match x {
        0 => Ok(false),
        1 => Ok(true),
        e => Err(IntoNationError::BadBooleanError(e)),
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
        // let name = match (value.name, value.id) {
        //     (Some(n), _) => Ok(n),
        //     (None, Some(i)) => Ok(pretty_name(i)),
        //     (None, None) => Err(IntoNationError::NoFieldError(String::from("name"))),
        // }?;

        let happenings = value
            .happenings
            .map(|h| h.inner.into_iter().map(Event::from).collect());

        let wa_status = value.unstatus.map(WAStatus::try_from).transpose()?;

        Ok(Self {
            raw_name: NationName(value.id.unwrap_or_else(|| value.name.clone().unwrap())),
            nice_name: value.name,
            kind: value.kind,
            full_name: value.fullname,
            motto: value.motto,
            category: value
                .category
                .map(GovernmentCategory::try_from)
                .transpose()?,
            wa_status,
            endorsements: value.endorsements.map(Endorsements::from),
            issues_answered: value.issues_answered,
            freedom: value.freedom.map(Freedoms::try_from).transpose()?,
            region: value.region.map(RegionName),
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
            first_login: value
                .firstlogin
                .map(|t| into_datetime_or_bad_field(t, "Nation.first_login"))
                .transpose()?,
            last_login: value
                .lastlogin
                .map(|t| into_datetime_or_bad_field(t, "Nation.last_login"))
                .transpose()?,
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
            banners: value.banners.map(Vec::<BannerId>::try_from).transpose()?,
            census: value
                .census
                .map(CensusData::try_from)
                .transpose()
                .map_err(IntoNationError::from)?,
            crime: value.crime,
            dispatch_list: value
                .dispatchlist
                .map(RawDispatchList::try_into)
                .transpose()?,
            factbook_list: value
                .factbooklist
                .map(RawFactbookList::try_into)
                .transpose()?,
            founded_time: value
                .foundedtime
                .map(into_datetime)
                .map(MaybeSystemTime::from),
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
            policies: value.policies.map(Vec::<Policy>::try_from).transpose()?,
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
            tg_can_recruit: value.tgcanrecruit.map(try_into_bool).transpose()?,
            tg_can_campaign: value.tgcancampaign.map(try_into_bool).transpose()?,
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
            name: NationName(value.name),
            kind: value.kind,
            full_name: value.fullname,
            motto: value.motto,
            category: value.category.try_into()?,
            wa_status: value.unstatus.try_into()?,
            endorsements: Endorsements::from(value.endorsements),
            issues_answered: value.issues_answered,
            freedom: value.freedom.try_into()?,
            region: RegionName(value.region),
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
            first_login: into_datetime_or_bad_field(
                value.firstlogin,
                "StandardNation.first_login",
            )?,
            last_login: into_datetime_or_bad_field(value.lastlogin, "StandardNation.last_login")?,
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
