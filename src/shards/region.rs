//! For region shard requests.
use crate::shards::{CensusModes, CensusScales, NSRequest, Params, RequestBuildError, BASE_URL};
use itertools::Itertools;
use strum::AsRefStr;
use url::Url;

/// A request of a region.
#[derive(AsRefStr, Clone, Debug)]
pub enum RegionShard {
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
    /// [source](https://www.nationstates.net/pages/api.html#nationapi-publicshards)
    ///
    /// Parallels [`PublicNationShard::Census`][crate::shards::nation::PublicNationShard::Census].
    Census {
        /// Specify the World Census scale(s) to list, using numerical IDs.
        /// For all scales, use `Some(`[`CensusScales::All`]`)`.
        scale: Option<CensusScales>,
        /// Specify what population the scale should be compared against.
        modes: Option<CensusModes>,
    },
    /// Information on how nations in the region rank according to the World Census.
    ///
    /// Parallels [`WorldShard::CensusRanks`][crate::shards::world::WorldShard::CensusRanks].
    CensusRanks {
        /// The World Census ranking to use. If `None`, returns the day's featured World Census ranking.
        scale: Option<u8>,
        /// The rank at which to start listing (e.g. `Some(1000)` would start at the 1000th nation).
        start: Option<u32>,
    },
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
    Messages {
        /// Return this many messages. Must be in the range 1-100.
        limit: Option<u8>,
        /// Skip the most recent (number) messages.
        offset: Option<u32>,
        /// Instead of returning the most recent messages, return messages starting from this post ID.
        from_id: Option<u32>,
    },
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

pub struct RegionRequest<'a> {
    region: &'a str,
    shards: Vec<RegionShard>,
}

#[derive(Default)]
pub struct RegionRequestBuilder<'a> {
    region: Option<&'a str>,
    shards: Vec<RegionShard>,
}

impl<'a> RegionRequest<'a> {
    fn new(region: &'a str, shards: Vec<RegionShard>) -> Self {
        Self { region, shards }
    }
    fn new_standard(region: &'a str) -> Self {
        Self {
            region,
            shards: vec![],
        }
    }

    fn build() -> RegionRequestBuilder<'a> {
        RegionRequestBuilder {
            region: None,
            shards: vec![],
        }
    }
}

impl<'a> RegionRequestBuilder<'a> {
    fn new(region: &'a str) -> Self {
        Self {
            region: Some(region),
            shards: vec![],
        }
    }
    fn with_shards(shards: Vec<RegionShard>) -> Self {
        Self {
            region: None,
            shards,
        }
    }

    fn region(&mut self, region: &'a str) -> &mut Self {
        self.region = Some(region);
        self
    }
    fn shards<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Vec<RegionShard>) -> Vec<RegionShard>,
    {
        f(&mut self.shards);
        self
    }
    fn add_shard(&mut self, shard: RegionShard) -> &mut Self {
        self.shards.push(shard);
        self
    }
    fn add_shards<T>(&mut self, shards: T) -> &mut Self
    where
        T: IntoIterator<Item = RegionShard>,
    {
        self.shards.extend(shards.into_iter());
        self
    }
    fn set_shards(&mut self, shards: Vec<RegionShard>) -> &mut Self {
        self.shards = shards;
        self
    }

    fn build(&self) -> Result<RegionRequest, RequestBuildError> {
        Ok(RegionRequest::new(
            self.region
                .ok_or_else(|| RequestBuildError::MissingParam("region"))?,
            self.shards.clone(),
        ))
    }
}

impl<'a> From<RegionRequest<'a>> for RegionRequestBuilder<'a> {
    fn from(value: RegionRequest<'a>) -> Self {
        Self {
            region: Some(value.region),
            shards: value.shards,
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
            RegionShard::Census { scale, modes } => {
                params.insert_scale(&scale).insert_modes(&modes);
            }
            RegionShard::CensusRanks { scale, start } => {
                params
                    .insert_scale(&scale.map(CensusScales::One))
                    .insert_start(&start);
            }
            RegionShard::Messages {
                limit,
                offset,
                from_id,
            } => {
                if let Some(l) = limit {
                    params.insert("limit", l.to_string());
                }
                if let Some(o) = offset {
                    params.insert("offset", o.to_string());
                }
                if let Some(f) = from_id {
                    params.insert("fromid", f.to_string());
                }
            }
            _ => {}
        });

        Url::parse_with_params(BASE_URL, {
            let mut p = vec![("region", self.region.to_string()), ("q", query)];
            p.extend(params.drain());
            p
        })
        .unwrap()
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
