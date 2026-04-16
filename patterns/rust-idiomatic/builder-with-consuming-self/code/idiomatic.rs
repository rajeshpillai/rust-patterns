// Builder with consuming self — the default Rust builder shape. Each
// setter takes `self` by value and returns `Self`, so the fluent
// chain is pure ownership transfer — no interior mutability, no
// cloning, no Rc/RefCell.
//
// Two complete builders in this file:
//
//   A) `HttpClient::builder()` — consuming self (this pattern).
//      One chain, one .build(), no ability to re-use the builder
//      once .build() has been called.
//
//   B) `LongLivedBuilder` — &mut self (contrast). Good when callers
//      need to branch mid-chain or call .build() multiple times.
//      Fields must be Clone because .build(&self) copies them out.

use std::time::Duration;

// ---- A) Consuming-self builder ---------------------------------------

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
    timeout:  Option<Duration>,
    retries:  Option<u8>,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum BuildError { MissingEndpoint, RetriesTooHigh(u8) }

impl HttpClientBuilder {
    /// Each setter takes `self` by value and returns `Self`. The old
    /// binding is moved; calling code must chain or re-bind.
    pub fn endpoint(mut self, v: impl Into<String>) -> Self {
        self.endpoint = Some(v.into()); self
    }
    pub fn timeout(mut self, v: Duration) -> Self {
        self.timeout = Some(v); self
    }
    pub fn retries(mut self, v: u8) -> Self {
        self.retries = Some(v); self
    }

    /// `build` also takes `self` — it moves the fully-configured
    /// builder into the returned HttpClient. After this call, the
    /// builder is gone. Calling .build() twice on the same binding
    /// is E0382 (use of moved value).
    pub fn build(self) -> Result<HttpClient, BuildError> {
        let endpoint = self.endpoint.ok_or(BuildError::MissingEndpoint)?;
        let timeout  = self.timeout.unwrap_or(Duration::from_secs(30));
        let retries  = self.retries.unwrap_or(0);
        if retries > 10 { return Err(BuildError::RetriesTooHigh(retries)); }
        Ok(HttpClient { endpoint, timeout, retries })
    }
}

// ---- B) &mut self builder — when callers need to re-use it ----------

// Example use case: a test harness that builds many "almost the same"
// HttpClients, tweaking one field per iteration.
#[derive(Clone, Default)]
pub struct LongLivedBuilder {
    endpoint: Option<String>,
    timeout:  Option<Duration>,
    retries:  Option<u8>,
}

impl LongLivedBuilder {
    pub fn endpoint(&mut self, v: impl Into<String>) -> &mut Self {
        self.endpoint = Some(v.into()); self
    }
    pub fn timeout(&mut self, v: Duration) -> &mut Self {
        self.timeout = Some(v); self
    }
    pub fn retries(&mut self, v: u8) -> &mut Self {
        self.retries = Some(v); self
    }

    /// `.build(&self)` clones the fields out so the builder stays
    /// alive and callable. Fields must implement Clone.
    pub fn build(&self) -> Result<HttpClient, BuildError> {
        let endpoint = self.endpoint.clone().ok_or(BuildError::MissingEndpoint)?;
        let timeout  = self.timeout.unwrap_or(Duration::from_secs(30));
        let retries  = self.retries.unwrap_or(0);
        if retries > 10 { return Err(BuildError::RetriesTooHigh(retries)); }
        Ok(HttpClient { endpoint, timeout, retries })
    }
}

fn main() -> Result<(), BuildError> {
    // A) Consuming self — the fluent one-shot chain.
    let client = HttpClient::builder()
        .endpoint("https://api.example.com")
        .timeout(Duration::from_secs(5))
        .retries(3)
        .build()?;
    println!("{client:?}");

    // B) &mut self — reuse the builder across branches.
    let mut b = LongLivedBuilder::default();
    b.endpoint("https://api.example.com").timeout(Duration::from_secs(5));

    for retries in [0_u8, 3, 5] {
        b.retries(retries);
        let c = b.build()?;   // .build(&self) leaves b usable
        println!("retries={retries} -> {c:?}");
    }
    Ok(())
}
