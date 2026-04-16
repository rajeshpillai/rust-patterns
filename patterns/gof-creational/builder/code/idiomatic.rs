// Idiomatic Rust Builder — consuming self, fluent, validated at build().
//
// Properties:
//   * Every setter takes `self` by value and returns `Self`, so the chain
//     is just ownership transfer — no RefCell, no Rc, no dyn.
//   * Missing or invalid fields surface as a typed `BuildError`, not a
//     panic. Callers use `?` to propagate; they never `unwrap`.
//   * `#[must_use]` on the builder warns if a chain is ignored.
//   * `#[non_exhaustive]` on BuildError lets us add variants without
//     breaking downstream `match`es.

use std::fmt;
use std::time::Duration;

#[derive(Debug)]
pub struct HttpClient {
    pub endpoint: String,
    pub timeout: Duration,
    pub retries: u8,
}

impl HttpClient {
    pub fn builder() -> HttpClientBuilder {
        HttpClientBuilder::default()
    }
}

#[must_use = "HttpClientBuilder does nothing until .build() is called"]
#[derive(Default)]
pub struct HttpClientBuilder {
    endpoint: Option<String>,
    timeout: Option<Duration>,
    retries: Option<u8>,
}

impl HttpClientBuilder {
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    pub fn retries(mut self, retries: u8) -> Self {
        self.retries = Some(retries);
        self
    }

    pub fn build(self) -> Result<HttpClient, BuildError> {
        let endpoint = self.endpoint.ok_or(BuildError::MissingEndpoint)?;
        let timeout = self.timeout.unwrap_or(Duration::from_secs(30));
        let retries = self.retries.unwrap_or(0);
        if retries > 10 {
            return Err(BuildError::RetriesTooHigh(retries));
        }
        Ok(HttpClient { endpoint, timeout, retries })
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum BuildError {
    MissingEndpoint,
    RetriesTooHigh(u8),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BuildError::MissingEndpoint => f.write_str("endpoint is required"),
            BuildError::RetriesTooHigh(n) => {
                write!(f, "retries ({n}) exceeds the allowed maximum of 10")
            }
        }
    }
}

impl std::error::Error for BuildError {}

fn main() -> Result<(), BuildError> {
    let client = HttpClient::builder()
        .endpoint("https://api.example.com")
        .timeout(Duration::from_secs(5))
        .retries(3)
        .build()?;

    println!("{client:?}");
    Ok(())
}
