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
use std::fmt::Debug;
use std::num::{NonZeroU32, NonZeroU64, NonZeroU8};
use strum::Display;
use thiserror::Error;

pub(crate) const BASE_URL: &str = "https://www.nationstates.net/cgi-bin/api.cgi?";

/// Type that maps extra parameters in the query to their values.
#[derive(Debug, Default)]
pub(crate) struct Params<'a>(HashMap<&'a str, String>);

impl<'a> Params<'a> {
    pub(crate) fn insert(&mut self, k: &'a str, v: String) {
        self.0.insert(k, v);
    }

    pub(crate) fn insert_scale(&mut self, scale: &CensusScales) -> &mut Self {
        if let Some(v) = match scale {
            CensusScales::One(scale) => Some(scale.to_string()),
            CensusScales::Many(scales) => Some(scales.iter().join("+")),
            CensusScales::All => Some("all".to_string()),
            CensusScales::Today => None,
        } {
            self.insert("scale", v)
        }
        self
    }

    pub(crate) fn insert_rank_scale(&mut self, scale: &Option<NonZeroU8>) -> &mut Self {
        self.insert_scale(&scale.map_or(CensusScales::Today, |x| CensusScales::One(x.get() - 1)))
    }

    pub(crate) fn insert_modes(&mut self, modes: &CensusModes) -> &mut Self {
        match modes {
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
        self
    }

    pub(crate) fn insert_start(&mut self, start: &Option<NonZeroU32>) -> &mut Self {
        if let Some(s) = start {
            if s.get() > 1 {
                self.insert("start", s.to_string());
            }
        }
        self
    }

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

#[derive(Clone, Debug)]
pub struct CensusShard<'a> {
    /// Specify the World Census scale(s) to list, using numerical IDs.
    /// For all scales, use [`CensusScales::All`].
    /// For Today's World Census Report, use [`CensusScales::Today`].
    pub scale: CensusScales<'a>,
    /// Specify what population the scale should be compared against.
    /// For the default behavior without any modes listed:
    /// ```
    /// use crustacean_states::shards::CensusModes;
    /// use crustacean_states::shards::CensusCurrentMode as CCM;
    /// let modes = CensusModes::Current(&[CCM::Score, CCM::Rank, CCM::PercentRank]);
    /// ```
    pub modes: CensusModes<'a>,
}

/// World census scales as numerical IDs.
/// The IDs can be found [here](https://forum.nationstates.net/viewtopic.php?f=15&t=159491)
/// or in the URL of [World Census](https://www.nationstates.net/page=list_nations?censusid=0)
/// pages.
/// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
#[derive(Clone, Debug, Default)]
pub enum CensusScales<'a> {
    #[default]
    Today,
    /// Only one scale.
    One(u8),
    /// Multiple scales.
    Many(&'a [u8]),
    /// All scales.
    All,
}

/// Either describes current or historical data.
#[derive(Clone, Debug)]
pub enum CensusModes<'a> {
    /// This is a special mode that cannot be combined with other modes,
    /// as only scores are available, not ranks.
    /// When requesting history, you can optionally specify a time window, using Unix epoch times.
    /// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
    History(CensusHistoryParams),
    /// Represents current data.
    Current(&'a [CensusCurrentMode]),
}

#[derive(Clone, Debug, Default)]
pub struct CensusHistoryParams {
    /// Beginning of the measurement.
    from: Option<NonZeroU64>,
    /// End of the measurement.
    to: Option<NonZeroU64>,
}

impl CensusHistoryParams {
    pub fn new(after: NonZeroU64, before: NonZeroU64) -> Self {
        Self::default().before(before).after(after).to_owned()
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

/// Information on how nations in the region rank according to the World Census.
#[derive(Clone, Debug, Default)]
pub struct CensusRanksShard {
    /// The World Census ranking to use. If `None`, returns the day's featured World Census ranking.
    scale: Option<NonZeroU8>,
    /// The rank at which to start listing (e.g. `Some(1000)` would start at the 1000th nation).
    start: Option<NonZeroU32>,
}

impl CensusRanksShard {
    pub fn new(scale: u8, start: NonZeroU32) -> Self {
        Self::default().scale(scale).start(start).to_owned()
    }

    pub fn scale(&mut self, x: u8) -> &mut Self {
        self.scale = NonZeroU8::try_from(x + 1).ok();
        self
    }

    pub fn start(&mut self, x: NonZeroU32) -> &mut Self {
        self.start = Some(x);
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::shards::{
        CensusCurrentMode, CensusHistoryParams, CensusModes, CensusScales, Params,
    };
    use std::num::{NonZeroU64, NonZeroU8};

    // test Params
    #[test]
    fn new_params() {
        assert!(Params::default().0.is_empty());
    }

    #[test]
    fn insert_param() {
        let mut params = Params::default();
        params.insert("this", String::from("that"));
        assert_eq!(params.0.get("this"), Some(&String::from("that")));
    }

    #[test]
    fn insert_one_scale() {
        let mut params = Params::default();
        params.insert_scale(&CensusScales::One(3));
        assert_eq!(params.0.get("scale"), Some(&3.to_string()));
    }

    #[test]
    fn insert_many_scales() {
        let mut params = Params::default();
        params.insert_scale(&CensusScales::Many(&[3, 4, 5]));
        assert_eq!(params.0.get("scale"), Some(&String::from("3+4+5")));
    }

    #[test]
    fn insert_all_scales() {
        let mut params = Params::default();
        params.insert_scale(&CensusScales::All);
        assert_eq!(params.0.get("scale"), Some(&String::from("all")));
    }

    #[test]
    fn insert_today_scale() {
        let mut params = Params::default();
        params.insert_scale(&CensusScales::Today);
        assert_eq!(params.0.get("scale"), None);
    }

    #[test]
    fn insert_rank_scale() {
        let mut params = Params::default();
        // note: we do a little trolling, Some(x) = actual ID and None = not using any IDs
        params.insert_rank_scale(&Some(NonZeroU8::new(10).unwrap()));
        assert_eq!(params.0.get("scale"), Some(&9.to_string()));
    }

    #[test]
    fn insert_mode_history_from_and_to() {
        let mut params = Params::default();
        params.insert_modes(&CensusModes::History(CensusHistoryParams::new(
            NonZeroU64::new(6900).unwrap(),
            NonZeroU64::new(42000).unwrap(),
        )));
        assert_eq!(params.0.get("mode"), Some(&String::from("history")));
        assert_eq!(params.0.get("from"), Some(&6900.to_string()));
        assert_eq!(params.0.get("to"), Some(&42000.to_string()));
    }

    #[test]
    fn insert_mode_current_one() {
        let mut params = Params::default();
        params.insert_modes(&CensusModes::Current(&[CensusCurrentMode::PercentRank]));
        assert_eq!(params.0.get("mode"), Some(&String::from("prank")));
    }
}
