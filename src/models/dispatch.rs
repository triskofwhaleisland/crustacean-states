//! Contains information about the Dispatch

use std::fmt::{Display, Formatter};

use strum::AsRefStr;

pub struct DispatchId(pub u32);

/// The categories of dispatches.
#[derive(Clone, Debug, PartialEq, AsRefStr)]
pub enum DispatchCategory {
    /// Factbooks officially describe a nation.
    Factbook(FactbookCategory),
    /// Bulletins address gameplay.
    Bulletin(BulletinCategory),
    /// Accounts are articles or stories involving a nation's people.
    Account(AccountCategory),
    /// Meta dispatches tend to address out-of-character and outside-of-role-play situations.
    Meta(MetaCategory),
}

#[derive(Clone, Debug, PartialEq, AsRefStr)]
#[allow(missing_docs)]
#[non_exhaustive]
/// The subcategories of factbooks.
/// Note that the [`FactbookCategory::Any`] variant can be used as a shard
/// to ask for any factbook.
pub enum FactbookCategory {
    Overview,
    History,
    Geography,
    Culture,
    Politics,
    Legislation,
    Religion,
    Military,
    Economy,
    International,
    Trivia,
    Miscellaneous,
    /// The type to choose if you are not picking a subcategory.
    /// NOTE:
    /// This is only used in shard queries;
    /// no [`Dispatch`][crate::parsers::Dispatch] will ever be tagged [`FactbookCategory::Any`].
    Any,
}

#[derive(Clone, Debug, PartialEq, AsRefStr)]
#[allow(missing_docs)]
#[non_exhaustive]
/// The subcategories of bulletins.
/// Note that the [`BulletinCategory::Any`] variant can be used as a shard
/// to ask for any bulletin.
pub enum BulletinCategory {
    Policy,
    News,
    Opinion,
    Campaign,
    /// The type to choose if you are not picking a subcategory.
    /// NOTE:
    /// This is only used in shard queries;
    /// no [`Dispatch`][crate::parsers::Dispatch] will ever be tagged [`BulletinCategory::Any`].
    Any,
}

#[derive(Clone, Debug, PartialEq, AsRefStr)]
#[allow(missing_docs)]
#[non_exhaustive]
/// The subcategories of accounts.
/// Note that the [`AccountCategory::Any`] variant can be used as a shard
/// to ask for any account.
pub enum AccountCategory {
    Military,
    Trade,
    Sport,
    Drama,
    Diplomacy,
    Science,
    Culture,
    Other,
    /// The type to choose if you are not picking a subcategory.
    /// NOTE:
    /// This is only used in shard queries;
    /// no [`Dispatch`][crate::parsers::Dispatch] will ever be tagged [`AccountCategory::Any`].
    Any,
}

#[derive(Clone, Debug, PartialEq, AsRefStr)]
#[allow(missing_docs)]
#[non_exhaustive]
/// The subcategories of meta-category dispatches.
/// Note that the [`MetaCategory::Any`] variant can be used as a shard
/// to ask for any meta-category dispatch.
pub enum MetaCategory {
    Gameplay,
    Reference,
    /// The type to choose if you are not picking a subcategory.
    /// NOTE:
    /// This is only used in shard queries;
    /// no [`Dispatch`][crate::parsers::Dispatch] will ever be tagged [`MetaCategory::Any`].
    Any,
}

impl Display for DispatchCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: {}",
            self.as_ref(),
            match self {
                // Nope, these cases can't fold because `cat` is a different type every time :(
                DispatchCategory::Factbook(cat) => cat.as_ref(),
                DispatchCategory::Bulletin(cat) => cat.as_ref(),
                DispatchCategory::Account(cat) => cat.as_ref(),
                DispatchCategory::Meta(cat) => cat.as_ref(),
            }
        )
    }
}
