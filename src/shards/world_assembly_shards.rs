use crate::shards::Shard;

#[repr(u8)]
#[derive(Clone, Debug)]
/// One of the two World Assembly chambers (or "councils").
pub enum WACouncil {
    /// The General Assembly.
    ///
    /// In-game description:
    /// "The oldest Council of the World Assembly,
    /// the General Assembly concerns itself with international law.
    /// Its resolutions are applied immediately upon passing in all WA member nations."
    /// [link](https://www.nationstates.net/page=ga)
    GeneralAssembly = 1,
    /// The Security Council.
    ///
    /// In-game description:
    /// "The Security Council recognizes and responds to individual nations and regions,
    /// with the aim of ensuring global harmony."
    /// [link](https://www.nationstates.net/page=sc)
    SecurityCouncil = 2,
}

#[derive(Debug)]
pub enum WAGeneralShard<'a> {
    NumNations,
    NumDelegates,
    Delegates,
    Members,
    Happenings,
    Proposals,
    Resolution(&'a [ResolutionShard]),
    LastResolution,
}

impl<'a> From<WAGeneralShard<'a>> for Shard {
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
pub enum ResolutionShard {
    Voters,
    VoteTrack,
    DelLog,
    DelVotes,
}
