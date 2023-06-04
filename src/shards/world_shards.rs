use crate::impl_display_as_debug;
use crate::shards::public_nation_shards::{
    format_census, format_census_scale, CensusModes, CensusScales,
};
use crate::shards::world_shards::HappeningsViewType::{Nation, Region};
use crate::shards::{Params, Shard};
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

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

impl From<WorldShard> for Shard {
    fn from(value: WorldShard) -> Self {
        Self {
            query: Self::name(&value),
            params: {
                let mut param_map = Params::new();
                match &value {
                    WorldShard::Banner(banners) => {
                        param_map.insert("banner".to_string(), banners.iter().join(","));
                    }
                    WorldShard::Census { scale, modes } => {
                        format_census(&mut param_map, scale, modes);
                    }
                    WorldShard::CensusDesc(scale)
                    | WorldShard::CensusScale(scale)
                    | WorldShard::CensusName(scale)
                    | WorldShard::CensusTitle(scale) => {
                        if let Some(s) = scale.as_ref() {
                            param_map.insert("scale".to_string(), s.to_string());
                        }
                    }
                    WorldShard::CensusRanks { scale, start } => {
                        format_census_ranks(&mut param_map, &scale.map(CensusScales::One), start);
                    }
                    WorldShard::Dispatch(id) => {
                        param_map.insert("dispatchid".to_string(), id.to_string());
                    }
                    WorldShard::DispatchList {
                        author,
                        category,
                        sort,
                    } => {
                        if let Some(a) = author.as_ref() {
                            param_map.insert("dispatchauthor".to_string(), a.to_string());
                        }
                        if let Some(c) = category.as_ref() {
                            param_map.insert(
                                "dispatchcategory".to_string(),
                                format!(
                                    "{}{}",
                                    match c {
                                        DispatchCategory::Factbook(_) => "Factbook",
                                        DispatchCategory::Bulletin(_) => "Bulletin",
                                        DispatchCategory::Account(_) => "Account",
                                        DispatchCategory::Meta(_) => "Meta",
                                    },
                                    match c {
                                        DispatchCategory::Factbook(subcategory) => {
                                            subcategory.as_ref().map(|s| s.to_string())
                                        }
                                        DispatchCategory::Bulletin(subcategory) => {
                                            subcategory.as_ref().map(|s| s.to_string())
                                        }
                                        DispatchCategory::Account(subcategory) => {
                                            subcategory.as_ref().map(|s| s.to_string())
                                        }
                                        DispatchCategory::Meta(subcategory) => {
                                            subcategory.as_ref().map(|s| s.to_string())
                                        }
                                    }
                                    .map(|s| format!(":{s}"))
                                    .unwrap_or_default()
                                ),
                            );
                        }
                        if let Some(s) = sort.as_ref() {
                            param_map.insert("dispatchsort".to_string(), s.to_string());
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
                        if let Some(v) = view.as_ref() {
                            param_map.insert(
                                "view".to_string(),
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
                        if let Some(filters) = filter.as_ref() {
                            param_map.insert("filter".to_string(), filters.iter().join("+"));
                        }
                        if let Some(x) = limit.as_ref() {
                            param_map.insert("limit".to_string(), x.to_string());
                        }
                        if let Some(x) = since_id.as_ref() {
                            param_map.insert("sinceid".to_string(), x.to_string());
                        }
                        if let Some(x) = before_id.as_ref() {
                            param_map.insert("beforeid".to_string(), x.to_string());
                        }
                        if let Some(x) = since_time.as_ref() {
                            param_map.insert("sincetime".to_string(), x.to_string());
                        }
                        if let Some(x) = before_time.as_ref() {
                            param_map.insert("beforetime".to_string(), x.to_string());
                        }
                    }
                    WorldShard::RegionsByTag(complex_tags) => {
                        param_map.insert("tags".to_string(), complex_tags.iter().join(","));
                    }
                    _ => {}
                }
                param_map
            },
        }
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

// TODO make banner ids
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
pub(crate) fn format_census_ranks(
    param_map: &mut HashMap<String, String>,
    scale: &Option<CensusScales>,
    start: &Option<u32>,
) {
    format_census_scale(param_map, scale);
    if let Some(s) = start.as_ref() {
        param_map.insert("start".to_string(), s.to_string());
    }
}
