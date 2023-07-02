//! A NationStates API wrapper that takes full advantage of Rust's type system.
//!
//! Works with the current API (v12) as of 2 July 2023.
//!
//! Using the library usually takes three steps:
//!
//! 1. Creating a [`NSRequest`][crate::shard::NSRequest] with the relevant shards.
//! 2. Sending the request as a URL through a [`Client`][crate::client::Client].
//! 3. Parsing the response using a parser in [`parsers`].
//!
//! Currently, the following requests can be formed and sent:
//! - Nation (public shards only): [`NSRequest::new_nation`], from [`PublicNationShards`][crate::shards::public_nation::PublicNationShard]; also, [`NSRequest::new_nation_standard`]
//! - Region: [`NSRequest::new_region`], from [`RegionShards`][crate::shards::region::RegionShard]; also, [`NSRequest::new_region_standard`]
//! - World (except for `regionsfromtag`): [`NSRequest::new_world`], from [`WorldShards`][crate::shards::world::WorldShard]
//! - WA (World Assembly): [`NSRequest::new_wa`], from [`WAShards`][crate::shards::world_assembly::WAShard`]
//!
//! The following requests can be parsed:
//! - [`Nation`][crate::parsers::nation::Nation] (some fields still being finalized)
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

// #![deny(missing_docs)]

#[allow(unused_imports)] // it's for the docs :)
use crate::shards::NSRequest;

#[doc(hidden)]
mod macros;

pub mod client;
pub mod parsers;
pub mod shards;

/// Takes a nation name with capital letters and spaces and turns it into a safe-to-send, lowercase name.
pub fn safe_name(unsafe_name: impl ToString) -> String {
    unsafe_name
        .to_string()
        .to_ascii_lowercase()
        .replace(' ', "_")
        .to_ascii_lowercase()
}

/// Takes a lowercase, web-safe name and replaces it with a name that should match the real name on NationStates.
///
/// Note: this will not always result in a name that is capitalized the same way as it is on NationStates.
pub fn pretty_name(safe_name: impl ToString) -> String {
    safe_name
        .to_string()
        .replace('_', " ")
        .chars()
        .fold(String::new(), |s, c| {
            format!(
                "{s}{}",
                if s.is_empty() || s.ends_with(' ') {
                    c.to_ascii_uppercase()
                } else {
                    c
                }
            )
        })
}
