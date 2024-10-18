use crate::models::dispatch::DispatchId;
use crate::parsers::happenings::Happenings;
use crate::parsers::nation::IntoNationError;
use crate::parsers::region::{
    Embassy, EmbassyKind, MaybeMessageContents, Message, Region, RegionWAVote,
};
use crate::parsers::{
    into_datetime,
    region::{IntoRegionError, Officer, OfficerAuthority},
    CensusCurrentData, CensusData, CensusHistoricalData, CensusRegionRanks, MaybeRelativeTime,
    MaybeSystemTime, ParsingError, RawCensus, RawCensusRanks, RawHappenings,
};
use chrono::{DateTime, Utc};
use itertools::Itertools;
use serde::Deserialize;

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawRegion {
    // default shards
    name: Option<String>,          // nice name
    factbook: Option<String>,      // contains factbook
    numnations: Option<u32>,       // number of nations inside
    nations: Option<String>,       // colon-separated list of nations
    delegate: Option<String>,      // internal name of delegate
    delegatevotes: Option<u32>,    // number of votes delegate has in World Assembly
    delegateauth: Option<String>,  // authorities that delegate has
    frontier: Option<u8>,          // TODO understand
    founder: Option<String>,       // name of the nation that founded the region
    governor: Option<String>,      // name of the nation that is governor
    officers: Option<RawOfficers>, // list of officers
    power: Option<String>,         // regional power level
    flag: Option<String>,          // URL to region's flag
    banner: Option<u32>,           // region's banner ID
    bannerurl: Option<String>,     // incomplete URL to banner.
    // appears to not have nationstates.net at the beginning
    embassies: Option<RawEmbassies>, // list of region's embassies
    // END default
    banned: Option<String>, // who is banned? separated by colons, internal name
    bannerby: Option<String>, // who made the banner?
    census: Option<RawCensus>,
    censusranks: Option<RawCensusRanks>,
    dbid: Option<u32>,
    dispatches: Option<String>, // list of IDs of pinned dispatches, comma separated
    embassyrmb: Option<String>, // permissions given for embassies posting on the RMB TODO find all
    founded: Option<String>,    // relative time since the region was founded
    foundedtime: Option<i64>,   // UNIX timestamp when the region was founded
    gavote: Option<RawRegionWAVote>,
    happenings: Option<RawHappenings>,
    history: Option<RawHappenings>,
    lastupdate: Option<i64>,
    lastmajorupdate: Option<i64>,
    lastminorupdate: Option<i64>,
    messages: Option<RawMessages>,
    unnations: Option<String>, // comma-separated list of nations, only those in the WA
    numunnations: Option<u32>, // number of WA nations
    poll: Option<RawPoll>,
    scvote: Option<RawRegionWAVote>,
    tags: Option<RawRegionTags>,
    wabadges: Option<RawRegionWABadges>,
}

#[derive(Debug, Deserialize)]
struct RawOfficers {
    #[serde(rename = "OFFICER", default)]
    inner: Vec<RawOfficer>,
}

impl TryFrom<RawOfficers> for Vec<Officer> {
    type Error = IntoRegionError;

    fn try_from(value: RawOfficers) -> Result<Self, Self::Error> {
        value
            .inner
            .into_iter()
            .map(Officer::try_from)
            .collect::<Result<Vec<_>, _>>()
    }
}

#[derive(Debug, Deserialize)]
struct RawEmbassies {
    #[serde(rename = "EMBASSY", default)]
    inner: Vec<RawEmbassy>,
}

impl TryFrom<RawEmbassies> for Vec<Embassy> {
    type Error = IntoRegionError;

    fn try_from(value: RawEmbassies) -> Result<Self, Self::Error> {
        value
            .inner
            .into_iter()
            .map(Embassy::try_from)
            .collect::<Result<Vec<_>, _>>()
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawOfficer {
    nation: String,
    office: String,
    authority: String,
    time: u64,
    by: String,
    order: i16,
}

impl TryFrom<RawOfficer> for Officer {
    type Error = IntoRegionError;

    fn try_from(value: RawOfficer) -> Result<Self, Self::Error> {
        let RawOfficer {
            nation,
            office,
            authority,
            time,
            by,
            order,
        } = value;
        Ok(Officer {
            nation,
            office,
            authority: authority
                .chars()
                .map(OfficerAuthority::try_from)
                .collect::<Result<Vec<_>, _>>()?,
            time,
            by,
            order,
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawEmbassy {
    #[serde(rename = "@type")] // attribute: "type"
    kind: Option<String>,
    #[serde(rename = "$value")] // extract inner text
    region: String,
}

impl TryFrom<RawEmbassy> for Embassy {
    type Error = IntoRegionError;
    fn try_from(value: RawEmbassy) -> Result<Self, Self::Error> {
        Ok(Self {
            region_name: value.region,
            kind: value
                .kind
                .map(|kind| match kind.as_str() {
                    "pending" => Ok(EmbassyKind::Pending),
                    "requested" => Ok(EmbassyKind::Requested),
                    "invited" => Ok(EmbassyKind::Invited),
                    "rejected" => Ok(EmbassyKind::Rejected),
                    "closing" => Ok(EmbassyKind::Closing),
                    _ => Err(IntoRegionError::BadFieldError("EmbassyKind", kind)),
                })
                .transpose()?
                .unwrap_or_default(),
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawRegionWAVote {
    #[serde(rename = "FOR")]
    for_vote: u16,
    #[serde(rename = "AGAINST")]
    against_vote: u16,
}

impl From<RawRegionWAVote> for RegionWAVote {
    fn from(value: RawRegionWAVote) -> Self {
        let RawRegionWAVote {
            for_vote,
            against_vote,
        } = value;
        RegionWAVote {
            for_vote,
            against_vote,
        }
    }
}

#[derive(Debug, Deserialize)]
struct RawMessages {
    #[serde(rename = "POST", default)]
    inner: Vec<RawMessage>,
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawMessage {
    #[serde(rename = "@id")]
    id: u32,
    timestamp: u64,
    nation: String,
    status: u8,                 // 0, 1, 2, 9
    suppressor: Option<String>, // nation
    edited: Option<u64>,        // timestamp
    likes: u16,                 // number of likes
    likers: Option<String>,     // list of nations that liked
    embassy: Option<String>,    // embassy region that nation posted from, if applicable
    message: String,            // the actual contents (thank god)
}

impl TryFrom<RawMessage> for Message {
    type Error = IntoRegionError;

    fn try_from(value: RawMessage) -> Result<Self, Self::Error> {
        let RawMessage {
            id,
            timestamp,
            nation,
            status,
            suppressor,
            edited,
            likes,
            likers,
            embassy,
            message,
        } = value;
        Ok(Message {
            id,
            timestamp: into_datetime(timestamp as i64).ok_or(IntoRegionError::BadFieldError(
                "Message.timestamp",
                timestamp.to_string(),
            ))?,
            nation,
            status: status.try_into()?,
            suppressor,
            edited: edited
                .map(|e| {
                    into_datetime(e as i64).ok_or(IntoRegionError::BadFieldError(
                        "Message.edited",
                        e.to_string(),
                    ))
                })
                .transpose()?,
            likes,
            likers,
            embassy,
            message: match message.as_str() {
                "Message suppressed by a moderator" | "Message deleted by author" => {
                    MaybeMessageContents::NoContents
                }
                _ => MaybeMessageContents::Contents(message),
            },
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawPoll {
    #[serde(rename = "@id")]
    id: u32,
    title: String,
    text: Option<String>,
    region: String,
    start: u64,
    stop: u64,
    author: String,
    options: RawPollOptions,
}

#[derive(Debug, Deserialize)]
struct RawPollOptions {
    #[serde(rename = "OPTIONS", default)]
    inner: Vec<RawPollOption>,
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
struct RawPollOption {
    #[serde(rename = "@id")]
    id: u32,
    optiontext: String,
    votes: u32,
    voters: String,
}

#[derive(Debug, Deserialize)]
struct RawRegionTags {
    #[serde(rename = "TAG", default)]
    inner: Vec<String>,
}

//noinspection SpellCheckingInspection
#[derive(Debug, Deserialize)]
struct RawRegionWABadges {
    #[serde(rename = "WABADGE", default)]
    inner: Vec<RawRegionWABadge>,
}

#[derive(Debug, Deserialize)]
struct RawRegionWABadge {
    #[serde(rename = "@type")]
    kind: String,
    #[serde(rename = "$text")]
    resolution: u16,
}

fn parse_dispatches(dispatch_list: String) -> Result<Vec<DispatchId>, IntoRegionError> {
    dispatch_list
        .split(',')
        .map(str::trim)
        .map(str::parse::<u32>)
        .map_ok(DispatchId)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| IntoRegionError::BadFieldError("Region.dispatches", e.to_string()))
}

fn try_into_bool(x: u8) -> Result<bool, IntoRegionError> {
    match x {
        0 => Ok(false),
        1 => Ok(true),
        e => Err(IntoRegionError::BadBooleanError(e)),
    }
}

fn try_into_datetime(
    value: Option<i64>,
    field: &'static str,
) -> Result<Option<DateTime<Utc>>, IntoRegionError> {
    match value {
        Some(v) => match into_datetime(v) {
            Some(d) => Ok(Some(d)),
            None => Err(IntoRegionError::BadFieldError(field, v.to_string())),
        },
        None => Ok(None),
    }
}

impl Region {
    /// Converts the XML response from NationStates to a [`Region`].
    pub fn from_xml(xml: &[u8]) -> Result<Self, IntoRegionError> {
        Self::try_from(quick_xml::de::from_reader::<&[u8], RawRegion>(xml)?)
    }
}

impl TryFrom<RawRegion> for Region {
    type Error = IntoRegionError;

    fn try_from(value: RawRegion) -> Result<Self, Self::Error> {
        Ok(Region {
            name: value.name,
            factbook: value.factbook,
            num_nations: value.numnations,
            nations: value
                .nations
                .map(|nations| nations.split(":").map(String::from).collect::<Vec<_>>()),
            delegate: value.delegate,
            delegate_votes: value.delegatevotes,
            delegate_authority: value
                .delegateauth
                .map(OfficerAuthority::vec_from_raw)
                .transpose()?,
            frontier: value.frontier.map(try_into_bool).transpose()?,
            founder: value.founder,
            governor: value.governor,
            officers: value.officers.map(RawOfficers::try_into).transpose()?,
            power: value.power,
            flag: value.flag,
            banner: value.banner,
            banner_url: value.bannerurl,
            embassies: value.embassies.map(RawEmbassies::try_into).transpose()?,
            banned: value.banned,
            banner_by: value.bannerby,
            census: value
                .census
                .map(CensusData::try_from)
                .transpose()
                .map_err(IntoRegionError::from)?,
            census_ranks: value
                .censusranks
                .map(CensusRegionRanks::try_from)
                .transpose()?,
            dbid: value.dbid,
            dispatches: value.dispatches.map(parse_dispatches).transpose()?,
            embassy_rmb: value.embassyrmb,
            founded: value.founded.map(MaybeRelativeTime::from),
            founded_time: value
                .foundedtime
                .map(into_datetime)
                .map(MaybeSystemTime::from),
            ga_vote: value.gavote.map(RegionWAVote::from),
            happenings: value.happenings.map(RawHappenings::into),
            history: value.history.map(RawHappenings::into), // TODO parsing history
            last_update: try_into_datetime(value.lastupdate, "Region.last_update")?,
            last_major_update: try_into_datetime(
                value.lastmajorupdate,
                "Region.last_major_update",
            )?,
            last_minor_update: try_into_datetime(
                value.lastminorupdate,
                "Region.last_minor_update",
            )?,
            messages: value
                .messages
                .map(|m| {
                    m.inner
                        .into_iter()
                        .map(Message::try_from)
                        .collect::<Result<Vec<_>, _>>()
                })
                .transpose()?,
            wa_nations: value.unnations,
            num_wa_nations: value.numunnations,
            poll: value.poll,
            sc_vote: value.scvote.map(RegionWAVote::from),
            tags: value.tags,
            wa_badges: value.wabadges,
        })
    }
}
