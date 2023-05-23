use crate::safe_name;
use crate::shards::public_nation_shards::{format_census, CensusModes, CensusScales};
use crate::shards::world_shards::format_census_ranks;
use std::fmt::{Display, Formatter};

pub struct RegionRequest {
    region: String,
    shards: Option<Vec<RegionShard>>,
}

impl RegionRequest {
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
                    .fold(String::new(), |acc, shard| format!("{acc}&{shard}")))
                .unwrap_or_default()
        )
    }
}

#[derive(Clone, Debug)]
pub enum RegionShard {
    BanList,
    Banner,
    BannerBy,
    BannerUrl,
    Census {
        scale: Option<CensusScales>,
        modes: Option<CensusModes>,
    },
    CensusRanks {
        scale: Option<u8>,
        start: Option<u32>,
    },
    DbId,
    Delegate,
    DelegateAuth,
    DelegateVotes,
    Dispatches,
    Embassies,
    EmbassyRmb,
    Factbook,
    Flag,
    Founded,
    FoundedTime,
    Founder,
    Frontier,
    GAVote,
    Happenings,
    History,
    LastUpdate,
    LastMajorUpdate,
    LastMinorUpdate,
    Messages {
        limit: Option<u8>,
        offset: Option<u32>,
        from_id: Option<u32>,
    },
    Name,
    Nations,
    NumNations,
    NumWANations,
    Officers,
    Poll,
    Power,
    SCVote,
    Tags,
    WABadges,
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
                    format_census_ranks(scale, start)
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
