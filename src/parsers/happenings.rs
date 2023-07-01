//! National, regional, and world happenings.

use crate::parsers::nation::RawEvent;
use crate::regex;
use once_cell::sync::Lazy;
use regex::{Regex, RegexSet};

/// A happenings line.
#[derive(Debug)]
#[non_exhaustive]
pub struct Event {
    /// The Unix timestamp when the event happened.
    pub timestamp: u64,
    /// The exact contents of the event.
    /// Nations are wrapped in double @s, while regions are wrapped in double %s.
    pub text: String,
    /// The nations mentioned in the event text.
    pub nations: Vec<String>,
    /// The regions mentioned in the event text.
    pub regions: Vec<String>,
    /// The kind of event that this was.
    /// NOTE: this will always default to "None" until the happenings parsing update.
    pub kind: Option<EventKind>,
}

#[derive(Debug)]
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

static NATION_RE: Lazy<&Regex> = Lazy::new(|| regex!(r"@@[\w-]+@@"));
static REGION_RE: Lazy<&Regex> = Lazy::new(|| regex!(r"%%[\w-]+%%"));
static ALL_EXPRESSIONS: Lazy<RegexSet> =
    Lazy::new(|| RegexSet::new([NATION_RE.as_str(), REGION_RE.as_str()]).unwrap());

impl From<RawEvent> for Event {
    fn from(value: RawEvent) -> Self {
        let which_matched = ALL_EXPRESSIONS.matches(&value.text);

        let nations = if which_matched.matched(0) {
            NATION_RE
                .find_iter(&value.text)
                .map(|m| m.as_str().to_string())
                .collect()
        } else {
            vec![]
        };
        let regions = if which_matched.matched(1) {
            REGION_RE
                .find_iter(&value.text)
                .map(|m| m.as_str().to_string())
                .collect()
        } else {
            vec![]
        };

        Self {
            timestamp: value.timestamp,
            text: value.text,
            nations,
            regions,
            kind: None,
        }
    }
}
