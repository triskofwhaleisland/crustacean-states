//! For world shard requests.

use crate::impl_display_as_debug;
use crate::shards::nation::{CensusModes, CensusScales};
use crate::shards::world::HappeningsViewType::{Nation, Region};
use crate::shards::{Params, Shard};
use itertools::Itertools;

use crate::parsers::nation::IntoNationError;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

/// A request for the wide world of NationStates.
#[derive(Clone, Debug)]
pub enum WorldShard<'a> {
    /// Provides the name of a banner given its ID, as well as the necessary conditions to unlock it.
    Banner(Vec<BannerId>),
    /// By default, returns the score, rank, and region rank on today's featured World Census scale.
    /// Can be optionally configured with additional parameters.
    /// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
    ///
    /// Parallels [`PublicNationShard::Census`][crate::shards::public_nation::PublicNationShard::Census].
    Census {
        /// Specify the World Census scale(s) to list, using numerical IDs.
        /// For all scales, use `Some(`[`CensusScales::All`]`)`.
        scale: Option<CensusScales>,
        /// Specify what population the scale should be compared against.
        modes: Option<CensusModes>,
    },
    /// Today's featured census scale.
    CensusId,
    /// Provides the description of a given census scale if `Some(id)`
    /// or of today's featured census scale if `None`.
    CensusDesc(Option<u8>),
    /// Provides the name of a given census scale if `Some(id)`
    /// or of today's featured census scale if `None`.
    CensusName(Option<u8>),
    /// Provides 20 nations and their world census scale ranking.
    CensusRanks {
        /// If `Some(id)`, a given census scale; if `None`, today's featured census scale.
        scale: Option<u8>,
        /// If `Some(x)`, the ranking place to start at (e.g. `Some(1000)`
        /// means to start at the 1000th-place nation and descend to the 1019th);
        /// if `None`, start at the nation ranked first and descend to the 20th.
        start: Option<u32>,
    },
    /// Provides the units of a given census scale if `Some(id)`
    /// or of today's featured census scale if `None`.
    CensusScale(Option<u8>),
    /// Provides the index that nations are ranked on for a given census scale if `Some(id)`
    /// or for today's featured census scale if `None`.
    CensusTitle(Option<u8>),
    /// Gets a dispatch with a specific ID.
    Dispatch(u32),
    /// Lists 20 dispatches. The fields can provide more control.
    DispatchList {
        /// If `Some(nation)`, search only for dispatches written by `nation`.
        author: Option<&'a str>,
        /// If `Some(category)`, search only for dispatches that have a certain category.
        category: Option<DispatchCategory>,
        /// If `Some(sort)`, sort according to the dispatch sorting rules.
        sort: Option<DispatchSort>,
    },
    /// Gets the featured region on the website, which updates daily.
    FeaturedRegion,
    /// Lists the 100 most recent events. The fields can provide more control.
    Happenings {
        /// Only get events from a certain nation or region.
        view: Option<HappeningsViewType>,
        /// Only get events of a certain type.
        filter: Option<Vec<HappeningsFilterType>>,
        /// Limit the number of events. NOTE: the limit cannot be less than 100.
        limit: Option<u8>,
        /// Filters events to only those after a certain event ID. NOTE:
        /// if the ID was issued more than 100 events ago,
        /// only the 100 most recent events will be provided.
        since_id: Option<u32>,
        /// Filters events to only those before a certain event ID. NOTE:
        /// if the ID was issued more than 100 events ago, no events will be provided.
        before_id: Option<u32>,
        /// Filters events to only those after a certain timestamp.
        /// NOTE: If more than 100 events have occurred since this timestamp,
        /// only the 100 most recent events will be provided.
        since_time: Option<u64>,
        /// Filters events to only those before a certain timestamp.
        /// NOTE:
        /// if more than 100 events have occurred since this timestamp, no events will be provided.
        before_time: Option<u64>,
    },
    /// The most recently issued event ID.
    LastEventId,
    /// List of every nation in the game right now.
    /// WARNING:
    /// There are nearly 300,000 nations currently on NationStates (as of 20 June 2023),
    /// over 8.6 million nations have been created on the site,
    /// and in April 2020 there was an all-time high of 600,000 nations in the game.
    /// Be very careful.
    Nations,
    /// The 50 most recently created nations.
    NewNations,
    /// The number of nations currently in the game.
    NumNations,
    /// The number of regions currently in the game.
    NumRegions,
    /// Get a poll with a specific poll ID.
    Poll(u32),
    /// List of every region in the game right now.
    /// WARNING:
    /// There are nearly 30,000 regions currently on NationStates (as of 20 June 2023),
    /// and there have been times when there are even more.
    /// Be careful when requesting this!
    Regions,
    // TODO implement correctly
    // /// List of regions which do have some tags and don't have others.
    // RegionsByTag(Vec<IncludeOrExcludeTag>),
    /// The number of manual, mass, and API telegrams in the queue.
    TGQueue,
}

impl<'a> From<WorldShard<'a>> for Shard<'a> {
    //noinspection SpellCheckingInspection
    fn from(value: WorldShard) -> Self {
        Self {
            query: Self::name(&value),
            params: {
                let mut param_map = Params::default();
                match value {
                    WorldShard::Banner(banners) => {
                        param_map
                            .0
                            .insert("banner", banners.iter().map(BannerId::to_string).join(","));
                    }
                    WorldShard::Census { scale, modes } => {
                        param_map.insert_scale(&scale).insert_modes(&modes);
                    }
                    WorldShard::CensusDesc(scale)
                    | WorldShard::CensusScale(scale)
                    | WorldShard::CensusName(scale)
                    | WorldShard::CensusTitle(scale) => {
                        if let Some(s) = scale {
                            param_map.0.insert("scale", s.to_string());
                        }
                    }
                    WorldShard::CensusRanks { scale, start } => {
                        param_map
                            .insert_scale(&scale.map(CensusScales::One))
                            .insert_start(&start);
                    }
                    WorldShard::Dispatch(id) => {
                        param_map.0.insert("dispatchid", id.to_string());
                    }
                    WorldShard::DispatchList {
                        author,
                        category,
                        sort,
                    } => {
                        if let Some(a) = author {
                            param_map.0.insert("dispatchauthor", a.to_string());
                        }
                        if let Some(c) = category {
                            param_map.0.insert("dispatchcategory", c.to_string());
                        }
                        if let Some(s) = sort {
                            param_map.0.insert("dispatchsort", s.to_string());
                        }
                    }
                    WorldShard::Happenings {
                        view,
                        filter,
                        limit,
                        since_id,
                        before_id,
                        since_time,
                        before_time,
                    } => {
                        if let Some(v) = view {
                            param_map.0.insert(
                                "view",
                                format!(
                                    "{}.{}",
                                    match v {
                                        Nation(..) => "nation",
                                        Region(..) => "region",
                                    },
                                    match v {
                                        Nation(entities) | Region(entities) => {
                                            entities.iter().join(",")
                                        }
                                    }
                                ),
                            );
                        }
                        if let Some(filters) = filter {
                            param_map.0.insert("filter", filters.iter().join("+"));
                        }
                        if let Some(x) = limit {
                            param_map.0.insert("limit", x.to_string());
                        }
                        if let Some(x) = since_id {
                            param_map.0.insert("sinceid", x.to_string());
                        }
                        if let Some(x) = before_id {
                            param_map.0.insert("beforeid", x.to_string());
                        }
                        if let Some(x) = since_time {
                            param_map.0.insert("sincetime", x.to_string());
                        }
                        if let Some(x) = before_time {
                            param_map.0.insert("beforetime", x.to_string());
                        }
                    }
                    // WorldShard::RegionsByTag(complex_tags) => {
                    //     param_map.0.insert("tags", complex_tags.iter().join(","));
                    // }
                    _ => {}
                }
                param_map
            },
        }
    }
}

/// The best way to build a request for world events.
#[derive(Default)]
pub struct HappeningsShardBuilder {
    view: Option<HappeningsViewType>,
    filter: Vec<HappeningsFilterType>,
    limit: Option<u8>,
    since_id: Option<u32>,
    before_id: Option<u32>,
    since_time: Option<u64>,
    before_time: Option<u64>,
}

impl HappeningsShardBuilder {
    /// Create a new [`HappeningsShardBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Restrict the events gathered to one nation.
    pub fn view_nation(mut self, nation: &str) -> Self {
        self.view = Some(Nation(vec![nation.to_string()]));
        self
    }

    /// Restrict the events gathered to several nations.
    pub fn view_nations(mut self, nations: &[&str]) -> Self {
        self.view = Some(Nation(
            nations.iter().map(|nation| nation.to_string()).collect(),
        ));
        self
    }

    /// Restrict the events gathered to one region.
    pub fn view_region(mut self, region: &str) -> Self {
        self.view = Some(Region(vec![region.to_string()]));
        self
    }

    /// Restrict the events gathered to several regions.
    pub fn view_regions(mut self, regions: &[&str]) -> Self {
        self.view = Some(Region(
            regions.iter().map(|region| region.to_string()).collect(),
        ));
        self
    }

    /// Add one filter to the events request.
    pub fn add_filter(self, filter: HappeningsFilterType) -> Self {
        self.add_filters(vec![filter])
    }

    /// Add several filters to the events request.
    pub fn add_filters(mut self, filters: Vec<HappeningsFilterType>) -> Self {
        filters
            .into_iter()
            .for_each(|filter| self.filter.push(filter));
        self
    }

    /// Limit event gathering to a certain number of results. NOTE: This number may not be larger than 100.
    pub fn limit(mut self, max_results: u8) -> Self {
        self.limit = Some(max_results);
        self
    }

    /// Filters events to only those after a certain event ID. NOTE:
    /// if the ID was issued more than 100 events ago,
    /// only the 100 most recent events will be provided.
    pub fn since_id(mut self, id: u32) -> Self {
        self.since_id = Some(id);
        self
    }

    /// Filters events to only those before a certain event ID. NOTE:
    /// if the ID was issued more than 100 events ago, no events will be provided.
    pub fn before_id(mut self, id: u32) -> Self {
        self.before_id = Some(id);
        self
    }

    /// Filters events to only those after a certain timestamp.
    /// NOTE: If more than 100 events have occurred since this timestamp,
    /// only the 100 most recent events will be provided.
    pub fn since_time(mut self, timestamp: u64) -> Self {
        self.since_time = Some(timestamp);
        self
    }

    /// Filters events to only those before a certain timestamp.
    /// NOTE:
    /// if more than 100 events have occurred since this timestamp, no events will be provided.
    pub fn before_time(mut self, timestamp: u64) -> Self {
        self.before_time = Some(timestamp);
        self
    }

    /// Creates a [`WorldShard::Happenings`] variant from the provided information.
    pub fn build<'a>(self) -> WorldShard<'a> {
        WorldShard::Happenings {
            view: self.view,
            filter: if self.filter.is_empty() {
                None
            } else {
                Some(self.filter)
            },
            limit: self.limit,
            since_id: self.since_id,
            before_id: self.before_id,
            since_time: self.since_time,
            before_time: self.before_time,
        }
    }
}

/// The ID of a banner. WIP. TODO make banner ids
#[derive(Clone, Debug)]
pub struct BannerId {
    pub(crate) category: String,
    pub(crate) number: u16,
}

impl Display for BannerId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.category.to_ascii_lowercase(), self.number)
    }
}

impl BannerId {
    fn new(category: impl ToString, number: u16) -> Self {
        Self {
            category: category.to_string(),
            number,
        }
    }
}

impl TryFrom<String> for BannerId {
    type Error = IntoNationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let split_index = value.chars().position(|c| c.is_ascii_digit());
        if split_index.is_none() || split_index == Some(0) {
            return Err(IntoNationError::BadBannerId(value));
        }
        let (cat, num) = value.split_at(split_index.unwrap());
        let num = u16::from_str(num).map_err(|_| IntoNationError::BadBannerId(value.clone()))?;
        Ok(BannerId::new(cat, num))
    }
}

/// The ways to sort dispatches.
#[derive(Clone, Debug)]
pub enum DispatchSort {
    /// Newest first.
    New,
    /// Highest-rated first.
    Best,
}

impl_display_as_debug!(DispatchSort);

/// The categories of dispatches.
#[derive(Clone, Debug)]
pub enum DispatchCategory {
    /// Factbooks officially describe a nation.
    Factbook(FactbookCategory),
    /// Bulletins address gameplay.
    Bulletin(BulletinCategory),
    /// Accounts are articles or stories involving a nation's people.
    Account(AccountCategory),
    /// Meta dispatches tend to address out-of-character and outside-of-role-play situations.
    Meta(MetaCategory),
}

#[derive(Clone, Debug)]
#[allow(missing_docs)]
#[non_exhaustive]
/// The subcategories of factbooks.
/// Note that the [`FactbookCategory::Any`] variant can be used as a shard
/// to ask for any factbook.
pub enum FactbookCategory {
    Overview,
    History,
    Geography,
    Culture,
    Politics,
    Legislation,
    Religion,
    Military,
    Economy,
    International,
    Trivia,
    Miscellaneous,
    /// The type to choose if you are not picking a subcategory.
    /// NOTE:
    /// This is only used in shard queries;
    /// no [`Dispatch`][crate::parsers::nation::Dispatch] will ever be tagged [`FactbookCategory::Any`].
    Any,
}

#[derive(Clone, Debug)]
#[allow(missing_docs)]
#[non_exhaustive]
/// The subcategories of bulletins.
/// Note that the [`BulletinCategory::Any`] variant can be used as a shard
/// to ask for any bulletin.
pub enum BulletinCategory {
    Policy,
    News,
    Opinion,
    Campaign,
    /// The type to choose if you are not picking a subcategory.
    /// NOTE:
    /// This is only used in shard queries;
    /// no [`Dispatch`][crate::parsers::nation::Dispatch] will ever be tagged [`BulletinCategory::Any`].
    Any,
}

#[derive(Clone, Debug)]
#[allow(missing_docs)]
#[non_exhaustive]
/// The subcategories of accounts.
/// Note that the [`AccountCategory::Any`] variant can be used as a shard
/// to ask for any account.
pub enum AccountCategory {
    Military,
    Trade,
    Sport,
    Drama,
    Diplomacy,
    Science,
    Culture,
    Other,
    /// The type to choose if you are not picking a subcategory.
    /// NOTE:
    /// This is only used in shard queries;
    /// no [`Dispatch`][crate::parsers::nation::Dispatch] will ever be tagged [`AccountCategory::Any`].
    Any,
}

#[derive(Clone, Debug)]
#[allow(missing_docs)]
#[non_exhaustive]
/// The subcategories of meta-category dispatches.
/// Note that the [`MetaCategory::Any`] variant can be used as a shard
/// to ask for any meta-category dispatch.
pub enum MetaCategory {
    Gameplay,
    Reference,
    /// The type to choose if you are not picking a subcategory.
    /// NOTE:
    /// This is only used in shard queries;
    /// no [`Dispatch`][crate::parsers::nation::Dispatch] will ever be tagged [`MetaCategory::Any`].
    Any,
}

impl Display for FactbookCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FactbookCategory::Any => String::new(),
                c => format!("{c:?}"),
            }
        )
    }
}

impl Display for BulletinCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BulletinCategory::Any => String::new(),
                c => format!("{c:?}"),
            }
        )
    }
}

impl Display for AccountCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AccountCategory::Any => String::new(),
                c => format!("{c:?}"),
            }
        )
    }
}

impl Display for MetaCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MetaCategory::Any => String::new(),
                c => format!("{c:?}"),
            }
        )
    }
}

impl Display for DispatchCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let c: (&str, Option<String>) = match self {
                DispatchCategory::Factbook(subcategory) => (
                    "Factbook",
                    match subcategory {
                        FactbookCategory::Any => None,
                        other => Some(other.to_string()),
                    },
                ),
                DispatchCategory::Bulletin(subcategory) => (
                    "Bulletin",
                    match subcategory {
                        BulletinCategory::Any => None,
                        other => Some(other.to_string()),
                    },
                ),
                DispatchCategory::Account(subcategory) => (
                    "Account",
                    match subcategory {
                        AccountCategory::Any => None,
                        other => Some(other.to_string()),
                    },
                ),
                DispatchCategory::Meta(subcategory) => (
                    "Meta",
                    match subcategory {
                        MetaCategory::Any => None,
                        other => Some(other.to_string()),
                    },
                ),
            };
            format!(
                "{}{}",
                c.0,
                c.1.map(|x| format!(": {x}")).unwrap_or_default(),
            )
        })
    }
}

/// The happenings shard can either target nations or regions.
#[derive(Clone, Debug)]
pub enum HappeningsViewType {
    /// Targets one or more nations.
    Nation(Vec<String>),
    /// Targets one or more regions.
    Region(Vec<String>),
}

/// The happenings shard can target multiple kinds of events.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum HappeningsFilterType {
    /// Triggered by answering an issue (dismissing the issue results in no event).
    /// Follows the form "Following new legislation in NATION, \[joke about new policy]."
    Law,
    /// This category includes multiple events,
    /// such as nations being reclassified due to their responses to issues,
    /// altering of national flags and other custom fields, and creating custom banners.
    Change,
    /// Announces the publishing of a dispatch.
    /// Follows the form "NATION published "Dispatch Title" (Category: Subcategory)."
    Dispatch,
    /// A nation posted on a regional message board.
    /// Follows the form "NATION lodged a message on the REGION regional message board."
    Rmb,
    /// Has to do with embassies between regions:
    /// proposing construction, agreeing to construction,
    /// rejecting requests, aborting construction, ordering closure,
    /// cancelling closure, establishment, and cancellation.
    Embassy,
    /// Has to do with the ejection or ejection + ban ("banjection") of a nation from a region.
    /// Follows the form "NATION was ejected (and banned) from REGION by OTHER NATION."
    Eject,
    /// Has to do with all administrative actions done in a region,
    /// such as banning nations, updating regional tags,
    /// updating the World Factbook Entry,
    /// appointing and dismissing regional officers,
    /// etc. It is also where WA rule-violators get ejected from the WA.
    Admin,
    /// A nation moving from one region to another.
    /// Follows the form "NATION relocated from REGION1 to REGION2."
    Move,
    /// A nation is founded.
    /// Note that if a nation is being revived, it is called a "refound".
    /// Follows the form "NATION was (re)founded in FEEDER/FRONTIER REGION
    Founding,
    /// A nation ceases to exist if it has not been logged in to for the past 28 days.
    /// If you enable "vacation mode" on your nation, it will cease to exist after 60 days.
    /// All CTEs happen at updates, except for when a nation is deleted by moderators.
    /// Follows the form "NATION ceased to exist in REGION."
    Cte,
    /// A nation casts a vote or withdraws its vote in the World Assembly.
    Vote,
    /// A World Assembly proposal is submitted, approved, withdrawn, or it fails to reach quorum.
    Resolution,
    /// A nation applies to, is admitted to, or resigns from the World Assembly.
    Member,
    /// A nation in the World Assembly endorses another nation in the World Assembly in the same region.
    /// Follows the form "NATION1 endorsed NATION2."
    Endo,
}

impl Display for HappeningsFilterType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_ascii_lowercase())
    }
}

// /// When searching regions by tag, you can do it by including certain tags and excluding others.
// /// Example:
// /// ```
// /// use url::Url;
// /// use crustacean_states::shards::NSRequest;
// /// use crustacean_states::shards::world::IncludeOrExcludeTag::{Exclude, Include};
// /// use crustacean_states::shards::world::Tag::{Fandom, Fascist, RegionalGovernment};
// /// use crustacean_states::shards::world::WorldShard;
// ///
// /// let request = Url::from(NSRequest::new_world(vec![WorldShard::RegionsByTag(vec![
// ///     Include(RegionalGovernment), Include(Fandom), Exclude(Fascist)
// /// ])]));
// /// assert_eq!(
// ///     request.as_str(),
// ///     "https://www.nationstates.net/cgi-bin/api.cgi?q=regionsbytag&tags=regionalgovernment%2Cfandom%2C-fascist",
// /// )
// /// ```
// #[derive(Clone, Debug)]
// pub enum IncludeOrExcludeTag {
//     /// Include this tag.
//     Include(Tag),
//     /// Exclude this tag.
//     Exclude(Tag),
// }

// impl Display for IncludeOrExcludeTag {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self {
//                 IncludeOrExcludeTag::Include(tag) => {
//                     format!("{:?}", tag).to_ascii_lowercase()
//                 }
//                 IncludeOrExcludeTag::Exclude(tag) => {
//                     format!("-{:?}", tag).to_ascii_lowercase()
//                 }
//             }
//         )
//     }
// }

//noinspection SpellCheckingInspection
#[derive(Clone, Debug)]
#[non_exhaustive]
#[allow(missing_docs)]
pub enum Tag {
    Anarchist,
    Anime,
    AntiCapitalist,
    AntiCommunist,
    AntiFascist,
    AntiGeneralAssembly,
    AntiSecurityCouncil,
    AntiWorldAssembly,
    Capitalist,
    Casual,
    Catcher,
    Class,
    Colony,
    Commended,
    Communist,
    Condemned,
    Conservative,
    Cyberpunk,
    Defender,
    Democratic,
    EcoFriendly,
    Egalitarian,
    EmbassyCollector,
    Enormous,
    ForumSeven,
    FutureTechFasterThanLight,
    FutureTechFasterThanLightInhibited,
    FutureTechSlowerThanLight,
    Fandom,
    FantasyTec,
    // Eww.
    Fascist,
    Featured,
    Feeder,
    Feminist,
    Founderless,
    FreeTrade,
    Frontier,
    FutureTech,
    GamePlayer,
    Gargantuan,
    GeneralAssembly,
    Generalite,
    Governorless,
    HumanOnly,
    Imperialist,
    Independent,
    Industrial,
    Injuncted,
    InternationalFederalist,
    Invader,
    Isolationist,
    IssuesPlayer,
    JumpPoint,
    Lgbt,
    Large,
    Liberal,
    Liberated,
    Libertarian,
    Magical,
    Map,
    Miniscule,
    ModernTech,
    Monarchist,
    MultiSpecies,
    NationalSovereigntist,
    Neutral,
    New,
    NonEnglish,
    OffsiteChat,
    OffsiteForums,
    OuterSpace,
    PortalToTheMultiverse,
    Pacifist,
    Parody,
    Password,
    PastTech,
    Patriarchal,
    PostApocalyptic,
    PostModernTech,
    PuppetStorage,
    RegionalGovernment,
    Religious,
    Restorer,
    RolePlayer,
    SecurityCouncil,
    Serious,
    Silly,
    Sinker,
    Small,
    Snarky,
    Social,
    Socialist,
    Sports,
    Steampunk,
    Surreal,
    Theocratic,
    Totalitarian,
    TradingCards,
    VideoGame,
    Warzone,
    WorldAssembly,
}
