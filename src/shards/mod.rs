pub mod public_nation_shards;
pub mod region_shards;
pub mod world_assembly_shards;
pub mod world_shards;

use crate::safe_name;
use crate::shards::public_nation_shards::PublicNationShard;
use crate::shards::region_shards::RegionShard;
use crate::shards::world_assembly_shards::WACouncil;
use std::collections::HashMap;
use std::fmt::{Debug, Display, Formatter};

pub type Params = HashMap<String, String>;

#[derive(Debug)]
pub struct Shard {
    pub(crate) query: String,
    pub(crate) params: HashMap<String, String>,
}

impl Shard {
    fn query_and_params<T: Into<Self> + Clone>(shards: &[T]) -> (String, Params) {
        let mut params = Params::new();
        let mut query = String::from("&q=");
        shards.iter().for_each(|s| {
            let shard: Shard = s.clone().into();
            if !query.is_empty() {
                query.push('+');
            }
            query.push_str(shard.query.as_str());
            params.extend(shard.params);
        });
        (query, params)
    }

    fn name<T: Debug>(shard: &T) -> String {
        let true_debug = format!("{shard:?}");
        if let Some((tuple, _)) = true_debug.split_once('(') {
            tuple.to_string()
        } else if let Some((struct_like, _)) = true_debug.split_once(' ') {
            struct_like.to_string()
        } else {
            true_debug
        }
    }
}

pub struct NSRequest {
    kind: NSRequestKind,
    query: String,
    params: Params,
}

pub enum NSRequestKind {
    PublicNation(String),
    Region(String),
    World,
    WA { council: WACouncil, id: Option<u16> },
}

impl NSRequest {
    /// Create a nation request with any number of [`PublicNationShard`]s.
    ///
    /// # Example
    ///
    /// ```
    /// use crustacean_states::shards::NSRequest;
    /// use crustacean_states::shards::public_nation_shards::PublicNationShard;
    ///
    /// let request = NSRequest::new_nation("Testlandia",
    ///         &[PublicNationShard::Region, PublicNationShard::Demonym]).to_string();
    /// ```
    /// When sent,
    /// it will request information about [Testlandia](https://www.nationstates.net/nation=testlandia)'s region and demonym.
    pub fn new_nation(nation: impl ToString, shards: &[PublicNationShard]) -> Self {
        let (query, params) = Shard::query_and_params(shards);
        Self {
            kind: NSRequestKind::PublicNation(nation.to_string()),
            query,
            params,
        }
    }
    /// Create a "standard" nation request.
    ///
    /// The following fields of [`Nation`] will not be `None`:
    ///
    /// `name`, `kind`, `full_name`, `motto`, `category`, `wa_status`, `endorsements`,
    /// `issues_answered`, `freedom`, `region`, `population`, `tax`, `animal`, `currency`,
    /// `demonym`, `demonym2`, `demonym2_plural`, `flag`, `major_industry`, `government_priority`,
    /// `government`, `founded`, `first_login`, `last_login`, `influence`, `freedom_scores`,
    /// `public_sector`, `deaths`, `factbooks`, `dispatches`, `dbid`
    ///
    ///
    /// The following fields will be filled
    /// only if the nation has reached a certain population and answered the relevant issue:
    /// - `capital`: 250 million
    ///
    /// - `kind` will deviate from the original pre-titles after 500 million.
    /// (No issue must be completed to unlock this ability.)
    ///
    /// - `leader`: 750 million
    ///
    /// - `religion`: 1 billion
    ///
    /// [`Nation`]: crate::parsers::nation::Nation
    pub fn new_nation_standard(nation: impl ToString) -> Self {
        Self {
            kind: NSRequestKind::PublicNation(nation.to_string()),
            query: Default::default(),
            params: Default::default(),
        }
    }

    /// Create a region request with any number of [`RegionShard`]s.
    ///
    /// # Example
    ///
    /// ```
    /// use crustacean_states::shards::NSRequest;
    /// use crustacean_states::shards::region_shards::RegionShard;
    ///
    /// let request = NSRequest::new_region("Testregionia",
    ///         &[RegionShard::Delegate, RegionShard::Flag]).to_string();
    /// ```
    ///
    /// When sent,
    /// it will request information about [Testregionia](https://www.nationstates.net/region=testregionia)'s delegate and flag.
    pub fn new_region(region: impl ToString, shards: &[RegionShard]) -> Self {
        let (query, params) = Shard::query_and_params(shards);
        Self {
            kind: NSRequestKind::Region(region.to_string()),
            query,
            params,
        }
    }
    /// Create a "standard" region request.
    pub fn new_region_standard(region: impl ToString) -> Self {
        Self {
            kind: NSRequestKind::Region(region.to_string()),
            query: Default::default(),
            params: Default::default(),
        }
    }
}

impl Display for NSRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}{}",
            match &self.kind {
                NSRequestKind::PublicNation(n) => format!("nation={}", safe_name(n)),
                NSRequestKind::Region(r) => format!("region={}", safe_name(r)),
                NSRequestKind::World => String::new(),
                NSRequestKind::WA { council, id } => match id {
                    Some(i) => format!("wa={}&id={i}", council.clone() as u8),
                    None => format!("wa={}", council.clone() as u8),
                },
            },
            (!self.query.is_empty())
                .then(|| format!("&q={}", self.query))
                .unwrap_or_default(),
            (!self.params.is_empty())
                .then(|| self
                    .params
                    .iter()
                    .fold(String::new(), |acc, (k, v)| format!("{acc}&{k}={v}")))
                .unwrap_or_default(),
        )
    }
}
