use crate::parsers::nation::RawEvent;
use once_cell::sync::Lazy;
use regex::{Regex, RegexSet};

#[derive(Debug)]
pub struct Event {
    pub timestamp: u64,
    pub text: String,
    pub nations: Vec<String>,
    pub regions: Vec<String>,
    pub kind: Option<EventKind>,
}

#[derive(Debug)]
pub enum EventKind {
    NewLaw {
        nation: String,
        joke: String,
    },
    NationReclassified {
        nation: String,
        from: String,
        to: String,
    },
    AlteredFlag {
        nation: String,
    },
    // can you tell where this is going?
}

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
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
