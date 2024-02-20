//! A NationStates API wrapper that takes full advantage of Rust's type system.
//!
//! Works with the current API (v12) as of 4 October 2023.
//!
//! Using the library usually takes three steps:
//!
//! 1. Creating a request
//! (e.g. [`PublicNationRequest`])
//! with the relevant shards.
//! 2. Sending the request as a URL through a [`Client`][crate::client::Client].
//! 3. Parsing the response using a parser in [`parsers`].
//!
//! Currently, the following requests can be formed and sent:
//! - Nation (public shards only):
//! [`PublicNationRequest::new`](shards::nation::PublicNationRequest::new),
//! from [`PublicNationShards`](shards::nation::PublicNationShard);
//! also, [`StandardPublicNationRequest`](shards::nation::StandardPublicNationRequest)
//! - Region: [`RegionRequest::new`](shards::region::RegionRequest::new),
//! from [`RegionShards`](shards::region::RegionShard);
//! also, [`StandardRegionRequest`](shards::region::StandardRegionRequest)
//! - World:
//! [`WorldRequest::new`](shards::world::WorldRequest::new),
//! from [`WorldShards`](shards::world::WorldShard)
//! - WA (World Assembly): [`WAShard`](shards::wa::WARequest),
//! from [`WAShards`](shards::wa::WAShard`)
//!
//! The following requests can be parsed:
//! - [`Nation`](parsers::nation::Nation) (some fields still being finalized)
//!
//! The following functionality is planned, but is not implemented:
//! - parsers for Region, World, and WA request responses
//! - private shards
//! - lighter-weight client using `hyper`
//! - breaking crate into features
//!
//! ## Examples
//! For a list of examples,
//! see [the examples folder on GitHub](https://github.com/triskofwhaleisland/crustacean-states/tree/main/examples).
//!
//! [`PublicNationRequest`]: [crate::shards::nation::PublicNationRequest]

// #![deny(missing_docs)]

#[doc(hidden)]
mod macros;

pub mod client;
pub mod models;
pub mod parsers;
pub mod shards;

/// Takes a nation name with capital letters and spaces
/// and turns it into a safe-to-send, lowercase name.
pub fn safe_name<S: ToString>(unsafe_name: S) -> String {
    unsafe_name
        .to_string()
        .to_ascii_lowercase()
        .replace(' ', "_")
        .to_ascii_lowercase()
}

/// Takes a lowercase, web-safe name and replaces it with a name
/// that should match the real name on NationStates.
///
/// Note: this will not always result in a name
/// that is capitalized the same way as it is on NationStates.
pub fn pretty_name<S: ToString>(safe_name: S) -> String {
    safe_name
        .to_string()
        .replace('_', " ")
        .chars()
        .fold(String::new(), |s, c| {
            format!(
                "{s}{}",
                if s.ends_with(' ') || s.is_empty() {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            )
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn safe_name_unchanged() {
        assert_eq!(super::safe_name("wow1"), String::from("wow1"));
    }

    #[test]
    fn safe_name_lowercase() {
        assert_eq!(super::safe_name("Exciting"), String::from("exciting"));
    }

    #[test]
    fn safe_name_underscore() {
        assert_eq!(
            super::safe_name("wow1 exciting"),
            String::from("wow1_exciting")
        );
    }

    #[test]
    fn safe_name_underscore_and_lowercase() {
        assert_eq!(
            super::safe_name("Wow1 Exciting"),
            String::from("wow1_exciting")
        );
    }

    #[test]
    fn pretty_name_uppercase() {
        assert_eq!(super::pretty_name("aramos"), String::from("Aramos"))
    }

    #[test]
    fn pretty_name_multiword() {
        assert_eq!(
            super::pretty_name("the_greater_low_countries"),
            String::from("The Greater Low Countries")
        )
    }
}
