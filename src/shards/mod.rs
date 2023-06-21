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

pub mod public_nation_shards;
pub mod region_shards;
pub mod world_assembly_shards;
pub mod world_shards;

use crate::safe_name;
use crate::shards::public_nation_shards::{CensusModes, CensusScales, PublicNationShard};
use crate::shards::region_shards::RegionShard;
use crate::shards::world_assembly_shards::{WACouncil, WAShard};
use crate::shards::world_shards::WorldShard;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};
use itertools::Itertools;

/// Type that maps extra parameters in the query to their values.
#[derive(Debug, Default)]
pub(crate) struct Params<'a>(HashMap<&'a str, String>);

impl<'a> Params<'a> {
    #[doc(hidden)]
    pub(crate) fn insert_scale(&mut self, scale: &Option<CensusScales>) -> &mut Self {
        if let Some(ref s) = scale {
            self.0.insert(
                "scale",
                {
                    let p = match s {
                        CensusScales::One(scale) => scale.to_string(),
                        CensusScales::Many(scales) => scales.iter().join("+"),
                        CensusScales::All => "all".to_string(),
                    };
                    p
                },
            );
        }
        self
    }
    #[doc(hidden)]
    pub(crate) fn insert_modes(&mut self, modes: &Option<CensusModes>) -> &mut Self {
        if let Some(ref m) = modes {
            match m {
                CensusModes::History { from, to } => {
                    self.0.insert("mode", String::from("history"));
                    if let Some(x) = from {
                        self.0.insert("from", x.to_string());
                    }
                    if let Some(x) = to {
                        self.0.insert("to", x.to_string());
                    }
                }
                CensusModes::Current(current_modes) => {
                    self.0.insert("mode", current_modes.iter().join("+"));
                }
            }
        }
        self
    }
    #[doc(hidden)]
    pub(crate) fn insert_start(&mut self, start: &Option<u32>, ) -> &mut Self {
        if let Some(s) = start {
            self.0.insert("start", s.to_string());
        }
        self
    }
}

#[derive(Debug)]
/// The smallest possible request that can be made to the website.
pub struct Shard<'a> {
    pub(crate) query: String,
    pub(crate) params: Params<'a>,
}

impl<'a> Shard<'a> {
    fn query_and_params<T: Into<Self> + Clone>(shards: &'a [T]) -> (String, Params) {
        let mut params = Params::default();
        let mut query = String::new();
        shards.iter().for_each(|s| {
            let shard: Shard = s.clone().into();
            if !query.is_empty() {
                query.push('+');
            }
            query.push_str(shard.query.to_lowercase().as_str());
            params.0.extend(shard.params.0);
        });
        (query, params)
    }

    fn name<T: Debug>(shard: &T) -> String {
        let true_debug = format!("{shard:?}");
        if let Some((tuple, _)) = true_debug.split_once('(') {
            tuple.to_string()
        } else if let Some((struct_like, _)) = true_debug.split_once(' ') {
            struct_like.to_string()
        } else {
            true_debug
        }
    }
}

/// The intermediate representation of a NationStates API request.
pub struct NSRequest<'a> {
    kind: NSRequestKind,
    query: String,
    params: Params<'a>,
}

/// The kind of request being made.
/// NOTE: as the API continues to expand, more categories of requests will be supported.
#[non_exhaustive]
pub enum NSRequestKind {
    /// A request about a nation (using public data).
    PublicNation(String),
    /// A request about a region.
    Region(String),
    /// A request about the world.
    World,
    /// A request about the World Assembly.
    WA {
        /// The council that the request is about.
        council: WACouncil,
        /// The ID of the resolution being inquired of.
        /// If left "None", the response will be for the current at-vote resolution.
        id: Option<u16>,
    },
}

impl<'a> NSRequest<'a> {
    /// Create a nation request with any number of [`PublicNationShard`]s.
    ///
    /// # Example
    ///
    /// ```
    /// use crustacean_states::shards::NSRequest;
    /// use crustacean_states::shards::public_nation_shards::PublicNationShard;
    ///
    /// let request = NSRequest::new_nation("Testlandia",
    ///         &[PublicNationShard::Region, PublicNationShard::Demonym]).to_string();
    /// ```
    /// When sent,
    /// it will request information about [Testlandia](https://www.nationstates.net/nation=testlandia)'s region and demonym.
    pub fn new_nation(nation: impl ToString, shards: &'a [PublicNationShard]) -> Self {
        let (query, params) = Shard::query_and_params(shards);
        Self {
            kind: NSRequestKind::PublicNation(nation.to_string()),
            query,
            params,
        }
    }
    /// Create a "standard" nation request.
    ///
    /// The following fields of [`Nation`] will not be `None`:
    ///
    /// `name`, `kind`, `full_name`, `motto`, `category`, `wa_status`, `endorsements`,
    /// `issues_answered`, `freedom`, `region`, `population`, `tax`, `animal`, `currency`,
    /// `demonym`, `demonym2`, `demonym2_plural`, `flag`, `major_industry`, `government_priority`,
    /// `government`, `founded`, `first_login`, `last_login`, `influence`, `freedom_scores`,
    /// `public_sector`, `deaths`, `factbooks`, `dispatches`, `dbid`
    ///
    ///
    /// The following fields will be filled
    /// only if the nation has reached a certain population and answered the relevant issue:
    /// - `capital`: 250 million
    ///
    /// - `kind` will deviate from the original pre-titles after 500 million.
    /// (No issue must be completed to unlock this ability.)
    ///
    /// - `leader`: 750 million
    ///
    /// - `religion`: 1 billion
    ///
    /// [`Nation`]: crate::parsers::nation::Nation
    pub fn new_nation_standard(nation: impl ToString) -> Self {
        Self {
            kind: NSRequestKind::PublicNation(nation.to_string()),
            query: Default::default(),
            params: Default::default(),
        }
    }

    /// Create a region request with any number of [`RegionShard`]s.
    ///
    /// # Example
    ///
    /// ```
    /// use crustacean_states::shards::NSRequest;
    /// use crustacean_states::shards::region_shards::RegionShard;
    ///
    /// let request = NSRequest::new_region("Testregionia",
    ///         &[RegionShard::Delegate, RegionShard::Flag]).to_string();
    /// ```
    ///
    /// When sent,
    /// it will request information about [Testregionia](https://www.nationstates.net/region=testregionia)'s delegate and flag.
    pub fn new_region(region: impl ToString, shards: &'a [RegionShard]) -> Self {
        let (query, params) = Shard::query_and_params(shards);
        Self {
            kind: NSRequestKind::Region(region.to_string()),
            query,
            params,
        }
    }
    /// Create a "standard" region request.
    pub fn new_region_standard(region: impl ToString) -> Self {
        Self {
            kind: NSRequestKind::Region(region.to_string()),
            query: Default::default(),
            params: Default::default(),
        }
    }

    /// Create a world request
    pub fn new_world(shards: &'a [WorldShard]) -> Self {
        let (query, params) = Shard::query_and_params(shards);
        Self {
            kind: NSRequestKind::World,
            query,
            params,
        }
    }

    /// Create a WA request
    pub fn new_wa(id: Option<u16>, shards: &'a [WAShard]) -> Self {
        let (query, params) = Shard::query_and_params(shards);
        Self {
            kind: NSRequestKind::WA {
                council: {
                    shards
                        .iter()
                        .find_map(|s| match s {
                            WAShard::Proposals(council)
                            | WAShard::CurrentResolution(council, _)
                            | WAShard::LastResolution(council) => Some(council.clone()),
                            _ => None,
                        })
                        .unwrap_or_default()
                },
                id,
            },
            query,
            params,
        }
    }
}

impl<'a> Display for NSRequest<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            match self.kind {
                NSRequestKind::PublicNation(ref n) => format!("nation={}", safe_name(n)),
                NSRequestKind::Region(ref r) => format!("region={}", safe_name(r)),
                NSRequestKind::World => String::new(),
                NSRequestKind::WA { ref council, id } => match id {
                    Some(i) => format!("wa={}&id={i}", council.clone() as u8),
                    None => format!("wa={}", council.clone() as u8),
                },
            },
            (!self.query.is_empty())
                .then(|| format!("&q={}", self.query))
                .unwrap_or_default(),
            (!self.params.0.is_empty())
                .then(|| self
                    .params.0
                    .iter()
                    .fold(String::new(), |acc, (k, v)| format!("{acc}&{k}={v}")))
                .unwrap_or_default(),
        )
    }
}
