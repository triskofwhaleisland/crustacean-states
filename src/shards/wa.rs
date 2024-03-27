//! For World Assembly shard requests.

use crate::shards::{NSRequest, Params, BASE_URL};
use itertools::Itertools;
use std::{
    fmt::{Display, Formatter},
    string::ToString,
};
use strum::{AsRefStr, Display};
use url::Url;

/// One of the two World Assembly chambers (or "councils").
#[repr(u8)]
#[derive(Copy, Clone, Debug, Default)]
pub enum WACouncil {
    /// The General Assembly.
    ///
    /// In-game description:
    /// "The oldest Council of the World Assembly,
    /// the General Assembly concerns itself with international law.
    /// Its resolutions are applied immediately upon passing in all WA member nations."
    /// [link](https://www.nationstates.net/page=ga)
    #[default]
    GeneralAssembly = 1,
    /// The Security Council.
    ///
    /// In-game description:
    /// "The Security Council recognizes and responds to individual nations and regions,
    /// with the aim of ensuring global harmony."
    /// [link](https://www.nationstates.net/page=sc)
    SecurityCouncil = 2,
}

/// A shard for the World Assembly.
#[derive(AsRefStr, Clone, Debug)]
pub enum WAShard<'a> {
    /// Information about the WA as a whole.
    GlobalInfo(WAGlobalShard),
    /// Information about an individual WA chamber.
    CouncilInfo(WACouncilShard),
    /// Information about a resolution in a World Assembly council.
    /// Request more information with [`ResolutionShard`]s.
    CurrentResolution(&'a [ResolutionShard]),
    /// Information about a previous resolution.
    PreviousResolution(u16),
}

impl<'a> From<WAGlobalShard> for WAShard<'a> {
    fn from(value: WAGlobalShard) -> Self {
        Self::GlobalInfo(value)
    }
}

impl<'a> From<WACouncilShard> for WAShard<'a> {
    fn from(value: WACouncilShard) -> Self {
        Self::CouncilInfo(value)
    }
}

impl<'a> From<&'a [ResolutionShard]> for WAShard<'a> {
    fn from(value: &'a [ResolutionShard]) -> Self {
        WAShard::CurrentResolution(value)
    }
}

impl<'a> From<u16> for WAShard<'a> {
    fn from(value: u16) -> Self {
        WAShard::PreviousResolution(value)
    }
}

impl<'a> Display for WAShard<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WAShard::GlobalInfo(g) => g.to_string(),
                WAShard::CouncilInfo(c) => c.to_string(),
                WAShard::CurrentResolution(a) => format!("resolution{}", a.iter().join("+")),
                WAShard::PreviousResolution(_) => String::from("resolution"),
            }
            .to_ascii_lowercase()
        )
    }
}

/// Information about the World Assembly as a whole.
#[derive(Clone, Debug, Display)]
pub enum WAGlobalShard {
    /// The number of nations in the World Assembly.
    NumNations,
    /// The number of delegates in the World Assembly.
    NumDelegates,
    /// The list of delegates currently serving in the World Assembly.
    Delegates,
    /// The list of all World Assembly members.
    Members,
}

/// Information for the World Assembly that is specific to a council.
#[derive(Clone, Debug, Display)]
pub enum WACouncilShard {
    /// A shard that returns `[Event]`s in the World Assembly.
    ///
    /// [Event]: crate::parsers::happenings::Event
    Happenings,
    /// All the currently proposed resolutions in a World Assembly council.
    Proposals,
    /// The most recent resolution in a World Assembly council.
    LastResolution,
}

/// Information about the current at-vote resolution.
#[derive(Clone, Debug, Display)]
pub enum ResolutionShard {
    /// Lists every nation voting for and against the resolution.
    Voters,
    /// Information about how many votes each side gets over time.
    VoteTrack,
    /// Lists every delegate's vote, including voting power.
    /// NOTE: this will not return the resolution text.
    /// Votes are chronologically ordered; the oldest vote comes first.
    DelLog,
    /// List every delegate's vote, including voting power.
    /// NOTE: Votes are grouped into yes and no votes.
    DelVotes,
}

/// Request information about the WA.
#[derive(Clone, Debug)]
pub enum WARequest<'a> {
    /// Information about the WA as a whole.
    Global(GlobalRequest<'a>),
    /// Information about a WA council.
    Council(CouncilRequest<'a>),
    /// Information about the at-vote resolution.
    AtVoteResolution(ResolutionRequest<'a>),
    /// Information about a previous resolution.
    PastResolution(ResolutionArchiveRequest),
}

/// Request information about the WA as a whole.
#[derive(Clone, Debug)]
pub struct GlobalRequest<'a> {
    shards: &'a [WAGlobalShard],
}

impl<'a> GlobalRequest<'a> {
    /// Create a new request about the WA as a whole.
    pub fn new(shards: &'a [WAGlobalShard]) -> Self {
        Self { shards }
    }
}

/// Request information about a WA council.
#[derive(Clone, Debug)]
pub struct CouncilRequest<'a> {
    council: WACouncil,
    shards: &'a [WAShard<'a>],
}

impl<'a> CouncilRequest<'a> {
    /// Create a request about a WA council.
    pub fn new(council: WACouncil, shards: impl AsRef<&'a [WAShard<'a>]>) -> Self {
        Self {
            council,
            shards: shards.as_ref(),
        }
    }
}
impl<'a, T: AsRef<&'a [WAShard<'a>]>> From<(WACouncil, T)> for CouncilRequest<'a> {
    fn from(value: (WACouncil, T)) -> Self {
        Self {
            council: value.0,
            shards: value.1.as_ref(),
        }
    }
}

/// Request information about the current at-vote resolution.
#[derive(Clone, Debug)]
pub struct ResolutionRequest<'a> {
    council: WACouncil,
    shards: &'a [ResolutionShard],
}

impl<'a> ResolutionRequest<'a> {
    /// Create a request about the current at-vote resolution.
    pub fn new(council: WACouncil, shards: &'a [ResolutionShard]) -> Self {
        Self { council, shards }
    }
}

/// Request information about previous resolutions.
#[derive(Clone, Debug)]
pub struct ResolutionArchiveRequest {
    council: WACouncil,
    id: u16,
}

impl ResolutionArchiveRequest {
    /// Create a request about previous resolutions.
    pub fn new(council: WACouncil, id: u16) -> Self {
        Self { council, id }
    }
}

impl<'a> NSRequest for WARequest<'a> {
    fn as_url(&self) -> Url {
        Url::parse_with_params(
            BASE_URL,
            Params::default()
                .insert(
                    "wa",
                    match self {
                        WARequest::Global(_) => None,
                        WARequest::Council(CouncilRequest { council, .. })
                        | WARequest::AtVoteResolution(ResolutionRequest { council, .. })
                        | WARequest::PastResolution(ResolutionArchiveRequest { council, .. }) => {
                            Some(*council)
                        }
                    }
                    .unwrap_or_default() as u8,
                )
                .insert_on(
                    "id",
                    &if let WARequest::PastResolution(ResolutionArchiveRequest { id, .. }) = self {
                        Some(id)
                    } else {
                        None
                    },
                )
                .insert(
                    "q",
                    match self {
                        WARequest::Global(GlobalRequest { shards }) => shards.iter().join("+"),
                        WARequest::Council(CouncilRequest { shards, .. }) => {
                            shards.iter().join("+")
                        }
                        WARequest::AtVoteResolution(ResolutionRequest { shards, .. }) => {
                            format!("resolution+{}", shards.iter().join("+"))
                        }
                        WARequest::PastResolution(_) => String::from("resolution"),
                    }
                    .to_ascii_lowercase(),
                ),
        )
        .unwrap()
    }
}
