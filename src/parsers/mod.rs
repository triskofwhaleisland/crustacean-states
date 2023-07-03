//! Contains the modules that parse responses from the NationStates API.
use crate::dispatch::DispatchCategory;
use serde::Deserialize;
use std::num::{NonZeroU32, NonZeroU64};

pub mod happenings;
pub mod nation;
mod raw_nation;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub(super) struct RawEvent {
    pub(super) timestamp: u64,
    pub(super) text: String,
}

/// A value that either comes from a default or was customized.
#[derive(Debug)]
pub enum DefaultOrCustom {
    /// The value is the default.
    Default(String),
    /// The value is custom.
    Custom(String),
}

/// A relative timestamp that may or may not have been recorded.
#[derive(Debug)]
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
        Option::<String>::from(value).unwrap_or_else(|| "0".to_string())
    }
}

/// An absolute Unix timestamp that may or may not have been recorded.
#[derive(Debug)]
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

/// World Census data about the nation. Either Current or Historical.
#[derive(Debug)]
pub enum CensusData {
    /// Current data.
    Current(Vec<CensusCurrentData>),
    /// Historical data.
    Historical(Vec<CensusHistoricalData>),
}

/// Current World Census data about the nation.
//noinspection SpellCheckingInspection
#[derive(Debug)]
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
//noinspection SpellCheckingInspection
#[derive(Debug)]
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
#[derive(Debug)]
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
