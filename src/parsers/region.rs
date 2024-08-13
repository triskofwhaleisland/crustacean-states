use crate::{
    parsers::{MaybeRelativeTime, MaybeSystemTime},
    shards::region::Tag,
};
use quick_xml::DeError;
use thiserror::Error;

#[derive(Debug)]
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
                String::from("OfficerAuthority"),
                String::from(c),
            )),
        }
    }
}

#[derive(Debug)]
pub struct Officer {
    pub nation: String,
    pub office: String,
    pub authority: Vec<OfficerAuthority>,
    pub time: u64,
    pub by: String,
    pub order: i16,
}

#[derive(Debug)]
pub struct Embassy;

#[derive(Debug)]
pub struct Census;

#[derive(Debug)]
pub struct CensusRanks;

#[derive(Debug)]
pub struct RegionWAVote;

#[derive(Debug)]
pub struct Message;

#[derive(Debug)]
pub struct Poll;

#[derive(Debug)]
pub struct RegionWABadge;

#[derive(Debug)]
pub struct Happenings;

#[derive(Debug)]
pub struct Region {
    // default shards
    pub name: Option<String>,               // nice name
    pub factbook: Option<String>,           // contains factbook
    pub num_nations: Option<u32>,           // number of nations inside
    pub nations: Option<String>,            // colon-separated list of nations
    pub delegate: Option<String>,           // internal name of delegate
    pub delegate_votes: Option<u32>,        // number of votes delegate has in World Assembly
    pub delegate_authority: Option<String>, // authorities that delegate has
    pub frontier: Option<bool>,             // 0 = not a frontier, 1 = frontier
    pub founder: Option<String>,            // name of the nation that founded the region
    pub governor: Option<String>,           // name of the nation that is governor
    pub officers: Option<Vec<Officer>>,     // list of officers
    pub power: Option<String>,              // regional power level
    pub flag: Option<String>,               // URL to region's flag
    pub banner: Option<u32>,                // region's banner ID
    pub banner_url: Option<String>,         // incomplete URL to banner.
    // appears to not have https://www.nationstates.net at the beginning
    pub embassies: Option<Vec<Embassy>>, // list of region's embassies
    // END default
    pub banned: Option<String>, // who is banned? separated by colons, internal name
    pub banner_by: Option<String>, // who made the banner?
    pub census: Option<Census>,
    pub census_ranks: Option<CensusRanks>,
    pub dbid: Option<u32>,
    pub dispatches: Option<String>, // list of IDs of pinned dispatches, comma separated
    pub embassy_rmb: Option<String>, // permissions given for embassies
    // posting on the RMB TODO find all
    pub founded: Option<MaybeRelativeTime>, // relative time since the region was founded
    pub founded_time: Option<MaybeSystemTime>, // UNIX timestamp when the region was founded
    pub ga_vote: Option<RegionWAVote>,
    pub happenings: Option<Happenings>,
    pub history: Option<Happenings>,
    pub last_update: Option<u64>,
    pub last_major_update: Option<u64>,
    pub last_minor_update: Option<u64>,
    pub messages: Option<Vec<Message>>,
    pub wa_nations: Option<String>, // comma-separated list of nations, only those in the WA
    pub num_wa_nations: Option<u32>, // number of WA nations
    pub poll: Option<Poll>,
    pub sc_vote: Option<RegionWAVote>,
    pub tags: Option<Vec<Tag>>,
    pub wa_badges: Option<Vec<RegionWABadge>>,
}

#[derive(Debug, Error)]
pub enum IntoRegionError {
    /// A field could not be parsed as the type it should be.
    #[error("malformed field {0} with value {1}")]
    BadFieldError(String, String),
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
    NoFieldError(String),
}
