//! For region shard requests.
use crate::shards::{
    CensusRanksShard, CensusShard, NSRequest, Params, RequestBuildError, BASE_URL,
};
use itertools::Itertools;
use std::num::{NonZeroU32, NonZeroU8};
use strum::AsRefStr;
use url::Url;

/// A request of a region.
#[derive(AsRefStr, Clone, Debug)]
pub enum RegionShard<'a> {
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
    ///
    /// Parallels [`PublicNationShard::Census`][crate::shards::nation::PublicNationShard::Census].
    Census(CensusShard<'a>),
    /// Information on how nations in the region rank according to the World Census.
    ///
    /// Parallels [`WorldShard::CensusRanks`][crate::shards::world::WorldShard::CensusRanks].
    CensusRanks(CensusRanksShard),
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
    Messages(RmbShard),
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

#[derive(Clone, Debug, Default)]
pub struct RmbShard {
    /// Return this many messages. Must be in the range 1-100.
    limit: Option<NonZeroU8>,
    /// Skip the most recent (number) messages. Begin back farther.
    offset: Option<NonZeroU32>,
    /// Instead of returning the most recent messages, return messages starting from this post ID.
    starting_post: Option<NonZeroU32>,
}

impl RmbShard {
    pub fn new(limit: u8, offset: u32, starting_post: u32) -> Self {
        Self::default()
            .limit(limit)
            .offset(offset)
            .starting_post(starting_post)
            .to_owned()
    }

    pub fn limit(&mut self, x: u8) -> &mut Self {
        self.limit = NonZeroU8::new(x);
        self
    }

    pub fn offset(&mut self, x: u32) -> &mut Self {
        self.offset = NonZeroU32::new(x);
        self
    }

    pub fn starting_post(&mut self, x: u32) -> &mut Self {
        self.starting_post = NonZeroU32::new(x);
        self
    }
}

#[derive(Clone, Debug, Default)]
pub struct RegionRequest<'a> {
    region: Option<&'a str>,
    shards: Vec<RegionShard<'a>>,
}

impl<'a> RegionRequest<'a> {
    pub fn new_empty(region: &'a str) -> Self {
        Self {
            region: Some(region),
            shards: vec![],
        }
    }

    pub fn new<T>(region: &'a str, shards: &'a T) -> Self
    where
        T: AsRef<[RegionShard<'a>]>,
    {
        Self {
            region: Some(region),
            shards: shards.as_ref().to_vec(),
        }
    }

    pub fn new_standard(region: &'a str) -> Self {
        Self {
            region: Some(region),
            shards: vec![],
        }
    }

    pub fn with_shards(shards: Vec<RegionShard<'a>>) -> Self {
        Self {
            region: None,
            shards,
        }
    }

    pub fn region(&mut self, region: &'a str) -> &mut Self {
        self.region = Some(region);
        self
    }

    pub fn shards<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Vec<RegionShard>),
    {
        f(&mut self.shards);
        self
    }

    pub fn add_shard(&mut self, shard: RegionShard<'a>) -> &mut Self {
        self.shards.push(shard);
        self
    }

    pub fn add_shards<I>(&mut self, shards: I) -> &mut Self
    where
        I: IntoIterator<Item = RegionShard<'a>>,
    {
        self.shards.extend(shards);
        self
    }

    pub fn build(&self) -> Result<RegionRequest, RequestBuildError> {
        Ok(RegionRequest::new(
            self.region
                .ok_or(RequestBuildError::MissingParam("region"))?,
            &self.shards,
        ))
    }
}

impl<'a> NSRequest for RegionRequest<'a> {
    //noinspection SpellCheckingInspection
    fn as_url(&self) -> Result<Url, RequestBuildError> {
        let query = self
            .shards
            .iter()
            .map(|s| s.as_ref())
            .join("+")
            .to_ascii_lowercase();
        let mut params = Params::default();
        self.shards.iter().for_each(|s| match s {
            RegionShard::Census(CensusShard { scale, modes }) => {
                params.insert_scale(scale).insert_modes(modes);
            }
            RegionShard::CensusRanks(CensusRanksShard { scale, start }) => {
                params.insert_rank_scale(scale).insert_start(start);
            }
            RegionShard::Messages(RmbShard {
                limit,
                offset,
                starting_post,
            }) => {
                params
                    .insert_on("limit", limit)
                    .insert_on("offset", offset)
                    .insert_on("fromid", starting_post);
            }
            _ => {}
        });

        Url::parse_with_params(
            BASE_URL,
            params.insert_front("q", query).insert_front(
                "region",
                self.region
                    .ok_or(RequestBuildError::MissingParam("region"))?,
            ),
        )
        .map_err(|e| RequestBuildError::UrlParse { source: e })
    }
}

//noinspection SpellCheckingInspection
#[derive(Debug)]
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
