//! A NationStates API wrapper that takes full advantage of Rust's type system.
//!
//! Works with the current API (v12) as of 4 October 2023.
//!
//! Using the library usually takes three steps:
//!
//! 1. Creating a request
//!    (e.g. [`PublicNationRequest`])
//!    with the relevant shards.
//! 2. Sending the request as a URL through a [`Client`][crate::client::Client].
//! 3. Parsing the response using a parser in [`parsers`].
//!
//! Currently, the following requests can be formed and sent:
//! - Nation (public shards only):
//!   [`PublicNationRequest::new`](shards::nation::PublicNationRequest::new),
//!   from [`PublicNationShards`](shards::nation::PublicNationShard);
//!   also, [`StandardPublicNationRequest`](shards::nation::StandardPublicNationRequest)
//! - Region: [`RegionRequest::new`](shards::region::RegionRequest::new),
//!   from [`RegionShards`](shards::region::RegionShard);
//!   also, [`StandardRegionRequest`](shards::region::StandardRegionRequest)
//! - World:
//!   [`WorldRequest::new`](shards::world::WorldRequest::new),
//!   from [`WorldShards`](shards::world::WorldShard)
//! - WA (World Assembly): [`WAShard`](shards::wa::WARequest),
//!   from [`WAShards`](shards::wa::WAShard`)
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
//!   see [the examples folder on GitHub](https://github.com/triskofwhaleisland/crustacean-states/tree/main/examples).
//!
//! [`PublicNationRequest`]: [crate::shards::nation::PublicNationRequest]

// #![deny(missing_docs)]

#[doc(hidden)]
mod macros;

#[cfg(feature = "client")]
pub mod client;
pub mod models;
pub mod parsers;
pub mod shards;
