//! For public nation shard requests.

use crate::shards::{CensusShard, NSRequest, Params, RequestBuildError, BASE_URL};
use itertools::Itertools;
use strum::AsRefStr;
use url::Url;

/// A nation request available to anyone (no login required).
///
/// Each request "shard"
/// is associated with a certain field in its associated parsed type,
/// [`Nation`](crate::parsers::nation::Nation).
/// Enum variant docs include the struct field associated with it.
//noinspection SpellCheckingInspection
#[derive(AsRefStr, Clone, Debug, PartialEq)]
pub enum PublicNationShard<'a> {
    /// A randomly-selected compliment for the nation.
    ///
    /// Nation field: [`admirable`](crate::parsers::nation::Nation.admirable)
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
    Banner,
    /// Returns a list of Rift banners that should be displayed:
    /// the nation's primary banner (if any) is always listed first,
    /// with the remainder in random order.
    /// Banner codes can be converted into image URLs by prepending `/images/banners/`
    /// and appending `.jpg`.
    Banners,
    /// The capital if a custom capital was chosen, or the nation name with "City"
    /// appended at the end if one has not been chosen yet.
    ///
    // /// See also: [`PublicNationShard::CustomCapital`]
    #[strum(serialize = "customcapital")]
    Capital,
    /// One of the 27 national classifications that the game assigns based on personal,
    /// economic, and political freedom.
    Category,
    /// By default, returns the score, rank, and region rank on today's featured World Census scale.
    /// Can be optionally configured with additional parameters.
    ///
    /// Parallels [`WorldShard::Census`](crate::shards::world::WorldShard::Census).
    Census(CensusShard<'a>),
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
    /// *Note that in the English language,
    /// the word "some" (plural indefinite article) is not normally used in that way;
    /// however it would be more inaccurate to use "the" (plural definite article).
    /// Other languages use articles differently than English.*
    ///
    /// *It should also be noted that the words "Frenchman" and "Frenchmen"
    /// are no longer the preferred English words to describe French people;
    /// the adjectival demonym with the words "person" or "people" is now preferred:
    /// e.g. I am a French (adj.) person.*
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
    /// and political freedom within the country using *categorical* descriptors.
    Freedom,
    /// Describes civil rights, the economy,
    /// and political freedom within the country using *numerical* descriptors.
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
    #[strum(serialize = "customleader")]
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
    #[strum(serialize = "customreligion")]
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
    /// Whether a recruitment telegram will be blocked by the nation's telegram settings.
    TGCanRecruit {
        /// Whether the nation will deny a recruitment telegram from this region in particular due to having received one too recently.
        from: Option<&'a str>,
    },
    /// Whether a campaign telegram will be blocked by the nation's telegram settings.
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

/// A request of the public nation API.
/// If you're going to make a request, start here!
/// ## Example
/// ```rust
/// # use crustacean_states::shards::nation::{PublicNationRequest, PublicNationShard};
/// let request = PublicNationRequest::new_with_shards(
///     "Aramos",
///     vec![PublicNationShard::Capital],
/// );
/// ```
#[derive(Clone, Debug, Default, PartialEq)]
pub struct PublicNationRequest<'a> {
    nation: Option<&'a str>,
    shards: Vec<PublicNationShard<'a>>,
}

impl<'a> PublicNationRequest<'a> {
    /// Creates a new builder given a nation name.
    ///
    /// If you do not modify the shards on this request,
    /// you will get a default response using the "standard public nation API shard set".
    /// See [`StandardPublicNationRequest`] for more information.
    pub fn new(nation: &'a str) -> Self {
        Self {
            nation: Some(nation),
            shards: vec![],
        }
    }

    /// Create a new request.
    pub fn new_with_shards<T>(nation: &'a str, shards: T) -> Self
    where
        T: AsRef<[PublicNationShard<'a>]>,
    {
        Self {
            nation: Some(nation),
            shards: shards.as_ref().to_vec(),
        }
    }

    /// Creates a new builder given shards but no nation name.
    ///
    /// Warning: a nation name must be provided before building!
    pub fn with_shards(shards: Vec<PublicNationShard<'a>>) -> Self {
        Self {
            nation: None,
            shards,
        }
    }

    /// Sets the nation for the request.
    pub fn nation(&mut self, nation: &'a str) -> &mut Self {
        self.nation = Some(nation);
        self
    }

    /// Modify shards using a function.
    ///
    /// ## Example
    /// ```rust
    /// # use crustacean_states::shards::nation::{PublicNationRequest, PublicNationShard};
    /// let mut request_builder = PublicNationRequest::new("Aramos");
    /// request_builder.shards(|s| {
    ///     s.push(PublicNationShard::Capital);
    /// });
    /// assert_eq!(
    ///     request_builder,
    ///     PublicNationRequest::new_with_shards(
    ///         "Aramos",
    ///         vec![PublicNationShard::Capital]
    ///     ),
    /// );
    /// ```
    pub fn shards<F>(&mut self, f: F) -> &mut Self
    where
        F: FnOnce(&mut Vec<PublicNationShard<'a>>),
    {
        f(&mut self.shards);
        self
    }

    /// Add a shard.
    ///
    /// ## Example
    /// ```rust
    /// # use crustacean_states::shards::nation::{
    /// #   PublicNationRequest, PublicNationShard
    /// # };
    /// let mut request_builder = PublicNationRequest::new("Aramos");
    /// request_builder.add_shard(PublicNationShard::Capital);
    /// assert_eq!(
    ///     request_builder,
    ///     PublicNationRequest::new_with_shards(
    ///         "Aramos",
    ///         vec![PublicNationShard::Capital],
    ///     ),
    /// );
    /// ```
    pub fn add_shard(&mut self, shard: PublicNationShard<'a>) -> &mut Self {
        self.shards.push(shard);
        self
    }

    /// Add multiple shards.
    /// Note that the shards can be in any form of iterator, not just a `Vec`.
    ///
    /// ## Example
    /// ```rust
    /// # use std::error::Error;
    /// # use crustacean_states::shards::RequestBuildError;
    /// # use crustacean_states::shards::nation::{
    /// #    PublicNationRequest,
    /// #    PublicNationShard,
    /// # };
    /// # fn test() -> Result<(), Box<dyn Error>> {
    /// let mut request_builder = PublicNationRequest::new("Aramos");
    /// request_builder.add_shards(
    ///     [PublicNationShard::Capital, PublicNationShard::Animal]
    /// );
    /// assert_eq!(
    ///     request_builder,
    ///     PublicNationRequest::new_with_shards(
    ///         "Aramos",
    ///         vec![
    ///             PublicNationShard::Capital,
    ///             PublicNationShard::Animal
    ///         ],
    ///     ),
    /// );
    /// # Ok(())
    /// # }
    /// ```
    pub fn add_shards<I>(&mut self, shards: I) -> &mut Self
    where
        I: IntoIterator<Item = PublicNationShard<'a>>,
    {
        self.shards.extend(shards);
        self
    }
}

impl<'a> NSRequest for PublicNationRequest<'a> {
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
            PublicNationShard::Census(CensusShard { scale, modes }) => {
                params.insert_scale(scale).insert_modes(modes);
            }
            PublicNationShard::TGCanCampaign { from }
            | PublicNationShard::TGCanRecruit { from } => {
                params.insert_on("from", from);
            }
            _ => {} // no other public nation shards require parameters
        });

        Url::parse_with_params(
            BASE_URL,
            params.insert_front("q", query).insert_front(
                "nation",
                self.nation
                    .ok_or(RequestBuildError::MissingParam("nation"))?,
            ),
        )
        .map_err(RequestBuildError::UrlParse)
    }
}

/// A "standard" public nation API request.
/// Avoid this type if you only want certain information about a nation.
///
/// The associated parser type for this requester is
/// [`StandardNation`](crate::parsers::nation::StandardNation).
/// Parsing with [`Nation`](crate::parsers::nation::Nation)
/// is also possible, but it will almost definitely be slower.
///
/// What does "standard" mean?
/// NationStates will return certain information by default,
/// as if you had requested a certain set of shards.
/// Those shards are:
/// [`Name`](PublicNationShard::Name), [`Type`](PublicNationShard::Type),
/// [`FullName`](PublicNationShard::FullName), [`Motto`](PublicNationShard::Motto),
/// [`Category`](PublicNationShard::Category), [`WA`](PublicNationShard::WA),
/// [`Endorsements`](PublicNationShard::Endorsements), [`Answered`](PublicNationShard::Answered),
/// [`Freedom`](PublicNationShard::Freedom), [`Region`](PublicNationShard::Region),
/// [`Population`](PublicNationShard::Population), [`Tax`](PublicNationShard::Tax),
/// [`Animal`](PublicNationShard::Animal), [`Currency`](PublicNationShard::Currency),
/// [`Demonym`](PublicNationShard::Demonym), [`Demonym2`](PublicNationShard::Demonym2),
/// [`Demonym2Plural`](PublicNationShard::Demonym2Plural), [`Flag`](PublicNationShard::Flag),
/// [`MajorIndustry`](PublicNationShard::MajorIndustry),
/// [`GovtPriority`](PublicNationShard::GovtPriority), [`Govt`](PublicNationShard::Govt),
/// [`Founded`](PublicNationShard::Founded), [`FirstLogin`](PublicNationShard::FirstLogin),
/// [`LastLogin`](PublicNationShard::LastLogin), [`LastActivity`](PublicNationShard::LastActivity),
/// [`Influence`](PublicNationShard::Influence),
/// [`FreedomScores`](PublicNationShard::FreedomScores),
/// [`PublicSector`](PublicNationShard::PublicSector), [`Deaths`](PublicNationShard::Deaths),
/// [`Leader`](PublicNationShard::Leader), [`Capital`](PublicNationShard::Capital),
/// [`Religion`](PublicNationShard::Religion), [`Factbooks`](PublicNationShard::Factbooks), and
/// [`Dispatches`](PublicNationShard::Dispatches).
///
pub struct StandardPublicNationRequest<'a>(&'a str);

impl<'a> StandardPublicNationRequest<'a> {
    /// Create a new standard public nation request of the provided nation.
    pub fn new(nation: &'a str) -> Self {
        Self(nation)
    }
}

impl<'a> NSRequest for StandardPublicNationRequest<'a> {
    fn as_url(&self) -> Result<Url, RequestBuildError> {
        Url::parse_with_params(BASE_URL, [("nation", self.0)]).map_err(RequestBuildError::UrlParse)
    }
}

#[cfg(test)]
mod tests {
    use crate::shards::nation::PublicNationShard;
    use crate::shards::{CensusCurrentMode, CensusModes, CensusScales, CensusShard};

    #[test]
    fn pns_normal_as_str() {
        let shard = PublicNationShard::Happenings;
        assert_eq!(shard.as_ref(), "Happenings");
    }

    #[test]
    fn pns_complex_as_str() {
        let shard = PublicNationShard::Census(CensusShard::new(
            CensusScales::Today,
            CensusModes::from([CensusCurrentMode::Score].as_ref()),
        ));
        assert_eq!(shard.as_ref(), "Census")
    }

    #[test]
    fn add_shards() -> Result<(), crate::shards::RequestBuildError> {
        let mut request_builder = crate::shards::nation::PublicNationRequest::new("Aramos");
        request_builder.add_shards([PublicNationShard::Capital, PublicNationShard::Animal]);
        assert_eq!(request_builder.nation, Some("Aramos"));
        assert_eq!(
            request_builder.shards,
            vec![PublicNationShard::Capital, PublicNationShard::Animal]
        );
        Ok(())
    }
}
