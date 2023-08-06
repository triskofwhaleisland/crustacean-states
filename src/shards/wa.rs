//! For World Assembly shard requests.

use crate::shards::{NSRequest, BASE_URL};
use strum::AsRefStr;
use url::Url;

/// One of the two World Assembly chambers (or "councils").
#[repr(u8)]
#[derive(Clone, Debug, Default)]
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
    /// The number of nations in the World Assembly.
    NumNations,
    /// The number of delegates in the World Assembly.
    NumDelegates,
    /// The list of delegates currently serving in the World Assembly.
    Delegates,
    /// The list of all members of the World Assembly.
    Members,
    /// A shard that returns `[Event]`s in the World Assembly.
    ///
    /// [Event]: crate::parsers::happenings::Event
    Happenings,
    /// All the currently proposed resolutions in a World Assembly council.
    Proposals,
    /// Information about a resolution in a World Assembly council.
    /// Request more information with [`ResolutionShard`]s.
    CurrentResolution(&'a [ResolutionShard]),
    /// The most recent resolution in a World Assembly council.
    LastResolution,
    /// Information about a previous resolution.
    PreviousResolution(u16),
}

#[derive(Default)]
pub struct WARequest<'a> {
    council: WACouncil,
    shards: Vec<WAShard<'a>>,
}

impl<'a> NSRequest for WARequest<'a> {
    fn as_url(&self) -> Url {
        let mut query = vec![("wa", (self.council.clone() as u8).to_string())];
        self.shards.iter().for_each(|s| {
            if let WAShard::PreviousResolution(id) = s {
                query.push(("id", id.to_string()));
            };
            query.push((
                "q",
                if let WAShard::CurrentResolution(other_shards) = s {
                    other_shards
                        .iter()
                        .fold(String::from("resolution"), |acc, s| format!("{acc}+{s:?}"))
                } else {
                    s.as_ref().to_string()
                }.to_ascii_lowercase(),
            ))
        });
        Url::parse_with_params(BASE_URL, query).unwrap()
    }
}

/// Extra information about the current at-vote resolution.
#[derive(Debug)]
pub enum ResolutionShard {
    /// Lists every nation voting for and against the resolution.
    Voters,
    /// Information about how many votes each side gets over time.
    VoteTrack,
    /// Lists every delegate's vote, including voting power.
    /// NOTE: this will not return the resolution text.
    /// Votes are chronologically ordered, oldest vote first.
    DelLog,
    /// List every delegate's vote, including voting power.
    /// NOTE: Votes are grouped into yes and no votes.
    DelVotes,
}
