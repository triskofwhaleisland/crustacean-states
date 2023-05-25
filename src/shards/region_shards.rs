//! Contains everything needed to make region shard requests.

use crate::safe_name;
use crate::shards::public_nation_shards::{format_census, CensusModes, CensusScales};
use crate::shards::world_shards::format_census_ranks;
use std::fmt::{Display, Formatter};

/// The intended way to make a region API request.
pub struct RegionRequest {
    region: String,
    shards: Option<Vec<RegionShard>>,
}

impl RegionRequest {
    /// Create a region request with any number of [`RegionShard`]s.
    ///
    /// # Example
    ///
    /// ```
    /// use crustacean_states::shards::region_shards::{RegionRequest, RegionShard};
    ///
    /// let request = RegionRequest::new("Testregionia",
    ///         &[RegionShard::Delegate, RegionShard::Flag]).to_string();
    /// ```
    ///
    /// When sent,
    /// it will request information about [Testregionia](https://www.nationstates.net/region=testregionia)'s delegate and flag.
    pub fn new(region: impl ToString, shards: &[RegionShard]) -> Self {
        RegionRequest {
            region: region.to_string(),
            shards: if shards.is_empty() {
                None
            } else {
                Some(shards.to_vec())
            },
        }
    }
    /// Create a "standard" region request.
    pub fn new_standard(region: impl ToString) -> Self {
        RegionRequest {
            region: region.to_string(),
            shards: None,
        }
    }
}

impl Display for RegionRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "region={}{}",
            safe_name(&self.region),
            self.shards
                .as_ref()
                .map(|shards| shards
                    .iter()
                    .fold("&q=".to_string(), |acc, shard| format!("{acc}+{shard}")))
                .unwrap_or_default()
        )
    }
}

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
    /// [`PublicNationShard`]: crate::shards::public_nation_shards::PublicNationShard
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
    /// [`WorldShard`]: crate::shards::world_shards::WorldShard
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

impl Display for RegionShard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                RegionShard::Census { scale, modes } => {
                    format_census(scale, modes)
                }
                RegionShard::CensusRanks { scale, start } => {
                    format_census_ranks(&scale.map(CensusScales::One), start)
                }
                RegionShard::Messages {
                    limit,
                    offset,
                    from_id,
                } => {
                    format!(
                        "messages{}{}{}",
                        limit
                            .as_ref()
                            .map(|x| format!("&limit={x}"))
                            .unwrap_or_default(),
                        offset
                            .as_ref()
                            .map(|x| format!("&offset={x}"))
                            .unwrap_or_default(),
                        from_id
                            .as_ref()
                            .map(|x| format!("&fromid={x}"))
                            .unwrap_or_default(),
                    )
                }
                other_shard => format!("{:?}", other_shard).to_lowercase(),
            }
        )
    }
}
