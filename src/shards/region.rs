//! For region shard requests.
use crate::shards::{CensusRanksShard, CensusShard, NSRequest, Params, BASE_URL};
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

/// A builder for the [`RegionShard::Messages`] shard.
///
/// Be aware the default behavior is for the number of messages to be 20,
/// ending at the most recent message.
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
    /// Return this many messages. Must be in the range 1-100.
    ///
    /// This sets a *maximum* number of messages.
    /// If there are not enough messages based on other parameters
    /// (e.g. using [`starting_post`](RmbShard::starting_post) on a recent post),
    /// the website will return as many messages as it can.
    pub fn limit(&mut self, x: u8) -> &mut Self {
        self.limit = NonZeroU8::try_from(x).ok();
        self
    }

    /// Skip the `x` most recent messages.
    /// Begin further in the past.
    pub fn offset(&mut self, x: u32) -> &mut Self {
        self.offset = NonZeroU32::try_from(x).ok();
        self
    }

    /// Instead of returning the most recent messages, return messages starting from this post ID.
    pub fn starting_post(&mut self, post_id: u32) -> &mut Self {
        self.starting_post = NonZeroU32::try_from(post_id).ok();
        self
    }
}

/// Make a request of the region API.
///
/// ## Example
/// ```rust
/// # use crustacean_states::client::Client;
/// # use crustacean_states::shards::region::{RegionRequest, RegionShard};
/// # use std::error::Error;
/// # async fn test() -> Result<(), Box<dyn Error>> {
/// # let client = Client::new("");
/// let request = RegionRequest::new_with_shards("Anteria", &[RegionShard::NumNations]);
/// let response = client.get(request).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Default)]
pub struct RegionRequest<'a> {
    region: &'a str,
    shards: Vec<RegionShard<'a>>,
}

impl<'a> RegionRequest<'a> {
    pub fn new(region: &'a str) -> Self {
        Self {
            region,
            shards: vec![],
        }
    }

    pub fn new_with_shards<T>(region: &'a str, shards: T) -> Self
    where
        T: AsRef<[RegionShard<'a>]>,
    {
        Self {
            region,
            shards: shards.as_ref().to_vec(),
        }
    }

    pub fn region(&mut self, region: &'a str) -> &mut Self {
        self.region = region;
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
}

impl<'a> NSRequest for RegionRequest<'a> {
    //noinspection SpellCheckingInspection
    fn as_url(&self) -> Url {
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
            params
                .insert_front("q", query)
                .insert_front("region", self.region),
        )
        .unwrap()
    }
}

pub struct StandardRegionRequest<'a>(&'a str);

impl<'a> StandardRegionRequest<'a> {
    pub fn new(region: &'a str) -> Self {
        Self(region)
    }
}

impl<'a> NSRequest for StandardRegionRequest<'a> {
    fn as_url(&self) -> Url {
        Url::parse_with_params(BASE_URL, [("region", self.0)]).unwrap()
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
