//! Contains everything needed to make region shard requests.

use crate::shards::public_nation::{CensusModes, CensusScales};
use crate::shards::{Params, Shard};

/// A request of a region.
#[derive(Clone, Debug)]
pub enum RegionShard {
    /// The list of all nations banned from the region.
    BanList,
    /// The region's banner's ID.
    Banner,
    /// The nation who uploaded the region's banner.
    BannerBy,
    /// The URL for the banner.
    BannerUrl,
    /// By default, returns the score, rank, and region rank on today's featured World Census scale.
    /// Can be optionally configured with additional parameters.
    /// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
    ///
    /// Parallels [`PublicNationShard::Census`].
    ///
    /// [`PublicNationShard`]: crate::shards::public_nation::PublicNationShard
    Census {
        /// Specify the World Census scale(s) to list, using numerical IDs.
        /// For all scales, use `Some(`[`CensusScales::All`]`)`.
        scale: Option<CensusScales>,
        /// Specify what population the scale should be compared against.
        modes: Option<CensusModes>,
    },
    /// Information on how nations in the region rank according to the World Census.
    ///
    /// Parallels [`WorldShard::CensusRanks`].
    ///
    /// [`WorldShard`]: crate::shards::world::WorldShard
    CensusRanks {
        /// The World Census ranking to use. If `None`, returns the day's featured World Census ranking.
        scale: Option<u8>,
        /// The rank at which to start listing (e.g. `Some(1000)` would start at the 1000th nation).
        start: Option<u32>,
    },
    /// The database ID of the region.
    DbId,
    /// The delegate of the region.
    Delegate,
    /// The authorities the regional delegate has.
    DelegateAuth,
    /// The voting power the regional delegate has (number of verified endorsements + 1).
    DelegateVotes,
    /// The IDs of the dispatches pinned on the region's page.
    Dispatches,
    /// The list of all embassies the region has.
    Embassies,
    /// The authority necessary for nations in embassy regions to post on the regional message board.
    EmbassyRmb,
    /// The region's World Factbook Entry, returned as BBCode.
    ///
    /// Note: do not confuse this with a nation's factbook.
    Factbook,
    /// The regional flag.
    Flag,
    /// A formatted string that denotes how long ago the region was founded.
    ///
    /// Note: some regions have existed "since antiquity" (before this statistic was logged).
    Founded,
    /// The Unix timestamp of when the region was founded.
    ///
    /// Note: some regions have existed "since antiquity" (before this statistic was logged).
    FoundedTime,
    /// The founder of the region.
    ///
    /// Note: special regions (Feeders, Restorers, Catchers, and Sandboxes) do not have founders.
    Founder,
    /// Whether the region is a Frontier.
    Frontier,
    /// The number of nations voting for and against the current General Assembly resolution.
    GAVote,
    /// The 10 most recent events in the region.
    Happenings,
    /// The history of delegates of the region, as well as its embassies.
    History,
    /// The Unix timestamp when the region had its last update.
    LastUpdate,
    /// The Unix timestamp when the region had its last major update.
    LastMajorUpdate,
    /// The Unix timestamp when the region had its last minor update.
    LastMinorUpdate,
    /// Returns messages posted on a regional message board.
    /// By default, returns the 10 most recent messages, sorted from oldest to newest.
    Messages {
        /// Return this many messages. Must be in the range 1-100.
        limit: Option<u8>,
        /// Skip the most recent (number) messages.
        offset: Option<u32>,
        /// Instead of returning the most recent messages, return messages starting from this post ID.
        from_id: Option<u32>,
    },
    /// The name of the region.
    Name,
    /// The list of all nations in the region.
    Nations,
    /// The number of nations in the region.
    NumNations,
    /// The number of World Assembly nations in the region.
    NumWANations,
    /// The list of all regional officers.
    Officers,
    /// The current poll in the region.
    Poll,
    /// The power rating of the region.
    Power,
    /// The number of nations voting for and against the current Security Council resolution.
    SCVote,
    /// The list of tags the region uses.
    Tags,
    /// The list of passed World Assembly resolutions targeting the region.
    WABadges,
    /// The list of World Assembly nations in the region.
    WANations,
}

impl<'a> From<RegionShard> for Shard<'a> {
    //noinspection SpellCheckingInspection
    fn from(value: RegionShard) -> Self {
        Self {
            query: Self::name(&value),
            params: {
                let mut param_map = Params::default();
                match value {
                    RegionShard::Census { scale, modes } => {
                        param_map.insert_scale(&scale).insert_modes(&modes);
                    }
                    RegionShard::CensusRanks { scale, start } => {
                        param_map
                            .insert_scale(&scale.map(CensusScales::One))
                            .insert_start(&start);
                    }
                    RegionShard::Messages {
                        limit,
                        offset,
                        from_id,
                    } => {
                        if let Some(l) = limit {
                            param_map.0.insert("limit", l.to_string());
                        }
                        if let Some(o) = offset {
                            param_map.0.insert("offset", o.to_string());
                        }
                        if let Some(f) = from_id {
                            param_map.0.insert("fromid", f.to_string());
                        }
                    }
                    _ => {}
                };
                param_map
            },
        }
    }
}
