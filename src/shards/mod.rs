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
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::num::{NonZeroU32, NonZeroU64, NonZeroU8};
use strum::Display;
use thiserror::Error;

pub(crate) const BASE_URL: &str = "https://www.nationstates.net/cgi-bin/api.cgi?";

/// Type that maps extra parameters in the query to their values.
/// The HashMap is from parameter keys to values.
/// The Vec is the order of keys.
#[derive(Debug, Default)]
pub(crate) struct Params<'a>(HashMap<&'a str, String>, Vec<&'a str>);

impl<'a> Params<'a> {
    pub(crate) fn insert_on<T>(&mut self, k: &'a str, v: &Option<T>) -> &mut Self
        where
            T: ToString,
    {
        if let Some(s) = v {
            self.0.insert(k, s.to_string());
            self.1.push(k);
        }
        self
    }
    pub(crate) fn insert<T>(&mut self, k: &'a str, v: T) -> &mut Self
        where
            T: ToString,
    {
        Self::insert_on(self, k, &Some(v))
    }

    pub(crate) fn insert_front<T>(&mut self, k: &'a str, v: T) -> &mut Self where
    T: ToString, {
        self.0.insert(k, v.to_string());
        self.1.insert(0, k);
        self
    }

    pub(crate) fn insert_scale(&mut self, scale: &CensusScales) -> &mut Self {
        self.insert_on(
            "scale",
            &match scale {
                CensusScales::One(scale) => Some(scale.to_string()),
                CensusScales::Many(scales) => Some(scales.iter().join("+")),
                CensusScales::All => Some(String::from("all")),
                CensusScales::Today => None,
            },
        )
    }

    pub(crate) fn insert_rank_scale(&mut self, scale: &Option<NonZeroU8>) -> &mut Self {
        self.insert_scale(&scale.map_or(CensusScales::Today, |x| CensusScales::One(x.get() - 1)))
    }

    pub(crate) fn insert_modes(&mut self, modes: &CensusModes) -> &mut Self {
        self.insert_on(
            "mode",
            &match modes {
                CensusModes::History(..) => Some(String::from("history")),
                CensusModes::Current(current_modes) => Some(current_modes.0.iter().join("+")),
            },
        );
        if let CensusModes::History(CensusHistoryParams { from, to }) = modes {
            self.insert_on("from", from).insert_on("to", to);
        }
        self
    }

    pub(crate) fn insert_start(&mut self, start: &Option<NonZeroU32>) -> &mut Self {
        if let Some(s) = start {
            if s.get() > 1 {
                self.insert("start", s);
            }
        }
        self
    }
}

impl<'a> Iterator for Params<'a> {
    type Item = (&'a str, String);

    fn next(&mut self) -> Option<Self::Item> {
        if !self.1.is_empty() {
            Some(self.0.remove_entry(self.1.remove(0)).unwrap())
        } else {
            None
        }
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

#[derive(Clone, Debug, Default)]
pub struct CensusShard<'a> {
    scale: CensusScales<'a>,
    modes: CensusModes,
}

impl<'a> CensusShard<'a> {
    pub fn new(scale: CensusScales<'a>, modes: CensusModes) -> CensusShard<'a> {
        CensusShard { scale, modes }
    }

    /// Specify the World Census scale(s) to list, using numerical IDs.
    /// For all scales, use [`CensusScales::All`].
    /// For Today's World Census Report, use [`CensusScales::Today`].
    pub fn scale(&mut self, scale: CensusScales<'a>) -> &mut Self {
        self.scale = scale;
        self
    }

    /// Specify what population the scale should be compared against.
    /// For the default behavior without any modes listed:
    /// ```
    /// use std::collections::HashSet;
    /// use crustacean_states::shards::CensusModes;
    /// use crustacean_states::shards::CensusCurrentMode as CCM;
    /// let modes = CensusModes::from(&[CCM::Score, CCM::Rank, CCM::PercentRank]);
    /// ```
    pub fn modes(&mut self, modes: CensusModes) -> &mut Self {
        self.modes = modes;
        self
    }
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
pub enum CensusModes {
    /// This is a special mode that cannot be combined with other modes,
    /// as only scores are available, not ranks.
    /// When requesting history, you can optionally specify a time window, using Unix epoch times.
    /// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
    History(CensusHistoryParams),
    /// Represents current data.
    Current(CensusCurrentModes),
}

impl Default for CensusModes {
    fn default() -> Self {
        Self::Current(CensusCurrentModes::new([
            CensusCurrentMode::Score,
            CensusCurrentMode::Rank,
            CensusCurrentMode::RegionRank,
        ]))
    }
}

impl<T> From<T> for CensusModes
where
    T: AsRef<[CensusCurrentMode]>,
{
    fn from(value: T) -> Self {
        Self::Current(CensusCurrentModes::new(value.as_ref().iter().cloned()))
    }
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
#[derive(Clone, Debug, Display, Ord, PartialOrd, Eq, PartialEq, Hash)]
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

#[derive(Clone, Debug, Default)]
pub struct CensusCurrentModes(HashSet<CensusCurrentMode>);

impl CensusCurrentModes {
    pub fn new<I>(modes: I) -> CensusCurrentModes
    where
        I: IntoIterator<Item = CensusCurrentMode>,
    {
        CensusCurrentModes(HashSet::from_iter(modes))
    }
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
        CensusCurrentMode, CensusCurrentModes, CensusHistoryParams, CensusModes, CensusScales,
        Params,
    };
    use std::num::{NonZeroU64, NonZeroU8};

    // test Params
    #[test]
    fn new_params() {
        assert!(Params::default().0.is_empty());
    }

    #[test]
    fn insert_param() {
        assert_eq!(
            Params::default()
                .insert("this", "that")
                .0
                .get("this"),
            Some(&String::from("that"))
        );
    }

    #[test]
    fn insert_one_scale() {
        assert_eq!(
            Params::default()
                .insert_scale(&CensusScales::One(3))
                .0
                .get("scale"),
            Some(&3.to_string())
        );
    }

    #[test]
    fn insert_many_scales() {
        assert_eq!(
            Params::default()
                .insert_scale(&CensusScales::Many(&[3, 4, 5]))
                .0
                .get("scale"),
            Some(&String::from("3+4+5"))
        );
    }

    #[test]
    fn insert_all_scales() {
        assert_eq!(
            Params::default()
                .insert_scale(&CensusScales::All)
                .0
                .get("scale"),
            Some(&String::from("all"))
        );
    }

    #[test]
    fn insert_today_scale() {
        assert_eq!(
            Params::default()
                .insert_scale(&CensusScales::Today)
                .0
                .get("scale"),
            None
        );
    }

    #[test]
    fn insert_rank_scale() {
        // note: we do a little trolling, Some(x) = actual ID and None = not using any IDs
        assert_eq!(
            Params::default()
                .insert_rank_scale(&Some(NonZeroU8::new(10).unwrap()))
                .0
                .get("scale"),
            Some(&9.to_string())
        );
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
        assert_eq!(
            Params::default()
                .insert_modes(&CensusModes::Current(CensusCurrentModes::new([
                    CensusCurrentMode::PercentRank
                ])))
                .0
                .get("mode"),
            Some(&String::from("prank"))
        );
    }

    #[test]
    fn param_iter_easy() {
        assert_eq!(
            Params::default()
                .insert("this", "that")
                .next(),
            Some(("this", String::from("that")))
        );
    }

    #[test]
    fn param_iter_complex() {
        let mut params = Params::default();
        params.insert("this", "that").insert("thing1", "thing2").insert("wow", "yikes");
        assert_eq!(
            params.next(),
            Some(("this", String::from("that")))
        );
        assert_eq!(
            params.next(),
            Some(("thing1", String::from("thing2")))
        );
        assert_eq!(
            params.next(),
            Some(("wow", String::from("yikes")))
        );
        assert_eq!(params.next(), None);
    }
}
