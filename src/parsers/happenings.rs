//! National, regional, and world happenings.

use regex::{Regex, RegexSet};
use std::sync::LazyLock;

use crate::{
    parsers::{
        nation::NationName,
        region::RegionName,
        RawEvent
    },
    regex
};

#[derive(Clone, Debug)]
pub struct Happenings(pub Vec<Event>);

/// A line of `happenings`.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Event {
    /// The Unix timestamp when the event happened.
    pub timestamp: u64,
    /// The exact contents of the event.
    /// Nations are wrapped in `@@`, while regions are wrapped in `%%`.
    pub text: String,
    /// The nations mentioned in the event text.
    pub nations: Vec<NationName>,
    /// The regions mentioned in the event text.
    pub regions: Vec<RegionName>,
    /// The kind of event that this was.
    /// NOTE: this will always be `None` until the happenings parsing update.
    pub kind: Option<EventKind>,
}

#[derive(Clone, Debug)]
#[non_exhaustive]
/// The kind of event. Not currently implemented.
pub enum EventKind {
    // NewLaw {
    //     nation: String,
    //     joke: String,
    // },
    // NationReclassified {
    //     nation: String,
    //     from: String,
    //     to: String,
    // },
    // AlteredFlag {
    //     nation: String,
    // },
    // can you tell where this is going?
}

static NATION_RE: LazyLock<&Regex> = LazyLock::new(|| regex!(r"@@[a-zA-Z0-9-]+@@"));
static REGION_RE: LazyLock<&Regex> = LazyLock::new(|| regex!(r"%%[a-zA-Z0-9-]+%%"));
static ALL_EXPRESSIONS: LazyLock<RegexSet> =
    LazyLock::new(|| RegexSet::new([NATION_RE.as_str(), REGION_RE.as_str()]).unwrap());

impl From<RawEvent> for Event {
    fn from(value: RawEvent) -> Self {
        let which_matched = ALL_EXPRESSIONS.matches(&value.text);

        let nations = which_matched
            .matched(0)
            .then(|| {
                NATION_RE
                    .find_iter(&value.text)
                    .map(|m| m.as_str())
                    .map(|m| m.trim_start_matches("@").trim_end_matches("@"))
                    .map(String::from)
                    .map(NationName)
                    .collect()
            })
            .unwrap_or_default();

        let regions = which_matched
            .matched(1)
            .then(|| {
                REGION_RE
                    .find_iter(&value.text)
                    .map(|m| m.as_str())
                    .map(|m| m.trim_start_matches("%").trim_end_matches("%"))
                    .map(String::from)
                    .map(RegionName)
                    .collect()
            })
            .unwrap_or_default();

        Self {
            timestamp: value.timestamp,
            text: value.text,
            nations,
            regions,
            kind: None,
        }
    }
}
