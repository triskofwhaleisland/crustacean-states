//! Contains the modules that parse responses from the NationStates API.

use crate::models::dispatch::DispatchCategory;
use crate::parsers::nation::IntoNationError;
use serde::Deserialize;
use std::num::{NonZeroU32, NonZeroU64};

pub mod happenings;
pub mod nation;
mod raw_nation;
mod raw_region;
pub mod region;

pub(crate) const DEFAULT_LEADER: &str = "Leader";
pub(crate) const DEFAULT_RELIGION: &str = "a major religion";

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
    /// A known time.
    Recorded(NonZeroU64),
    /// A prehistoric time.
    Antiquity,
}

impl From<u64> for MaybeSystemTime {
    fn from(value: u64) -> Self {
        NonZeroU64::try_from(value)
            .map(MaybeSystemTime::Recorded)
            .unwrap_or_else(|_| MaybeSystemTime::Antiquity)
    }
}

impl From<Option<NonZeroU64>> for MaybeSystemTime {
    fn from(value: Option<NonZeroU64>) -> Self {
        match value {
            Some(x) => MaybeSystemTime::Recorded(x),
            None => MaybeSystemTime::Antiquity,
        }
    }
}

impl From<MaybeSystemTime> for Option<NonZeroU64> {
    fn from(value: MaybeSystemTime) -> Self {
        match value {
            MaybeSystemTime::Recorded(x) => Some(x),
            MaybeSystemTime::Antiquity => None,
        }
    }
}

impl From<MaybeSystemTime> for u64 {
    fn from(value: MaybeSystemTime) -> Self {
        Option::<NonZeroU64>::from(value)
            .map(u64::from)
            .unwrap_or_default()
    }
}

#[derive(Debug, Deserialize)]
pub(crate) struct RawCensus {
    #[serde(rename = "SCALE", default)]
    inner: Vec<RawCensusData>,
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

pub struct CensusRegionRanks {
    pub id: u8,
    pub nations: [CensusCurrentData; 20],
}

impl TryFrom<RawCensusRanks> for CensusRegionRanks {
    type Error = IntoNationError;
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
                            IntoNationError::BadFieldError(
                                String::from("CensusRegionRanks"),
                                e.to_string(),
                            )
                        }))
                        .transpose()?,
                        world_rank: None,
                        region_rank: nation.rank.try_into().ok(),
                        percent_world_rank: None,
                        percent_region_rank: None,
                    })
                })
                .collect::<Result<Vec<CensusCurrentData>, Self::Error>>()?
                .try_into()
                .map_err(|_| {
                    IntoNationError::WrongLengthError(String::from("CensusRegionRanks"), 20)
                })?,
        })
    }
}
