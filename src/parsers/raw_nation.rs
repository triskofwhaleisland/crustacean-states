use std::collections::HashMap;
use std::hash::Hash;
use crate::{
    models::dispatch::{
        AccountCategory, BulletinCategory, DispatchCategory, FactbookCategory, MetaCategory,
    },
    parsers::{
        happenings::Event,
        into_datetime,
        nation::{
            BannerId, Cause, CivilRights, Economy, FreedomScore, FreedomScores, Freedoms,
            Government, Nation, NationName, NationParsingError, Policy,
            PoliticalFreedoms, Sectors, StandardNation, WAStatus, WAVote,
        },
        region::RegionName
        , DefaultOrCustom, Dispatch, MaybeRelativeTime, MaybeSystemTime, RawCensus,
        RawHappenings,
    },
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize};
use std::num::{NonZeroU16, NonZeroU32, NonZeroU64};
use std::str::FromStr;
use enum_kinds::EnumKind;

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
    #[serde(with = "rust_decimal::serde::str_option")]
    tax: Option<Decimal>,
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
    #[serde(with = "rust_decimal::serde::str_option")]
    publicsector: Option<Decimal>,
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
    tgcanrecruit: Option<bool>,
    tgcancampaign: Option<bool>,
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
    #[serde(with = "rust_decimal::serde::str")]
    tax: Decimal,
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
    #[serde(with = "rust_decimal::serde::str")]
    publicsector: Decimal,
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
    type Error = NationParsingError;
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
    type Error = NationParsingError;

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
    type Error = NationParsingError;

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
    type Error = NationParsingError;

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
    type Error = NationParsingError;

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
    frequency: Decimal,
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
    fn dispatch_category(&self) -> Result<DispatchCategory, NationParsingError> {
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
                    _ => Err(NationParsingError::BadFieldError(
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
                    _ => Err(NationParsingError::BadFieldError(
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
                _ => Err(NationParsingError::BadFieldError(
                    "AccountCategory",
                    self.subcategory.clone(),
                )),
            }?)),
            "Meta" => Ok(DispatchCategory::Meta(match self.subcategory.as_str() {
                "Gameplay" => Ok(MetaCategory::Gameplay),
                "Reference" => Ok(MetaCategory::Reference),
                _ => Err(NationParsingError::BadFieldError(
                    "MetaCategory",
                    self.subcategory.clone(),
                )),
            }?)),
            _ => Err(NationParsingError::BadFieldError(
                "DispatchCategory",
                self.category.clone(),
            )),
        }
    }
}

impl TryFrom<RawDispatch> for Dispatch {
    type Error = NationParsingError;

    fn try_from(value: RawDispatch) -> Result<Self, Self::Error> {
        let category = value.dispatch_category()?;
        Ok(Dispatch {
            id: value.id,
            title: value.title,
            author: NationName(value.author),
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
    type Error = NationParsingError;
    fn try_from(value: RawFreedoms) -> Result<Self, Self::Error> {
        let RawFreedoms {
            civil_rights,
            economy,
            political_freedom,
        } = value;

        Ok(Self {
            civil_rights: CivilRights::from_str(&civil_rights)
                .map_err(NationParsingError::from_parse_error)?,
            economy: Economy::from_str(&economy).map_err(NationParsingError::from_parse_error)?,
            political_freedom: PoliticalFreedoms::from_str(&political_freedom)
                .map_err(NationParsingError::from_parse_error)?,
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
            civil_rights: FreedomScore::new(civil_rights),
            economy: FreedomScore::new(economy),
            political_freedom: FreedomScore::new(political_freedom),
        }
    }
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawGovernment {
    #[serde(with = "rust_decimal::serde::str")]
    administration: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    defence: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    education: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    environment: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    healthcare: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    commerce: Decimal,
    #[serde(rename = "INTERNATIONALAID", with = "rust_decimal::serde::str")]
    international_aid: Decimal,
    #[serde(rename = "LAWANDORDER", with = "rust_decimal::serde::str")]
    law_and_order: Decimal,
    #[serde(rename = "PUBLICTRANSPORT", with = "rust_decimal::serde::str")]
    public_transport: Decimal,
    #[serde(rename = "SOCIALEQUALITY", with = "rust_decimal::serde::str")]
    social_equality: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    spirituality: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    welfare: Decimal,
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
    #[serde(rename = "BLACKMARKET", with = "rust_decimal::serde::str")]
    black_market: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    government: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    industry: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    public: Decimal,
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
) -> Result<DateTime<Utc>, NationParsingError> {
    into_datetime(t).ok_or(NationParsingError::BadFieldError(field, t.to_string()))
}

fn try_into_bool(x: u8) -> Result<bool, NationParsingError> {
    match x {
        0 => Ok(false),
        1 => Ok(true),
        e => Err(NationParsingError::BadBooleanError(e)),
    }
}

pub(super) fn into_nation_list(list: String) -> Vec<NationName> {
    let delimiter = if list.contains(',') { ',' } else { ':' };
    if list.is_empty() {
        vec![]
    } else {
        list.split(delimiter)
            .filter(|x| !x.is_empty())
            .map(String::from)
            .map(NationName)
            .collect()
    }
}

impl Nation {
    /// Converts the XML response from NationStates to a [`Nation`].
    pub fn from_xml(xml: &str) -> Result<Self, NationParsingError> {
        Self::try_from(quick_xml::de::from_str::<RawNation>(xml)?)
    }
}

impl TryFrom<RawNation> for Nation {
    type Error = NationParsingError;

    fn try_from(value: RawNation) -> Result<Self, Self::Error> {
        // let name = match (value.name, value.id) {
        //     (Some(n), _) => Ok(n),
        //     (None, Some(i)) => Ok(pretty_name(i)),
        //     (None, None) => Err(IntoNationError::NoFieldError(String::from("name"))),
        // }?;

        let happenings = value
            .happenings
            .map(|h| h.inner.into_iter().map(Event::from).collect());

        let wa_status = value.unstatus.map(WAStatus::try_from);

        Ok(Self {
            raw_name: NationName(value.id.unwrap_or_else(|| value.name.clone().unwrap())),
            nice_name: value.name,
            kind: value.kind,
            full_name: value.fullname,
            motto: value.motto,
            category: value
                .category
                .as_deref() // Option<String> -> Option<&str>
                // through strum::EnumString
                .map(|c| c.try_into().map_err(NationParsingError::from_parse_error)),
            wa_status: wa_status.clone(),
            endorsements: value.endorsements.map(into_nation_list),
            issues_answered: value.issues_answered,
            freedom: value.freedom.map(Freedoms::try_from),
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
                .map(|t| into_datetime_or_bad_field(t, "Nation.first_login")),
            last_login: value
                .lastlogin
                .map(|t| into_datetime_or_bad_field(t, "Nation.last_login")),
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
            banner: value.banner.map(BannerId::try_from),
            banners: value.banners.map(Vec::<BannerId>::try_from),
            census: value
                .census
                .map(|c| c.try_into().map_err(NationParsingError::from)),
            crime: value.crime,
            dispatch_list: value.dispatchlist.map(RawDispatchList::try_into),
            factbook_list: value.factbooklist.map(RawFactbookList::try_into),
            founded_time: value
                .foundedtime
                .map(into_datetime)
                .map(MaybeSystemTime::from),
            ga_vote: match wa_status {
                Some(Ok(WAStatus::NonMember)) => None,
                _ => value.gavote.map(WAVote::try_from),
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
            policies: value.policies.map(Vec::<Policy>::try_from),
            poorest: value.poorest,
            regional_census: value.rcensus,
            richest: value.richest,
            sc_vote: match wa_status {
                Some(Ok(WAStatus::NonMember)) => None,
                _ => value.scvote.map(WAVote::try_from),
            },
            sectors: value.sectors.map(Sectors::from),
            sensibilities: value.sensibilities,
            // .map(|s| {
            //     let v = s.split(", ").collect::<Vec<_>>();
            //     [v[0].to_string(), v[1].to_string()]
            // })
            tg_can_recruit: value.tgcanrecruit,
            tg_can_campaign: value.tgcancampaign,
            world_census: value.wcensus,
        })
    }
}

impl StandardNation {
    /// Converts the XML response from NationStates to a [`Nation`].
    pub fn from_xml(xml: &str) -> Result<Self, NationParsingError> {
        Self::try_from(quick_xml::de::from_str::<RawStandardNation>(xml)?)
    }
}

impl TryFrom<RawStandardNation> for StandardNation {
    type Error = NationParsingError;

    fn try_from(value: RawStandardNation) -> Result<Self, Self::Error> {
        Ok(StandardNation {
            name: NationName(value.name),
            kind: value.kind,
            full_name: value.fullname,
            motto: value.motto,
            category: value.category.as_str().try_into()?,
            wa_status: value.unstatus.try_into()?,
            endorsements: into_nation_list(value.endorsements),
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
