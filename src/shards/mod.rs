//! A shard is a tiny request, composed of two parts: the query and the extra parameters.
//! You add multiple shards together in order to get the most efficient response.
//! Remember: 50 requests per 30 seconds is both a lot and very little at the same time!
//!
//! There are two very important restrictions for shards: first, you can only combine shards that are
//! - for the same nation, or
//! - for the same region, or
//! - for the same World Assembly council, or
//! - for the world.
//! Second, it is not possible to make two requests that use extra parameters with the same name.
//! Right now, `crustacean-states` allows for parameters to be overwritten.
//! In the future, it may be possible to create a series of requests that do not overlap.

pub mod nation;
pub mod region;
pub mod wa;
pub mod world;

use itertools::Itertools;
use reqwest::Url;
use std::collections::hash_map::Drain;
use std::collections::HashMap;

use either::Either;
use std::fmt::Debug;
use std::num::NonZeroU64;
use strum::Display;
use thiserror::Error;

pub(crate) const BASE_URL: &str = "https://www.nationstates.net/cgi-bin/api.cgi?";

/// Type that maps extra parameters in the query to their values.
#[derive(Debug, Default)]
pub(crate) struct Params<'a>(HashMap<&'a str, String>);

impl<'a> Params<'a> {
    #[doc(hidden)]
    pub(crate) fn insert(&mut self, k: &'a str, v: String) {
        self.0.insert(k, v);
    }

    #[doc(hidden)]
    pub(crate) fn insert_scale(&mut self, scale: &Option<CensusScales>) -> &mut Self {
        if let Some(ref s) = scale {
            self.insert("scale", {
                let p = match s {
                    CensusScales::One(scale) => scale.to_string(),
                    CensusScales::Many(scales) => scales.iter().join("+"),
                    CensusScales::All => "all".to_string(),
                };
                p
            });
        }
        self
    }
    #[doc(hidden)]
    pub(crate) fn insert_modes(&mut self, modes: &Option<CensusModes>) -> &mut Self {
        if let Some(ref m) = modes {
            match m {
                CensusModes::History(CensusHistoryParams { from, to }) => {
                    self.insert("mode", String::from("history"));
                    if let Some(x) = from {
                        self.insert("from", x.to_string());
                    }
                    if let Some(x) = to {
                        self.insert("to", x.to_string());
                    }
                }
                CensusModes::Current(current_modes) => {
                    self.insert("mode", current_modes.iter().join("+"));
                }
            }
        }
        self
    }
    #[doc(hidden)]
    pub(crate) fn insert_start(&mut self, start: &Option<u32>) -> &mut Self {
        if let Some(s) = start {
            self.insert("start", s.to_string());
        }
        self
    }

    #[doc(hidden)]
    pub(crate) fn drain(&mut self) -> Drain<'_, &'a str, String> {
        self.0.drain()
    }
}

#[derive(Debug, Error)]
pub enum RequestBuildError<'a> {
    #[error("Builder does not have {0}")]
    MissingParam(&'a str),
}

pub trait NSRequest {
    fn as_url(&self) -> Url;
}

pub struct CensusShard {
    pub scale: CensusScales,
    pub modes: Either<CensusCurrentMode, CensusHistoryParams>,
}

/// World census scales as numerical IDs.
/// The IDs can be found [here](https://forum.nationstates.net/viewtopic.php?f=15&t=159491)
/// or in the URL of [World Census](https://www.nationstates.net/page=list_nations?censusid=0)
/// pages.
/// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
#[derive(Clone, Debug)]
pub enum CensusScales {
    /// Only one scale.
    One(u8),
    /// Multiple scales.
    Many(Vec<u8>),
    /// All scales.
    All,
}

/// Either describes current or historical data.
#[derive(Clone, Debug)]
pub enum CensusModes {
    /// This is a special mode that cannot be combined with other modes,
    /// as only scores are available, not ranks.
    /// When requesting history, you can optionally specify a time window, using Unix epoch times.
    /// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
    History(CensusHistoryParams),
    /// Represents current data.
    Current(Vec<CensusCurrentMode>),
}

#[derive(Clone, Debug, Default)]
pub struct CensusHistoryParams {
    /// Beginning of the measurement.
    from: Option<NonZeroU64>,
    /// End of the measurement.
    to: Option<NonZeroU64>,
}

impl CensusHistoryParams {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn after(&mut self, timestamp: NonZeroU64) -> &mut Self {
        self.from = Some(timestamp);
        self
    }
    pub fn before(&mut self, timestamp: NonZeroU64) -> &mut Self {
        self.to = Some(timestamp);
        self
    }
}

//noinspection SpellCheckingInspection
/// Describes data that can currently be found on the World Census.
#[derive(Clone, Debug, Display)]
pub enum CensusCurrentMode {
    /// Raw value.
    Score,
    /// World rank (e.g. "334" means 334th in the world).
    Rank,
    /// Region rank.
    #[strum(serialize = "rrank")]
    RegionRank,
    /// World rank as a percentage (e.g. "15" means "Top 15%").
    #[strum(serialize = "prank")]
    PercentRank,
    /// Region rank as a percentage.
    #[strum(serialize = "prrank")]
    PercentRegionRank,
}
