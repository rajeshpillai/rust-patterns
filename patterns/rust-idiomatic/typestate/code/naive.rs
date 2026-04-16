// Naive alternative — runtime-checked construction.
//
// This file compiles, and is a reasonable fallback if you do not want to
// reach for typestate. The tradeoff is explicit: the invariant "endpoint
// was set" is enforced at RUNTIME via `Result<_, BuildError>`, not at
// COMPILE TIME. A caller that forgets to call .endpoint() will get an
// error in production the first time that code path runs, not when they
// hit "cargo check".
//
// Keep this around as the honest comparison: typestate costs more types,
// but trades compile-time verification for runtime checks.

use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub struct HttpClient {
    pub endpoint: String,
    pub timeout: Duration,
}

#[must_use]
#[derive(Default)]
pub struct HttpClientBuilder {
    endpoint: Option<String>,
    timeout: Option<Duration>,
}

impl HttpClientBuilder {
    pub fn endpoint(mut self, e: impl Into<String>) -> Self {
        self.endpoint = Some(e.into());
        self
    }
    pub fn timeout(mut self, t: Duration) -> Self {
        self.timeout = Some(t);
        self
    }
    pub fn build(self) -> Result<HttpClient, BuildError> {
        let endpoint = self.endpoint.ok_or(BuildError::MissingEndpoint)?;
        let timeout = self.timeout.unwrap_or(Duration::from_secs(30));
        Ok(HttpClient { endpoint, timeout })
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum BuildError {
    MissingEndpoint,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::MissingEndpoint => f.write_str("endpoint is required"),
        }
    }
}
impl std::error::Error for BuildError {}

fn main() -> Result<(), BuildError> {
    let client = HttpClientBuilder::default()
        .endpoint("https://api.example.com")
        .build()?;
    println!("{client:?}");
    Ok(())
}
