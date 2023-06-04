use crate::shards::Shard;

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum WACouncil {
    GeneralAssembly = 1,
    SecurityCouncil = 2,
}

#[derive(Debug)]
pub enum WAGeneralShard {
    NumNations,
    NumDelegates,
    Delegates,
    Members,
    Happenings,
    Proposals,
    Resolution(Vec<WAAdditionalShards>),
    LastResolution,
}

impl From<WAGeneralShard> for Shard {
    fn from(value: WAGeneralShard) -> Self {
        Self {
            query: match value {
                WAGeneralShard::Resolution(additional_shards) => additional_shards
                    .iter()
                    .fold(String::from("resolution"), |acc, s| format!("{acc}+{s:?}"))
                    .to_lowercase(),
                other => Self::name(&other),
            },
            params: Default::default(),
        }
    }
}

#[derive(Debug)]
pub enum WAAdditionalShards {
    Voters,
    VoteTrack,
    DelLog,
    DelVotes,
}
