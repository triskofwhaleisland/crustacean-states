//! A NationStates API wrapper that takes full advantage of Rust's type system.

// #![deny(missing_docs)]

#[doc(hidden)]
mod macros;

pub mod parsers;
pub mod rate_limiter;
pub mod shards;

/// Takes a nation name with capital letters and spaces and turns it into a safe-to-send, lowercase name.
pub fn safe_name(unsafe_name: impl ToString) -> String {
    unsafe_name
        .to_string()
        .to_ascii_lowercase()
        .replace(' ', "_")
        .to_lowercase()
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
            if s.is_empty() || s.ends_with(' ') {
                s + c.to_ascii_uppercase().to_string().as_str()
            } else {
                s + c.to_string().as_str()
            }
        })
}
