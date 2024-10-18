use crate::models::dispatch::DispatchId;
use crate::parsers::happenings::Happenings;
use crate::parsers::nation::{BannerId, IntoNationError, NationName};
use crate::parsers::{NumNations, ParsingError};
use crate::{
    parsers::{CensusData, CensusRegionRanks, MaybeRelativeTime, MaybeSystemTime},
    shards::region::Tag,
};
use chrono::{DateTime, Utc};
use quick_xml::DeError;
use std::ops::Deref;
use thiserror::Error;
use url::Url;

#[derive(Clone, Debug)]
pub struct RegionName(pub String);

#[derive(Clone, Debug)]
#[non_exhaustive]
pub enum OfficerAuthority {
    Executive,
    WorldAssembly,
    Succession,
    Appearance,
    BorderControl,
    Communications,
    Embassies,
    Polls,
}

impl TryFrom<char> for OfficerAuthority {
    type Error = IntoRegionError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(OfficerAuthority::Executive),
            'W' => Ok(OfficerAuthority::WorldAssembly),
            'S' => Ok(OfficerAuthority::Succession),
            'A' => Ok(OfficerAuthority::Appearance),
            'B' => Ok(OfficerAuthority::BorderControl),
            'C' => Ok(OfficerAuthority::Communications),
            'E' => Ok(OfficerAuthority::Embassies),
            'P' => Ok(OfficerAuthority::Polls),
            c => Err(IntoRegionError::BadFieldError(
                "OfficerAuthority",
                String::from(c),
            )),
        }
    }
}

impl OfficerAuthority {
    pub(super) fn vec_from_raw(auth_str: String) -> Result<Vec<Self>, IntoRegionError> {
        auth_str.chars().map(OfficerAuthority::try_from).collect()
    }
}

#[derive(Debug)]
pub struct Officer {
    pub nation: String,
    pub office: String,
    pub authority: Vec<OfficerAuthority>,
    pub time: DateTime<Utc>,
    pub by: String,
    pub order: i16,
}

#[derive(Debug)]
pub struct Embassy {
    pub region_name: String,
    pub kind: EmbassyKind,
}

#[derive(Debug, Default)]
pub enum EmbassyKind {
    /// The default status of an embassy.
    #[default]
    Established,
    /// The embassy is being built.
    Pending,
    /// The embassy has been proposed by this region.
    Requested,
    /// The embassy has been proposed by the other region.
    Invited,
    /// The embassy proposal was rejected.
    Rejected,
    /// The embassy is closing.
    Closing,
}

#[derive(Clone, Debug)]
pub enum EmbassyRmbPerms {
    NoEmbassyPosting,
    DelegatesAndFounders,
    Officers,
    OfficersWithCommsAuth,
    All,
}

impl TryFrom<String> for EmbassyRmbPerms {
    type Error = IntoRegionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "0" => Ok(EmbassyRmbPerms::NoEmbassyPosting),
            "con" => Ok(EmbassyRmbPerms::DelegatesAndFounders),
            "off" => Ok(EmbassyRmbPerms::Officers),
            "com" => Ok(EmbassyRmbPerms::OfficersWithCommsAuth),
            "all" => Ok(EmbassyRmbPerms::All),
            _ => Err(IntoRegionError::BadFieldError("Region.embassy_rmb", value)),
        }
    }
}

#[derive(Debug)]
pub struct RegionWAVote {
    pub for_vote: u16,
    pub against_vote: u16,
}

#[derive(Clone, Debug)]
pub struct Message {
    pub id: u32,
    pub timestamp: DateTime<Utc>,
    pub nation: String,
    pub status: MessageStatus,
    pub suppressor: Option<String>,    // nation
    pub edited: Option<DateTime<Utc>>, // timestamp
    pub likes: u16,                    // number of likes
    pub likers: Option<String>,        // list of nations that liked
    pub embassy: Option<String>,       // embassy region that nation posted from, if applicable
    pub message: String,               // the actual contents (thank god)
}

#[derive(Clone, Debug)]
pub enum MessageStatus {
    Visible,
    Suppressed,
    Deleted,
    ModSuppressed,
}

impl TryFrom<u8> for MessageStatus {
    type Error = ParsingError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MessageStatus::Visible),
            1 => Ok(MessageStatus::Suppressed),
            2 => Ok(MessageStatus::Deleted),
            9 => Ok(MessageStatus::ModSuppressed),
            _ => Err(ParsingError::BadFieldError(
                "Message.status",
                value.to_string(),
            )),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Poll {
    pub id: u32,
    pub title: String,
    pub text: Option<String>,
    pub region: RegionName,
    pub start: DateTime<Utc>,
    pub stop: DateTime<Utc>,
    pub author: NationName,
    pub options: Vec<PollOption>,
}

#[derive(Clone, Debug)]
pub struct PollOption {
    pub(crate) id: u32,
    pub(crate) text: String,
    pub(crate) votes: u32,
    pub(crate) voters: Vec<NationName>,
}

#[derive(Clone, Debug)]
pub struct RegionBannerId(pub u32);

#[derive(Debug)]
pub struct RegionWABadge;

#[derive(Debug)]
pub struct Region {
    // default shards
    pub name: Option<RegionName>,                          // nice name
    pub factbook: Option<String>,                          // contains factbook TODO make struct
    pub num_nations: Option<NumNations>,                   // number of nations inside
    pub nations: Option<Vec<NationName>>,                  // list of nations
    pub delegate: Option<NationName>,                      // internal name of delegate
    pub delegate_votes: Option<NumNations>, // number of votes delegate has in World Assembly
    pub delegate_authority: Option<Vec<OfficerAuthority>>, // authorities that delegate has
    pub frontier: Option<bool>,             // 0 = not a frontier, 1 = frontier
    pub founder: Option<NationName>,        // name of the nation that founded the region
    pub governor: Option<NationName>,       // name of the nation that is governor
    pub officers: Option<Vec<Officer>>,     // list of officers
    pub power: Option<String>,              // regional power level TODO make enum
    pub flag: Option<String>,               // URL to region's flag TODO make struct
    pub banner: Option<RegionBannerId>,     // region's banner ID
    pub banner_url: Option<Url>,            // incomplete URL to banner.
    // appears to not have https://www.nationstates.net at the beginning
    pub embassies: Option<Vec<Embassy>>, // list of region's embassies
    // END default
    pub banned: Option<Vec<NationName>>, // who is banned? separated by colons, internal name
    pub banner_by: Option<NationName>,   // who made the banner?
    pub census: Option<CensusData>,
    pub census_ranks: Option<CensusRegionRanks>,
    pub dbid: Option<u32>,
    pub dispatches: Option<Vec<DispatchId>>, // list of IDs of pinned dispatches, comma separated
    pub embassy_rmb: Option<EmbassyRmbPerms>, // permissions given for embassies
    // posting on the RMB TODO find all
    pub founded: Option<MaybeRelativeTime>, // relative time since the region was founded
    pub founded_time: Option<MaybeSystemTime>, // UNIX timestamp when the region was founded
    pub ga_vote: Option<RegionWAVote>,
    pub happenings: Option<Happenings>,
    pub history: Option<Happenings>, // TODO change this
    pub last_update: Option<DateTime<Utc>>,
    pub last_major_update: Option<DateTime<Utc>>,
    pub last_minor_update: Option<DateTime<Utc>>,
    pub messages: Option<Vec<Message>>,
    pub wa_nations: Option<Vec<NationName>>, // comma-separated list of nations,
    // only those in the WA
    pub num_wa_nations: Option<NumNations>, // number of WA nations
    pub poll: Option<Poll>,
    pub sc_vote: Option<RegionWAVote>,
    pub tags: Option<Vec<Tag>>,
    pub wa_badges: Option<Vec<RegionWABadge>>,
}

#[derive(Clone, Debug, Error)]
#[non_exhaustive]
pub enum IntoRegionError {
    /// A field could not be parsed as the type it should be.
    #[error("malformed field {0} with value {1}")]
    BadFieldError(&'static str, String),
    /// A `u8` could not be parsed as a `bool` because it was not `0` or `1`.
    #[error("boolean cannot be derived from {0}")]
    BadBooleanError(u8),
    /// Something bad happened in deserialization.
    #[error("deserialization failed")]
    DeserializationError {
        /// The error source. Look here for what went wrong.
        #[from]
        source: DeError,
    },
    /// A field was missing from the response.
    #[error("could not find the field {0} in response")]
    NoFieldError(&'static str),

    #[error("{0:?} cannot be converted into {1}")]
    WrongGeneric(ParsingError, &'static str),
    
    #[error("Converting string to enum failed")]
    StrumParseError {
        #[from]
        source: strum::ParseError,
    }
}

impl From<ParsingError> for IntoRegionError {
    fn from(value: ParsingError) -> Self {
        match value {
            ParsingError::Nation(ref _n) => IntoRegionError::WrongGeneric(value, "IntoRegionError"),
            ParsingError::Region(r) => r.deref().clone(),
            ParsingError::BadFieldError(field, value) => {
                IntoRegionError::BadFieldError(field, value)
            }
            ParsingError::NoFieldError(field) => IntoRegionError::NoFieldError(field),
        }
    }
}
