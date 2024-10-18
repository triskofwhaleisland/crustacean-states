//! Contains the modules that parse responses from the NationStates API.

use crate::models::dispatch::DispatchCategory;
use crate::parsers::happenings::{Event, Happenings};
use crate::parsers::nation::IntoNationError;
use crate::parsers::region::IntoRegionError;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::num::{NonZeroI64, NonZeroU32, NonZeroU64};
use thiserror::Error;

pub mod happenings;
pub mod nation;
mod raw_nation;
mod raw_region;
pub mod region;

pub(crate) const DEFAULT_LEADER: &str = "Leader";
pub(crate) const DEFAULT_RELIGION: &str = "a major religion";

#[derive(Clone, Debug, Error)]
pub enum ParsingError {
    #[error("{0:?}")]
    Nation(Box<IntoNationError>),
    #[error("{0:?}")]
    Region(Box<IntoRegionError>),
    // field, value
    #[error("{0:?}")]
    BadFieldError(&'static str, String),
    #[error("{0:?}")]
    NoFieldError(&'static str),
}

// impl ParsingError {
//     /// Tread carefully: if this is not a BadFieldError, you will panic
//     fn bad_field_for_nation(self) -> IntoNationError {
//         match self {
//             ParsingError::BadFieldError(field, value) => {
//                 IntoNationError::BadFieldError(field, value)
//             }
//             _ => unreachable!(),
//         }
//     }
//     /// Tread carefully: if this is not a BadFieldError, you will panic
//     fn bad_field_for_region(self) -> IntoRegionError {
//         match self {
//             ParsingError::BadFieldError(field, value) => {
//                 IntoRegionError::BadFieldError(field, value)
//             }
//             _ => unreachable!(),
//         }
//     }
//
//     /// Tread carefully: if this is not a BadFieldError, you will panic
//     fn no_field_for_nation(self) -> IntoNationError {
//         match self {
//             ParsingError::NoFieldError(field) => IntoNationError::NoFieldError(field),
//             _ => unreachable!(),
//         }
//     }
//     /// Tread carefully: if this is not a BadFieldError, you will panic
//     fn no_field_for_region(self) -> IntoRegionError {
//         match self {
//             ParsingError::NoFieldError(field) => IntoRegionError::NoFieldError(field),
//             _ => unreachable!(),
//         }
//     }
// }

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
    pub score: Option<f64>,
    /// The placement the nation holds in the world ranking.
    pub world_rank: Option<NonZeroU32>,
    /// The placement the nation holds in its region ranking.
    pub region_rank: Option<NonZeroU32>,
    /// Kind of like a percentile, but backwards:
    /// the nation is in the top x% of nations according to this category,
    /// with x being this field.
    /// Note that all percentiles are to the nearest whole except for <1%,
    /// which are to the nearest tenth.
    pub percent_world_rank: Option<f64>,
    /// Like `percent_world_rank`, but only for the nation's region ranking.
    pub percent_region_rank: Option<f64>,
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
    pub score: Option<f64>,
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
    pub author: String,
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
                        score: Some(str::parse::<f64>(&*nation.score).map_err(|e| {
                            IntoRegionError::BadFieldError("CensusRegionRanks", e.to_string())
                        }))
                        .transpose()?,
                        world_rank: None,
                        region_rank: nation.rank.try_into().ok(),
                        percent_world_rank: None,
                        percent_region_rank: None,
                    })
                })
                .collect::<Result<Vec<CensusCurrentData>, Self::Error>>()?,
        })
    }
}
