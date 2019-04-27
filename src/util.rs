//! A module for common utilities that don't apply
//! to a bot necessarily.

/// Find a ENV variable by key
pub fn get_argv(key: &str) -> Option<String> {
    std::env::vars()
        .find(|(k, _)| *k == *key)
        .map(|(_, value)| value)
}
