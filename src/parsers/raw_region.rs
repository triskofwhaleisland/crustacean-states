use serde::Deserialize;

use crate::parsers::{
    region::{IntoRegionError, Officer, OfficerAuthority},
    RawCensus, RawCensusRanks, RawHappenings,
};

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
    foundedtime: Option<u64>,   // UNIX timestamp when the region was founded
    gavote: Option<RawRegionWAVote>,
    happenings: Option<RawHappenings>,
    history: Option<RawHappenings>,
    lastupdate: Option<u64>,
    lastmajorupdate: Option<u64>,
    lastminorupdate: Option<u64>,
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

#[derive(Debug, Deserialize)]
struct RawEmbassies {
    #[serde(rename = "EMBASSY", default)]
    inner: Vec<RawEmbassy>,
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

#[derive(Debug, Deserialize)]
struct RawRegionWAVote {
    #[serde(rename = "FOR")]
    for_vote: u16,
    #[serde(rename = "AGAINST")]
    against_vote: u16,
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

// impl Region {
//     /// Converts the XML response from NationStates to a [`Region`].
//     pub fn from_xml(xml: &[u8]) -> Result<Self, IntoRegionError> {
//         Self::try_from(quick_xml::de::from_reader::<&[u8], RawRegion>(xml)?)
//     }
// }

// impl TryFrom<RawRegion> for Region {
//     type Error = IntoRegionError;
//
//     fn try_from(value: RawRegion) -> Result<Self, Self::Error> {
//         Ok(Region {
//             inner: format!("{value:?}"),
//         })
//     }
// }
