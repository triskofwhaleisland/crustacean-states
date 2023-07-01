//! For public nation shard requests.

use crate::shards::{Params, Shard};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::num::NonZeroU64;

/// A nation request available to anyone.
#[derive(Clone, Debug)]
pub enum PublicNationShard<'a> {
    /// A randomly-selected compliment for the nation.
    Admirable,
    /// All possible compliments for the nation.
    Admirables,
    /// The national animal.
    Animal,
    /// Describes the national animal on the nation's page.
    AnimalTrait,
    /// Number of issues answered.
    Answered,
    /// Returns one Rift banner code that should be displayed for this nation:
    /// the nation's primary banner, if one is set; otherwise, a randomly chosen eligible banner.
    /// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
    Banner,
    /// Returns a list of Rift banners that should be displayed:
    /// the nation's primary banner (if any) is always listed first,
    /// with the remainder in random order.
    /// Banner codes can be converted into image URLs by prepending `/images/banners/`
    /// and appending `.jpg`.
    /// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
    Banners,
    /// The capital if a custom capital was chosen, or the nation name with "City"
    /// appended at the end if one has not been chosen yet.
    ///
    // /// See also: [`PublicNationShard::CustomCapital`]
    Capital,
    /// One of the 27 national classifications that the game assigns based on personal,
    /// economic, and political freedom.
    Category,
    /// By default, returns the score, rank, and region rank on today's featured World Census scale.
    /// Can be optionally configured with additional parameters.
    /// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
    Census {
        /// Specify the World Census scale(s) to list, using numerical IDs.
        /// For all scales, use `Some(`[`CensusScales::All`]`)`.
        scale: Option<CensusScales>,
        /// Specify what population the scale should be compared against.
        modes: Option<CensusModes>,
    },
    /// Describes crime in the nation on its nation page.
    Crime,
    /// Name of the national currency.
    Currency,

    // ---
    // **Disabled for overlap with other shards.**
    // /// The national leader only if a leader was chosen.
    // /// If no leader was chosen, the field will return as `Some(None)`.
    // ///
    // /// See also: [`PublicNationShard::Leader`]
    // CustomLeader,
    // /// The national capital only if a capital was chosen.
    // /// If no capital was chosen, the field will return as `Some(None)`.
    // ///
    // /// See also: [`PublicNationShard::Capital`]
    // CustomCapital,
    // /// The national religion only if a religion was chosen.
    // /// If no religion was chosen, the field will return as `Some(None)`.
    // ///
    // /// See also: [`PublicNationShard::Religion`]
    // CustomReligion,
    // ---
    /// The database ID of the nation.
    DbId,
    /// Causes of death and their frequencies as a percentage.
    Deaths,
    /// Adjective used to describe citizens of the nation: e.g. I am French.
    Demonym,
    /// Singular noun used to describe a citizen of the nation: e.g. I am a Frenchman.
    Demonym2,
    /// Plural noun used to describe citizens of the nation: e.g. They are (some) Frenchmen.
    ///
    /// Note that in the English language,
    /// the word "some" is not normally used in that way,
    /// but it would be more inaccurate to say "the".
    /// It should also be noted that the words "Frenchman" and "Frenchmen"
    /// are no longer preferred English words to describe French people;
    /// the adjectival demonym with the words "person" or "people" is now preferred,
    /// e.g. I am a French (adj.) person.
    Demonym2Plural,
    /// The number of dispatches published by this nation.
    ///
    /// This will always be greater than or equal to the number of factbooks published by this nation,
    /// which can be requested with [`PublicNationShard::Factbooks`].
    Dispatches,
    /// The list of all dispatches published by this nation.
    ///
    /// For a list of all factbooks (a subset of dispatches) published by this nation,
    /// see [`PublicNationShard::FactbookList`].
    DispatchList,
    /// The list of nations that endorse this nation.
    ///
    /// Note:
    /// this list provides the same information for nations not in the WA and nations
    /// who are in the WA but have no endorsements.
    /// For WA status, see [`PublicNationShard::WA`].
    Endorsements,
    /// The number of factbooks published by this nation.
    ///
    /// This will always be less than or equal to the number of dispatches published by this nation,
    /// which can be requested with [`PublicNationShard::Dispatches`].
    Factbooks,
    /// The list of all factbooks published by this nation.
    ///
    /// For a list of all dispatches (a superset of factbooks) published by this nation,
    /// see [`PublicNationShard::DispatchList`].
    FactbookList,
    /// The Unix timestamp of the first time the user logged into the nation.
    ///
    /// Note: some nations have existed "since antiquity" (before this statistic was logged).
    FirstLogin,
    /// The URL of the flag used by this nation.
    Flag,
    /// A relative timestamp that denotes how long ago the nation was founded.
    ///
    /// Note: some nations have existed "since antiquity" (before this statistic was logged).
    Founded,
    /// The Unix timestamp of when the nation was founded.
    ///
    /// Note: some nations have existed "since antiquity" (before this statistic was logged).
    FoundedTime,
    /// Describes civil rights, the economy,
    /// and political freedom within the country using *qualitative* descriptors.
    Freedom,
    /// Describes civil rights, the economy,
    /// and political freedom within the country using *quantitative* descriptors.
    FreedomScores,
    /// The full name of the nation.
    ///
    /// Will always be equivalent to "The {[`PublicNationShard::Type`]} of {[`PublicNationShard::Name`]}",
    /// where `{}` represents string interpolation.
    FullName,
    /// The vote of the nation in the General Assembly.
    ///
    /// Note: this shard cannot tell the difference
    /// between the nation not being part of the World Assembly and the nation not voting.
    /// For WA status, see [`PublicNationShard::WA`].
    ///
    /// For the nation's Security Council vote, see [`PublicNationShard::SCVote`].
    GAVote,
    /// The GDP of the nation in its national currency.
    Gdp,
    /// Describes the nation's government spending as percentages in various financial areas.
    ///
    /// To see the financial area where the government spends the most money,
    /// see [`PublicNationShard::GovtPriority`].
    Govt,
    /// The description of the nation's government found on its nation page.
    GovtDesc,
    /// The financial area where the government spends the most money.
    ///
    /// To see government spending in all financial areas, see [`PublicNationShard::Govt`].
    GovtPriority,
    /// The 10 most recent events in the nation.
    Happenings,
    /// The average income in the nation.
    Income,
    /// The description of the nation's industry found on its nation page.
    IndustryDesc,
    /// Describes the influence of the nation in its region using qualitative descriptors.
    Influence,
    /// A relative timestamp that denotes when the nation was last active.
    LastActivity,
    /// The Unix timestamp of when the nation was last logged into.
    LastLogin,
    /// The national leader if a custom leader was chosen, or "Leader"
    /// if one has not been chosen yet.
    ///
    // /// See also: [`PublicNationShard::CustomLeader`]
    Leader,
    /// The list of descriptions of laws in the nation found on its nation page.
    Legislation,
    /// The largest industry in the nation.
    MajorIndustry,
    /// The national motto.
    Motto,
    /// The name of the nation.
    ///
    /// Note: this will always return the correct capitalization of the nation name.
    Name,
    /// Three randomly selected notable facts about the nation,
    /// in the form "{`fact1`}, {`fact2`}, and {`fact3`}",
    /// where `{}` represents string interpolation.
    ///
    /// For all possible notable facts, use [`PublicNationShard::Notables`].
    Notable,
    /// The list of all possible notable facts about the nation.
    ///
    /// For three randomly selected notable facts, use [`PublicNationShard::Notable`].
    Notables,
    /// The list of policies the nation has in place.
    Policies,
    /// The average income of the poorest 10% in the nation.
    Poorest,
    /// The number of people in the nation, measured in millions of people.
    Population,
    /// The percentage of the economy controlled or funded by the government and the public.
    ///
    /// For a breakdown of all sectors and their percent of the economy that they control,
    /// see [`PublicNationShard::Sectors`].
    PublicSector,
    /// The region rank on today's featured World Census scale.
    RCensus,
    /// The region the nation resides in.
    Region,
    /// The national religion if a custom religion was chosen,
    /// or "a major religion" if one has not been chosen yet.
    ///
    // /// See also: [`PublicNationShard::CustomReligion`]
    Religion,
    /// The average income of the richest 10% in the nation.
    Richest,
    /// The vote of the nation in the Security Council.
    ///
    /// Note: this shard cannot tell the difference
    /// between the nation not being part of the World Assembly and the nation not voting.
    /// For WA status, see [`PublicNationShard::WA`].
    ///
    /// For the nation's General Assembly vote, see [`PublicNationShard::GAVote`].
    SCVote,
    /// Describes the nation's economy as percentages controlled or funded by various sectors.
    ///
    /// For the percentage controlled or funded by the government and the public,
    /// see [`PublicNationShard::PublicSector`].
    Sectors,
    /// Two adjectives that describe the nation's population on its nation page.
    Sensibilities,
    /// The national tax rate as a percentage.
    Tax,
    /// Whether a recruitment telegram can be sent to the nation or not.
    TGCanRecruit {
        /// Whether the nation will deny a recruitment telegram from this region in particular due to having received one too recently.
        from: Option<&'a str>,
    },
    /// Whether a campaign telegram can be sent to the nation or not.
    TGCanCampaign {
        /// Whether the nation will deny a campaign telegram from this region in particular due to having received one too recently.
        from: Option<&'a str>,
    },
    /// The pre-title of the nation.
    Type,
    /// Whether the nation is a World Assembly Delegate, a World Assembly member, or a non-member.
    WA,
    /// The list of World Assembly resolutions targeting the nation.
    WABadges,
    /// The world rank on today's featured World Census scale.
    WCensus,
}

#[derive(Clone, Debug)]
/// World census scales as numerical IDs.
/// The IDs can be found [here](https://forum.nationstates.net/viewtopic.php?f=15&t=159491)
/// or in the URL of [World Census](https://www.nationstates.net/page=list_nations?censusid=0)
/// pages.
/// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
pub enum CensusScales {
    /// Only one scale.
    One(u8),
    /// Multiple scales.
    Many(Vec<u8>),
    /// All scales.
    All,
}

#[derive(Clone, Debug)]
/// Either describes current or historical data.
pub enum CensusModes {
    /// This is a special mode that cannot be combined with other modes,
    /// as only scores are available, not ranks.
    /// When requesting history, you can optionally specify a time window, using Unix epoch times.
    /// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
    History {
        /// Beginning of the measurement.
        from: Option<NonZeroU64>,
        /// End of the measurement.
        to: Option<NonZeroU64>,
    },
    /// Represents current data.
    Current(Vec<CensusCurrentModes>),
}

#[derive(Clone, Debug)]
/// Describes data that can currently be found on the World Census.
pub enum CensusCurrentModes {
    /// Raw value.
    Score,
    /// World rank (e.g. "334" means 334th in the world).
    Rank,
    /// Region rank.
    RegionRank,
    /// World rank as a percentage (e.g. "15" means "Top 15%").
    PercentRank,
    /// Region rank as a percentage.
    PercentRegionRank,
}

impl Display for CensusCurrentModes {
    //noinspection SpellCheckingInspection
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CensusCurrentModes::Score => "score",
                CensusCurrentModes::Rank => "rank",
                CensusCurrentModes::RegionRank => "rrank",
                CensusCurrentModes::PercentRank => "prank",
                CensusCurrentModes::PercentRegionRank => "prrank",
            }
        )
    }
}

impl<'a> From<PublicNationShard<'a>> for Shard<'a> {
    //noinspection SpellCheckingInspection
    fn from(value: PublicNationShard<'a>) -> Self {
        Self {
            query: match &value {
                PublicNationShard::Capital => "customcapital".to_string(),
                PublicNationShard::Leader => "customleader".to_string(),
                PublicNationShard::Religion => "customreligion".to_string(),
                other => Self::name(&other),
            },
            params: {
                let mut param_map = Params::default();
                match &value {
                    PublicNationShard::Census { scale, modes } => {
                        param_map.insert_scale(&scale).insert_modes(&modes);
                    }
                    PublicNationShard::TGCanCampaign { from }
                    | PublicNationShard::TGCanRecruit { from } => {
                        if let Some(f) = from {
                            param_map.0.insert("from", f.to_string());
                        }
                    }
                    _ => {} // no other public nation shards require parameters
                };
                param_map
            },
        }
    }
}
