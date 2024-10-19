//! For region shard requests.
use std::num::{NonZeroU32, NonZeroU8};

use itertools::Itertools;
use strum::{AsRefStr, Display, EnumString};
use url::Url;

use crate::shards::{CensusRanksShard, CensusShard, NSRequest, Params, BASE_URL};

/// A request of a region.
#[derive(AsRefStr, Clone, Debug, PartialEq)]
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
    /// The history of regional delegates and embassies.
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
#[derive(Clone, Debug, Default, PartialEq)]
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
    /// (e.g., using [`starting_post`](RmbShard::starting_post) on a recent post),
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
/// let request = RegionRequest::from(("Anteria", [RegionShard::NumNations]));
/// let response = client.get(request).await?;
/// # Ok(())
/// # }
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RegionRequest<'a> {
    region: &'a str,
    shards: Vec<RegionShard<'a>>,
}

impl<'a> RegionRequest<'a> {
    /// Creates a new builder given a region name.
    ///
    /// If you do not modify the shards on this request,
    /// you will get a default response using the "standard region API shard set".
    /// See [`StandardRegionRequest`] for more information.
    pub fn new<'b: 'a>(region: &'b str) -> Self {
        Self {
            region,
            shards: vec![],
        }
    }

    /// Sets the region for the request.
    pub fn region(&mut self, region: &'a str) -> &mut Self {
        self.region = region;
        self
    }

    /// Modify shards using a function.
    ///
    /// ## Example
    /// ```rust
    /// # use crustacean_states::shards::region::{RegionRequest, RegionShard};
    /// let mut request_builder = RegionRequest::new("Anteria");
    /// request_builder.shards(|s| {
    ///     s.push(RegionShard::Delegate);
    /// });
    /// assert_eq!(
    ///     request_builder,
    ///     RegionRequest::from((
    ///         "Anteria",
    ///         [RegionShard::Delegate]
    ///     )),
    /// );
    /// ```
    pub fn shards<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Vec<RegionShard<'a>>),
    {
        f(&mut self.shards);
        self
    }

    /// Add a shard.
    ///
    /// ## Example
    /// ```rust
    /// # use crustacean_states::shards::region::{
    /// #   RegionRequest, RegionShard
    /// # };
    /// let mut request_builder = RegionRequest::new("Anteria");
    /// request_builder.add_shard(RegionShard::Delegate);
    /// assert_eq!(
    ///     request_builder,
    ///     RegionRequest::from((
    ///         "Anteria",
    ///         vec![RegionShard::Delegate],
    ///     )),
    /// );
    /// ```
    pub fn add_shard(&mut self, shard: RegionShard<'a>) -> &mut Self {
        self.shards(|v| v.push(shard))
    }

    /// Add multiple shards.
    /// Note that the shards can be in any form of iterator, not just a `Vec`.
    ///
    /// ## Example
    /// ```rust
    /// # use std::error::Error;
    /// # use crustacean_states::shards::region::{
    /// #    RegionRequest,
    /// #    RegionShard,
    /// # };
    /// # fn test() -> Result<(), Box<dyn Error>> {
    /// let mut request_builder = RegionRequest::new("Anteria");
    /// request_builder.add_shards(
    ///     [RegionShard::Delegate, RegionShard::BanList]
    /// );
    /// assert_eq!(
    ///     request_builder,
    ///     RegionRequest::from((
    ///         "Anteria",
    ///         [
    ///             RegionShard::Delegate,
    ///             RegionShard::BanList,
    ///         ],
    ///     )),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_shards<I>(&mut self, shards: I) -> &mut Self
    where
        I: IntoIterator<Item = RegionShard<'a>>,
    {
        self.shards(|v| v.extend(shards))
    }
}

impl<'a, T> From<(&'a str, T)> for RegionRequest<'a>
where
    T: IntoIterator<Item = RegionShard<'a>>,
{
    fn from(value: (&'a str, T)) -> Self {
        Self {
            region: value.0,
            shards: Vec::from_iter(value.1),
        }
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

/// A "standard" region API request.
/// Avoid this type if you only want certain information about a nation.
///
/// What does "standard" mean?
/// NationStates will return certain information by default,
/// as if you had requested a certain set of shards.
/// Those shards are:
/// [`Name`](RegionShard::Name), [`Factbook`](RegionShard::Factbook),
/// [`NumNations`](RegionShard::NumNations),
/// [`Nations`](RegionShard::Nations), [`Delegate`](RegionShard::Delegate),
/// [`DelegateVotes`](RegionShard::DelegateVotes),
/// [`DelegateAuth`](RegionShard::DelegateAuth),
/// [`Frontier`](RegionShard::Frontier),
/// [`Founder`](RegionShard::Founder), [`Governor`](RegionShard::Governor),
/// [`Officers`](RegionShard::Officers), [`Power`](RegionShard::Power), [`Flag`](RegionShard::Flag),
/// [`Banner`](RegionShard::Banner), [`BannerUrl`](RegionShard::BannerUrl),
/// [`Embassies`](RegionShard::Embassies), [`WABadges`](RegionShard::WABadges),
/// [`LastUpdate`](RegionShard::LastUpdate), [`LastMajorUpdate`](RegionShard::LastMajorUpdate), and
/// [`LastMinorUpdate`](RegionShard::LastMinorUpdate).
pub struct StandardRegionRequest<'a>(&'a str);

impl<'a> StandardRegionRequest<'a> {
    /// Create a new standard region request.
    pub fn new(region: &'a str) -> Self {
        Self(region)
    }
}

impl<'a> NSRequest for StandardRegionRequest<'a> {
    fn as_url(&self) -> Url {
        Url::parse_with_params(BASE_URL, [("region", self.0)]).unwrap()
    }
}

/// All the tags a region can have.
///
/// This list is non-exhaustive as new tags are added on occasion by NationStates.
///
/// Tags that a region may freely assign and unassign do not have a fixed meaning,
/// so they are only marked with (self-tag).
///
/// Some tags have been given added clarity in their variant name, and in those cases,
/// their original name is also documented.
//noinspection SpellCheckingInspection
#[derive(Clone, Debug, PartialEq, EnumString, Display)]
#[non_exhaustive]
#[allow(missing_docs)]
#[strum(ascii_case_insensitive)]
pub enum Tag {
    /// (self-tag)
    Anarchist,
    /// (self-tag)
    Anime,
    /// (self-tag)
    #[strum(serialize = "anti-capitalist")]
    AntiCapitalist,
    /// (self-tag)
    #[strum(serialize = "anti-communist")]
    AntiCommunist,
    /// (self-tag)
    #[strum(serialize = "anti-fascist")]
    AntiFascist,
    /// (self-tag)
    #[strum(serialize = "anti-general_assembly")]
    AntiGeneralAssembly,
    /// (self-tag)
    #[strum(serialize = "anti-security_council")]
    AntiSecurityCouncil,
    /// (self-tag)
    #[strum(serialize = "anti-world_assembly")]
    AntiWorldAssembly,
    /// (self-tag)
    Capitalist,
    /// (self-tag)
    Casual,
    /// Nations kicked out of another region go here.
    /// Tag is only given to
    /// [The Rejected Realms](https://www.nationstates.net/region=the_rejected_realms).
    Catcher,
    /// Part of the NationStates for Education project -- cannot be interacted with like other regions.
    Class,
    /// (self-tag)
    Colony,
    /// Target of a Security Council Commendation resolution.
    Commended,
    /// (self-tag)
    Communist,
    /// Target of a Security Council Condemnation resolution.
    Condemned,
    /// (self-tag)
    Conservative,
    /// (self-tag)
    Cyberpunk,
    /// (self-tag)
    Defender,
    /// (self-tag)
    Democratic,
    /// (self-tag)
    #[strum(serialize = "eco-friendly")]
    EcoFriendly,
    /// (self-tag)
    Egalitarian,
    /// (self-tag)
    #[strum(serialize = "embassy_collector")]
    EmbassyCollector,
    /// Has 101-500 nations.
    Enormous,
    /// (self-tag)
    Fandom,
    /// (self-tag)
    #[strum(serialize = "fantasy_tech")]
    FantasyTech,
    /// (self-tag) Eww.
    Fascist,
    /// Featured once (or more!) on the World page.
    Featured,
    /// Game-created, originates the majority of nations.
    /// Has five regions:
    /// [The North Pacific](https://www.nationstates.net/region=the_north_pacific),
    /// [The South Pacific](https://www.nationstates.net/region=the_south_pacific),
    /// [The East Pacific](https://www.nationstates.net/region=the_east_pacific),
    /// [The West Pacific](https://www.nationstates.net/region=the_west_pacific),
    /// and [The Pacific](https://www.nationstates.net/region=the_pacific).
    Feeder,
    /// (self-tag)
    Feminist,
    /// (self-tag) NationStates Forums, F7 section.
    #[strum(serialize = "f7er")]
    ForumSevener,
    /// Has a founder that ceased to exist.
    Founderless,
    /// (self-tag)
    #[strum(serialize = "free_trade")]
    FreeTrade,
    /// New nations may spawn (although not as fast as in a [`Feeder`](Tag::Feeder)).
    Frontier,
    /// (self-tag)
    #[strum(serialize = "future_tech")]
    FutureTech,
    /// (self-tag) Shortened to FT: FTL on-site.
    #[strum(serialize = "ft_ftl")]
    FutureTechFasterThanLight,
    /// (self-tag) Shortened to FT: FTLi on-site.
    #[strum(serialize = "ft_ftli")]
    FutureTechFasterThanLightInhibited,
    /// (self-tag) Shortened to FT: STL on-site.
    #[strum(serialize = "ft_stl")]
    FutureTechSlowerThanLight,
    /// (self-tag)
    #[strum(serialize = "game_player")]
    GamePlayer,
    /// Has 501+ nations.
    Gargantuan,
    /// (self-tag)
    #[strum(serialize = "general_assembly")]
    GeneralAssembly,
    /// (self-tag) NationStates Forums, General section.
    Generalite,
    /// Does not have a governor.
    /// Regions in this state will always have an executive World Assembly Delegate.
    Governorless,
    /// (self-tag)
    #[strum(serialize = "human-only")]
    HumanOnly,
    /// (self-tag)
    Imperialist,
    /// (self-tag)
    Independent,
    /// (self-tag)
    Industrial,
    /// Forbidden to convert to or from a Frontier, as resolved by the World Assembly Security Council.
    Injuncted,
    /// (self-tag)
    #[strum(serialize = "international_federalist")]
    InternationalFederalist,
    /// (self-tag)
    Invader,
    /// (self-tag)
    Isolationist,
    /// (self-tag)
    IssuesPlayer,
    /// (self-tag)
    JumpPoint,
    /// (self-tag)
    Lgbt,
    /// Has 51-100 nations.
    Large,
    /// (self-tag)
    Liberal,
    /// Forbidden from having a password, as resolved by the World Assembly Security Council.
    Liberated,
    /// (self-tag)
    Libertarian,
    /// (self-tag)
    Magical,
    /// (self-tag)
    Map,
    /// Has 11-50 nations.
    Medium,
    /// (self-tag)
    Mercenary,
    /// Has 1-5 nations.
    Miniscule,
    /// (self-tag)
    #[strum(serialize = "modern_tech")]
    ModernTech,
    /// (self-tag)
    Monarchist,
    /// (self-tag)
    #[strum(serialize = "multi-species")]
    MultiSpecies,
    /// (self-tag)
    #[strum(serialize = "national_sovereigntist")]
    NationalSovereigntist,
    /// (self-tag)
    Neutral,
    /// Created in the last week.
    New,
    /// (self-tag)
    #[strum(serialize = "non-english")]
    NonEnglish,
    /// (self-tag)
    #[strum(serialize = "offsite_chat")]
    OffsiteChat,
    /// (self-tag)
    #[strum(serialize = "offsite_forums")]
    OffsiteForums,
    /// (self-tag)
    #[strum(serialize = "outer_space")]
    OuterSpace,
    /// (self-tag) NationStates Forums, Portal To The Multiverse section.
    #[strum(serialize = "p2tm")]
    PortalToTheMultiverse,
    /// (self-tag)
    Pacifist,
    /// (self-tag)
    Parody,
    /// Has a password preventing free entry into the region.
    Password,
    /// (self-tag)
    #[strum(serialize = "past_tech")]
    PastTech,
    /// (self-tag)
    Patriarchal,
    /// (self-tag)
    #[strum(serialize = "post_apocalyptic")]
    PostApocalyptic,
    /// (self-tag)
    #[strum(serialize = "post-modern_tech")]
    PostModernTech,
    /// (self-tag)
    #[strum(serialize = "puppet_storage")]
    PuppetStorage,
    /// (self-tag)
    #[strum(serialize = "regional_government")]
    RegionalGovernment,
    /// (self-tag)
    Religious,
    /// Where nations get revived.
    /// Has three regions: [Lazarus](https://www.nationstates.net/region=lazarus),
    /// [Balder](https://www.nationstates.net/region=balder),
    /// and [Osiris](https://www.nationstates.net/region=osiris).
    Restorer,
    /// (self-tag)
    #[strum(serialize = "role_player")]
    RolePlayer,
    /// (self-tag)
    #[strum(serialize = "security_council")]
    SecurityCouncil,
    /// (self-tag)
    Serious,
    /// (self-tag)
    Silly,
    /// This tag got split into [`Catcher`](Tag::Catcher)
    /// and [`Restorer`](Tag::Restorer)
    /// because they perform very different roles.
    /// Contains the union of those two tags.
    Sinker,
    /// Has 6-10 nations.
    Small,
    /// (self-tag)
    Snarky,
    /// (self-tag)
    Social,
    /// (self-tag)
    Socialist,
    /// (self-tag)
    Sports,
    /// (self-tag)
    Steampunk,
    /// (self-tag)
    Surreal,
    /// (self-tag)
    Theocratic,
    /// (self-tag)
    Totalitarian,
    /// (self-tag)
    #[strum(serialize = "trading_cards")]
    TradingCards,
    /// (self-tag)
    #[strum(serialize = "video_game")]
    VideoGame,
    /// Game-created region where the bans are temporary and the coups are plenty.
    /// Currently, there are seven such regions:
    /// [Warzone Africa](https://www.nationstates.net/region=warzone_africa),
    /// [Warzone Airspace](https://www.nationstates.net/region=warzone_airspace),
    /// [Warzone Asia](https://www.nationstates.net/region=warzone_asia),
    /// [Warzone Australia](https://www.nationstates.net/region=warzone_australia),
    /// [Warzone Europe](https://www.nationstates.net/region=warzone_europe),
    /// [Warzone Sandbox](https://www.nationstates.net/region=warzone_sandbox),
    /// and [Warzone Trinidad](https://www.nationstates.net/region=warzone_trinidad).
    Warzone,
    /// (self-tag)
    #[strum(serialize = "world_assembly")]
    WorldAssembly,
}
