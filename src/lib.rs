//! Official Rust SDK for the Fluxx control plane API.

/// The current SDK version.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Entry point for the Fluxx API.
#[derive(Debug, Clone)]
pub struct Client {
    api_key: String,
}

impl Client {
    /// Construct a client bound to the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self { api_key: api_key.into() }
    }

    /// The API key this client is bound to (for diagnostics only — never log).
    pub(crate) fn api_key(&self) -> &str {
        &self.api_key
    }
}
