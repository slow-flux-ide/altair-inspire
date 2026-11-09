//! Static build info for Altair Inspire.

pub const BUILD_VERSION: &str  = "v1.6.28";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "8c421ca9efb4";
pub const BUILD_TIMESTAMP: u64 = 1781749682;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Altair Inspire", BUILD_VERSION, BUILD_CHANNEL)
}
