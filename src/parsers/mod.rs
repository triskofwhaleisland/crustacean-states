//! Contains the modules that parse responses from the NationStates API.

use crate::parsers::nation::NationName;
use crate::{
    models::dispatch::DispatchCategory,
    parsers::{
        happenings::{Event, Happenings},
        nation::NationParsingError,
        region::IntoRegionError,
    },
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;
use std::{
    fmt::Debug,
    num::{NonZeroI64, NonZeroU32, NonZeroU64},
    str::FromStr,
};
use thiserror::Error;

pub mod happenings;
pub mod nation;
mod raw_nation;
mod raw_region;
pub mod region;
mod reader;

pub(crate) const DEFAULT_LEADER: &str = "Leader";
pub(crate) const DEFAULT_RELIGION: &str = "a major religion";

pub type NumNations = u32;

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum ParsingError {
    #[error("{0:?}")]
    Nation(Box<NationParsingError>),
    #[error("{0:?}")]
    Region(Box<IntoRegionError>),
    // field, value
    #[error("{0}, {1}")]
    BadFieldError(&'static str, String),
    #[error("{0}")]
    NoFieldError(&'static str),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(super) struct RawEvent {
    pub(super) timestamp: u64,
    pub(super) text: String,
}

/// A value that either comes from a default or was customized.
#[derive(Clone, Debug)]
pub enum DefaultOrCustom {
    /// The value is the default.
    Default(String),
    /// The value is custom.
    Custom(String),
}

impl DefaultOrCustom {
    fn leader(l: String) -> Self {
        if l.is_empty() {
            DefaultOrCustom::Default(DEFAULT_LEADER.to_string())
        } else {
            DefaultOrCustom::Custom(l)
        }
    }
    fn capital(c: String) -> Self {
        if c.is_empty() {
            DefaultOrCustom::Default(format!("{} City", &c))
        } else {
            DefaultOrCustom::Custom(c)
        }
    }
    fn religion(r: String) -> Self {
        if r.is_empty() {
            DefaultOrCustom::Default(DEFAULT_RELIGION.to_string())
        } else {
            DefaultOrCustom::Custom(r)
        }
    }
}

pub(crate) fn into_datetime(t: i64) -> Option<DateTime<Utc>> {
    DateTime::from_timestamp(t, 0)
}

/// A relative timestamp that may or may not have been recorded.
#[derive(Clone, Debug)]
pub enum MaybeRelativeTime {
    /// A known time.
    Recorded(String),
    /// A prehistoric time.
    Antiquity,
}

impl From<String> for MaybeRelativeTime {
    fn from(value: String) -> Self {
        match value.as_str() {
            "0" => MaybeRelativeTime::Antiquity,
            _ => MaybeRelativeTime::Recorded(value),
        }
    }
}

impl From<Option<String>> for MaybeRelativeTime {
    fn from(value: Option<String>) -> Self {
        match value {
            Some(t) => MaybeRelativeTime::Recorded(t),
            None => MaybeRelativeTime::Antiquity,
        }
    }
}

impl From<MaybeRelativeTime> for Option<String> {
    fn from(value: MaybeRelativeTime) -> Self {
        match value {
            MaybeRelativeTime::Recorded(x) => Some(x),
            MaybeRelativeTime::Antiquity => None,
        }
    }
}

impl From<MaybeRelativeTime> for String {
    fn from(value: MaybeRelativeTime) -> Self {
        Option::<String>::from(value).unwrap_or_else(|| String::from("0"))
    }
}

/// An absolute Unix timestamp that may or may not have been recorded.
#[derive(Clone, Debug)]
pub enum MaybeSystemTime {
    /// A known time. Mirrors `Some(DateTime<Utc>)`.
    Recorded(DateTime<Utc>),
    /// A prehistoric time. Mirrors `None`.
    Antiquity,
}

impl From<Option<DateTime<Utc>>> for MaybeSystemTime {
    fn from(value: Option<DateTime<Utc>>) -> Self {
        match value {
            Some(dt) => MaybeSystemTime::Recorded(dt),
            None => MaybeSystemTime::Antiquity,
        }
    }
}

impl From<Option<NonZeroI64>> for MaybeSystemTime {
    fn from(value: Option<NonZeroI64>) -> Self {
        MaybeSystemTime::from(value.map(i64::from).map(into_datetime).unwrap())
    }
}

impl From<MaybeSystemTime> for Option<DateTime<Utc>> {
    fn from(value: MaybeSystemTime) -> Self {
        match value {
            MaybeSystemTime::Recorded(dt) => Some(dt),
            MaybeSystemTime::Antiquity => None,
        }
    }
}

impl From<MaybeSystemTime> for Option<NonZeroI64> {
    fn from(value: MaybeSystemTime) -> Self {
        Option::<DateTime<Utc>>::from(value)
            .as_ref()
            .map(DateTime::timestamp)
            .map(NonZeroI64::try_from)
            .transpose()
            .unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawCensus {
    #[serde(rename = "SCALE", default)]
    inner: Vec<RawCensusData>,
}

impl TryFrom<RawCensus> for CensusData {
    type Error = ParsingError;
    fn try_from(value: RawCensus) -> Result<Self, Self::Error> {
        match value.inner.first() {
            Some(f) if f.timestamp.is_some() => Ok(CensusData::Historical(
                value
                    .inner
                    .into_iter()
                    .map(CensusHistoricalData::from)
                    .collect(),
            )),
            Some(_) => Ok(CensusData::Current(
                value
                    .inner
                    .into_iter()
                    .map(CensusCurrentData::from)
                    .collect(),
            )),
            None => Err(ParsingError::NoFieldError("census")),
        }
    }
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
pub(crate) struct RawCensusData {
    #[serde(rename = "@id")]
    id: u8,
    #[serde(rename = "SCORE")]
    score: Option<String>,
    #[serde(rename = "RANK")]
    world_rank: Option<String>,
    #[serde(rename = "RRANK")]
    region_rank: Option<String>,
    #[serde(rename = "PRANK")]
    percent_world_rank: Option<String>,
    #[serde(rename = "PRRANK")]
    percent_region_rank: Option<String>,
    #[serde(rename = "TIMESTAMP")]
    timestamp: Option<NonZeroU64>,
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
            score: score.into(),
            world_rank: world_rank.into(),
            region_rank: region_rank.into(),
            percent_world_rank: percent_world_rank.into(),
            percent_region_rank: percent_region_rank.into(),
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
            score: score.into(),
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct RawCensusRanks {
    #[serde(rename = "@id")]
    scale: u8,
    #[serde(rename = "NATIONS")]
    nations: RawCensusRanksNations,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct RawCensusRanksNations {
    #[serde(rename = "NATION", default)]
    inner: Vec<RawCensusRanksNation>,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct RawCensusRanksNation {
    name: String,
    rank: u32,
    score: String,
}

#[derive(Debug, Deserialize)]
struct RawHappenings {
    #[serde(rename = "EVENT", default)]
    inner: Vec<RawEvent>,
}

impl From<RawHappenings> for Happenings {
    fn from(value: RawHappenings) -> Self {
        Happenings(value.inner.into_iter().map(Event::from).collect())
    }
}

/// World Census data about the nation. Either Current or Historical.
#[derive(Clone, Debug)]
pub enum CensusData {
    /// Current data.
    Current(Vec<CensusCurrentData>),
    /// Historical data.
    Historical(Vec<CensusHistoricalData>),
}

/// Current World Census data about the nation.
#[derive(Clone, Debug)]
pub struct CensusCurrentData {
    /// The ID used for the data point. For example,
    pub id: u8,
    /// The score of the nation on the Census scale.
    pub score: CensusDataPoint<u32>,
    /// The placement the nation holds in the world ranking.
    pub world_rank: CensusDataPoint<NonZeroU32>,
    /// The placement the nation holds in its region ranking.
    pub region_rank: CensusDataPoint<NonZeroU32>,
    /// Kind of like a percentile, but backwards:
    /// the nation is in the top x% of nations according to this category,
    /// with x being this field.
    /// Note that all percentiles are to the nearest whole except for <1%,
    /// which are to the nearest tenth.
    pub percent_world_rank: CensusDataPoint<Decimal>,
    /// Like `percent_world_rank`, but only for the nation's region ranking.
    pub percent_region_rank: CensusDataPoint<Decimal>,
}

#[derive(Clone, Debug)]
pub enum CensusDataPoint<T: Clone + Debug> {
    NotRequested,
    InvalidRequest,
    CannotParse(String),
    Valid(T),
}

impl<T> From<Option<String>> for CensusDataPoint<T>
where
    T: Clone + Debug + FromStr,
{
    fn from(value: Option<String>) -> Self {
        match value {
            Some(v) => match v.as_str() {
                "" => CensusDataPoint::InvalidRequest,
                other => match other.parse() {
                    Ok(good) => CensusDataPoint::Valid(good),
                    Err(_) => CensusDataPoint::CannotParse(v),
                },
            },
            None => CensusDataPoint::NotRequested,
        }
    }
}

impl<T: Clone + Debug> TryFrom<CensusDataPoint<T>> for Option<T> {
    type Error = ParsingError;

    fn try_from(value: CensusDataPoint<T>) -> Result<Self, Self::Error> {
        match value {
            CensusDataPoint::NotRequested => Ok(None),
            CensusDataPoint::Valid(v) => Ok(Some(v)),
            CensusDataPoint::CannotParse(s) => {
                Err(ParsingError::BadFieldError("CensusDataPoint", s))
            }
            CensusDataPoint::InvalidRequest => Err(ParsingError::BadFieldError(
                "CensusDataPoint",
                "request was invalid".to_string(),
            )),
        }
    }
}
/// Historical data from the World Census.
/// Note that only scores and not rankings are available this way.
#[derive(Clone, Debug)]
pub struct CensusHistoricalData {
    /// The ID used for the data point. For example,
    pub id: u8,
    /// When the nation was ranked.
    /// This usually corresponds to a time around the major
    /// (midnight Eastern Time) or minor (noon Eastern Time) game updates.
    pub timestamp: Option<NonZeroU64>,
    /// The score of the nation on the Census scale.
    pub score: CensusDataPoint<Decimal>,
}

/// Metadata about a dispatch.
#[derive(Clone, Debug)]
pub struct Dispatch {
    /// The numerical ID of the dispatch.
    /// This forms the URL: for example,
    /// <https://www.nationstates.net/page=dispatch/id=1> is the first dispatch ever created
    /// ("How to Write a Dispatch", Testlandia).
    pub id: u32,
    /// The title of the dispatch. This field can be edited.
    pub title: String,
    /// The nation that wrote the dispatch.
    pub author: NationName,
    /// The category and subcategory of the dispatch.
    pub category: DispatchCategory,
    /// The timestamp when the dispatch was created.
    pub created: u64,
    /// The timestamp when the dispatch was last edited.
    pub edited: Option<NonZeroU64>,
    /// The number of views the dispatch has.
    pub views: u32,
    /// The score of the dispatch
    pub score: u32,
}

#[derive(Clone, Debug)]
pub struct CensusRegionRanks {
    pub id: u8,
    pub nations: Vec<CensusCurrentData>,
}

impl TryFrom<RawCensusRanks> for CensusRegionRanks {
    type Error = IntoRegionError;
    fn try_from(value: RawCensusRanks) -> Result<Self, Self::Error> {
        Ok(Self {
            id: value.scale,
            nations: value
                .nations
                .inner
                .into_iter()
                .map(|nation| {
                    Ok(CensusCurrentData {
                        id: value.scale,
                        score: Some(nation.score).into(),
                        world_rank: CensusDataPoint::NotRequested,
                        region_rank: match nation.rank {
                            0 => CensusDataPoint::InvalidRequest,
                            n => CensusDataPoint::Valid(n.try_into().unwrap()),
                        },
                        percent_world_rank: CensusDataPoint::NotRequested,
                        percent_region_rank: CensusDataPoint::NotRequested,
                    })
                })
                .collect::<Result<Vec<CensusCurrentData>, Self::Error>>()?,
        })
    }
}
