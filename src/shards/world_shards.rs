use crate::impl_display_as_debug;
use crate::shards::public_nation_shards::{
    format_census, format_census_scale, CensusModes, CensusScales,
};
use crate::shards::world_shards::HappeningsViewType::{Nation, Region};
use std::fmt::{Display, Formatter};

/// A request to the world API.
pub struct WorldRequest(Vec<WorldShard>);

impl Display for WorldRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "q={}",
            self.0
                .iter()
                .fold(String::new(), |acc, shard| format!("{acc}+{shard}"))
        )
    }
}

#[derive(Debug)]
pub enum WorldShard {
    Banner(Vec<BannerId>),
    Census {
        scale: Option<CensusScales>,
        modes: Option<CensusModes>,
    },
    CensusId,
    CensusDesc(Option<u8>),
    CensusName(Option<u8>),
    CensusRanks {
        scale: Option<u8>,
        start: Option<u32>,
    },
    CensusScale(Option<u8>),
    CensusTitle(Option<u8>),
    Dispatch(u32),
    DispatchList {
        author: Option<String>,
        category: Option<DispatchCategory>,
        sort: Option<SortType>,
    },
    FeaturedRegion,
    Happenings {
        view: Option<HappeningsViewType>,
        filter: Option<Vec<HappeningsFilterType>>,
        limit: Option<u32>,
        since_id: Option<u32>,
        before_id: Option<u32>,
        since_time: Option<u64>,
        before_time: Option<u64>,
    },
    LastEventId,
    Nations,
    NewNations,
    NumNations,
    NumRegions,
    Poll,
    Regions,
    RegionsByTag(Vec<IncludeOrExcludeTag>),
    TGQueue,
}

impl Display for WorldShard {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                WorldShard::Banner(banners) => {
                    (!banners.is_empty())
                        .then(|| {
                            format!(
                                "banner&banner={}",
                                banners
                                    .iter()
                                    .map(|banner| banner.to_string())
                                    .collect::<Vec<_>>()
                                    .join(",")
                            )
                        })
                        .unwrap_or_default()
                }
                WorldShard::Census { scale, modes } => {
                    format_census(scale, modes)
                }
                WorldShard::CensusName(scale)
                | WorldShard::CensusDesc(scale)
                | WorldShard::CensusScale(scale)
                | WorldShard::CensusTitle(scale) => {
                    format!(
                        "{}{}",
                        match self {
                            WorldShard::CensusName(..) => "censusname",
                            WorldShard::CensusDesc(..) => "censusdesc",
                            WorldShard::CensusScale(..) => "censusscale",
                            WorldShard::CensusTitle(..) => "censustitle",
                            _ => "", // not really necessary but oh well
                        },
                        scale
                            .as_ref()
                            .map(|x| format!("&scale={x}"))
                            .unwrap_or_default()
                    )
                }
                WorldShard::CensusRanks { scale, start } => {
                    format_census_ranks(&scale.map(CensusScales::One), start)
                }
                WorldShard::DispatchList {
                    author,
                    category,
                    sort,
                } => {
                    format!(
                        "dispatchlist{}{}{}",
                        author
                            .as_ref()
                            .map(|a| format!("&dispatchauthor={a}"))
                            .unwrap_or_default(),
                        category
                            .as_ref()
                            .map(|c| format!(
                                "&dispatchcategory={}",
                                match c {
                                    DispatchCategory::Factbook(subcategory) => {
                                        format!(
                                            "Factbook:{}",
                                            subcategory
                                                .as_ref()
                                                .map(|s| s.to_string())
                                                .unwrap_or_default()
                                        )
                                    }
                                    DispatchCategory::Account(subcategory) => {
                                        format!(
                                            "Account:{}",
                                            subcategory
                                                .as_ref()
                                                .map(|s| s.to_string())
                                                .unwrap_or_default()
                                        )
                                    }
                                    DispatchCategory::Bulletin(subcategory) => {
                                        format!(
                                            "Bulletin:{}",
                                            subcategory
                                                .as_ref()
                                                .map(|s| s.to_string())
                                                .unwrap_or_default()
                                        )
                                    }
                                    DispatchCategory::Meta(subcategory) => {
                                        format!(
                                            "Meta:{}",
                                            subcategory
                                                .as_ref()
                                                .map(|s| s.to_string())
                                                .unwrap_or_default()
                                        )
                                    }
                                }
                            ))
                            .unwrap_or_default(),
                        sort.as_ref()
                            .map(|s| format!("&dispatchsort={s}"))
                            .unwrap_or_default(),
                    )
                }

                WorldShard::Dispatch(id) => {
                    format!("dispatch&dispatchid={id}")
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
                    format!(
                        "happenings{}{}{}{}{}{}{}",
                        view.as_ref()
                            .map(|kind| match kind {
                                Nation(entities) | Region(entities) => {
                                    (!entities.is_empty())
                                        .then(|| {
                                            entities.iter().fold(
                                                format!(
                                                    "&view={}",
                                                    match kind {
                                                        Nation(..) => "nation.",
                                                        Region(..) => "region.",
                                                    }
                                                ),
                                                |acc, nation| format!("{acc},{nation}"),
                                            )
                                        })
                                        .unwrap_or_default()
                                }
                            })
                            .unwrap_or_default(),
                        filter
                            .as_ref()
                            .map(|filters| filters
                                .iter()
                                .fold("&filter=".to_string(), |acc, kind| format!("{acc}+{kind}")))
                            .unwrap_or_default(),
                        limit
                            .as_ref()
                            .map(|x| format!("&limit={x}"))
                            .unwrap_or_default(),
                        before_id
                            .as_ref()
                            .map(|x| format!("&beforeid={x}"))
                            .unwrap_or_default(),
                        since_id
                            .as_ref()
                            .map(|x| format!("&sinceid={x}"))
                            .unwrap_or_default(),
                        before_time
                            .as_ref()
                            .map(|x| format!("&beforetime={x}"))
                            .unwrap_or_default(),
                        since_time
                            .as_ref()
                            .map(|x| format!("&sincetime={x}"))
                            .unwrap_or_default(),
                    )
                }
                WorldShard::RegionsByTag(complex_tags) => {
                    complex_tags
                        .iter()
                        .fold("regionsbytag&tags=".to_string(), |acc, tag| {
                            format!("{acc},{tag}")
                        })
                }
                other_shard => {
                    format!("{:?}", other_shard).to_lowercase()
                }
            }
        )
    }
}

#[derive(Default)]
pub struct HappeningsShardBuilder {
    view: Option<HappeningsViewType>,
    filter: Vec<HappeningsFilterType>,
    limit: Option<u32>,
    since_id: Option<u32>,
    before_id: Option<u32>,
    since_time: Option<u64>,
    before_time: Option<u64>,
}

impl HappeningsShardBuilder {
    pub fn new() -> Self {
        HappeningsShardBuilder::default()
    }

    pub fn view_nation(mut self, nation: &str) -> Self {
        self.view = Some(Nation(vec![nation.to_string()]));
        self
    }

    pub fn view_nations(mut self, nations: &[&str]) -> Self {
        self.view = Some(Nation(
            nations.iter().map(|nation| nation.to_string()).collect(),
        ));
        self
    }

    pub fn view_region(mut self, region: &str) -> Self {
        self.view = Some(Region(vec![region.to_string()]));
        self
    }

    pub fn view_regions(mut self, regions: &[&str]) -> Self {
        self.view = Some(Region(
            regions.iter().map(|region| region.to_string()).collect(),
        ));
        self
    }

    pub fn set_filters(mut self, filters: &[HappeningsFilterType]) -> Self {
        self.filter = filters.to_vec();
        self
    }

    pub fn add_filter(self, filter: HappeningsFilterType) -> Self {
        self.add_filters(&[filter])
    }

    pub fn add_filters(mut self, filters: &[HappeningsFilterType]) -> Self {
        filters
            .iter()
            .for_each(|filter| self.filter.push(filter.clone()));
        self
    }

    pub fn limit(mut self, max_results: u32) -> Self {
        self.limit = Some(max_results);
        self
    }

    pub fn since_id(mut self, id: u32) -> Self {
        self.since_id = Some(id);
        self
    }

    pub fn before_id(mut self, id: u32) -> Self {
        self.before_id = Some(id);
        self
    }

    pub fn since_time(mut self, timestamp: u64) -> Self {
        self.since_time = Some(timestamp);
        self
    }

    pub fn before_time(mut self, timestamp: u64) -> Self {
        self.before_time = Some(timestamp);
        self
    }

    pub fn build(self) -> WorldShard {
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

#[derive(Debug)]
pub struct BannerId {}

impl_display_as_debug!(BannerId);

#[derive(Debug)]
pub enum SortType {
    New,
    Best,
}

impl_display_as_debug!(SortType);

#[derive(Debug)]
pub enum DispatchCategory {
    Factbook(Option<FactbookCategory>),
    Bulletin(Option<BulletinCategory>),
    Account(Option<AccountCategory>),
    Meta(Option<MetaCategory>),
}

#[derive(Debug)]
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
}

#[derive(Debug)]
pub enum BulletinCategory {
    Policy,
    News,
    Opinion,
    Campaign,
}

#[derive(Debug)]
pub enum AccountCategory {
    Military,
    Trade,
    Sport,
    Drama,
    Diplomacy,
    Science,
    Culture,
    Other,
}

#[derive(Debug)]
pub enum MetaCategory {
    Gameplay,
    Reference,
}

impl_display_as_debug!(DispatchCategory);
impl_display_as_debug!(FactbookCategory);
impl_display_as_debug!(BulletinCategory);
impl_display_as_debug!(AccountCategory);
impl_display_as_debug!(MetaCategory);

#[derive(Debug)]
pub enum HappeningsViewType {
    Nation(Vec<String>),
    Region(Vec<String>),
}

#[derive(Debug, Clone)]
pub enum HappeningsFilterType {
    Law,
    Change,
    Dispatch,
    Rmb,
    Embassy,
    Eject,
    Admin,
    Move,
    Founding,
    Cte,
    Vote,
    Resolution,
    Member,
    Endo,
}

impl Display for HappeningsFilterType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", format!("{:?}", self).to_lowercase())
    }
}

#[derive(Debug)]
pub enum IncludeOrExcludeTag {
    Include(Tag),
    Exclude(Tag),
}

impl Display for IncludeOrExcludeTag {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                IncludeOrExcludeTag::Include(tag) => {
                    format!("{:?}", tag)
                }
                IncludeOrExcludeTag::Exclude(tag) => {
                    format!("-{:?}", tag)
                }
            }
        )
    }
}

#[derive(Debug)]
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

#[doc(hidden)]
pub(crate) fn format_census_ranks(scale: &Option<CensusScales>, start: &Option<u32>) -> String {
    format!(
        "censusranks{}{}",
        format_census_scale(scale),
        start
            .as_ref()
            .map(|x| format!("&start={x}"))
            .unwrap_or_default()
    )
}
