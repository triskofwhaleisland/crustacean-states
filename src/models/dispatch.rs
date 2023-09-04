//! Contains information about the Dispatch

use std::fmt::{Display, Formatter};

/// The categories of dispatches.
#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
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

impl Display for FactbookCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FactbookCategory::Any => String::new(),
                c => format!("{c:?}"),
            }
        )
    }
}

impl Display for BulletinCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BulletinCategory::Any => String::new(),
                c => format!("{c:?}"),
            }
        )
    }
}

impl Display for AccountCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                AccountCategory::Any => String::new(),
                c => format!("{c:?}"),
            }
        )
    }
}

impl Display for MetaCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                MetaCategory::Any => String::new(),
                c => format!("{c:?}"),
            }
        )
    }
}

impl Display for DispatchCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", {
            let c: (&str, Option<String>) = match self {
                DispatchCategory::Factbook(subcategory) => (
                    "Factbook",
                    match subcategory {
                        FactbookCategory::Any => None,
                        other => Some(other.to_string()),
                    },
                ),
                DispatchCategory::Bulletin(subcategory) => (
                    "Bulletin",
                    match subcategory {
                        BulletinCategory::Any => None,
                        other => Some(other.to_string()),
                    },
                ),
                DispatchCategory::Account(subcategory) => (
                    "Account",
                    match subcategory {
                        AccountCategory::Any => None,
                        other => Some(other.to_string()),
                    },
                ),
                DispatchCategory::Meta(subcategory) => (
                    "Meta",
                    match subcategory {
                        MetaCategory::Any => None,
                        other => Some(other.to_string()),
                    },
                ),
            };
            format!(
                "{}{}",
                c.0,
                c.1.map(|x| format!(": {x}")).unwrap_or_default(),
            )
        })
    }
}
