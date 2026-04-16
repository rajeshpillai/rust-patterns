// Typestate builder — the state of the construction lives in a generic
// type parameter. `build()` is defined ONLY for the "endpoint provided"
// state, so calling it without first calling `.endpoint(...)` is a
// compile error, not a runtime panic.
//
// Compare with the `Result<T, BuildError>` form in
// patterns/gof-creational/builder/code/idiomatic.rs — that version
// catches the missing-endpoint case at runtime. This one catches it
// at compile time.

use std::marker::PhantomData;
use std::time::Duration;

// ---- State markers -----------------------------------------------------

pub struct Missing;
pub struct Provided;

// ---- Product -----------------------------------------------------------

#[derive(Debug)]
pub struct HttpClient {
    pub endpoint: String,
    pub timeout: Duration,
    pub retries: u8,
}

impl HttpClient {
    pub fn builder() -> HttpClientBuilder<Missing> {
        HttpClientBuilder::new()
    }
}

// ---- Builder, parameterized over the endpoint state -------------------

#[must_use = "HttpClientBuilder does nothing until .build() is called"]
pub struct HttpClientBuilder<Endpoint> {
    endpoint: Option<String>,
    timeout: Duration,
    retries: u8,
    _marker: PhantomData<Endpoint>,
}

impl HttpClientBuilder<Missing> {
    pub fn new() -> Self {
        Self {
            endpoint: None,
            timeout: Duration::from_secs(30),
            retries: 0,
            _marker: PhantomData,
        }
    }

    // Supplying the endpoint moves us from `Missing` to `Provided`.
    // The old builder is consumed; the new one has a different type.
    pub fn endpoint(self, endpoint: impl Into<String>) -> HttpClientBuilder<Provided> {
        HttpClientBuilder {
            endpoint: Some(endpoint.into()),
            timeout: self.timeout,
            retries: self.retries,
            _marker: PhantomData,
        }
    }
}

// Optional fields are available in any state. Generic over `E` so they
// do not change the endpoint state.
impl<E> HttpClientBuilder<E> {
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub fn retries(mut self, retries: u8) -> Self {
        self.retries = retries;
        self
    }
}

// `build()` lives ONLY in the `Provided` impl block.
// Calling it on `HttpClientBuilder<Missing>` is a compile error.
impl HttpClientBuilder<Provided> {
    pub fn build(self) -> HttpClient {
        HttpClient {
            // The type system has proven this is Some, but `unwrap` is
            // still a panic at runtime if the invariant is ever broken.
            // We deliberately avoid it: pattern-match and unreachable!
            // expresses the invariant to the reader and to clippy.
            endpoint: match self.endpoint {
                Some(e) => e,
                None => unreachable!("typestate guarantees endpoint is set"),
            },
            timeout: self.timeout,
            retries: self.retries,
        }
    }
}

fn main() {
    let client = HttpClient::builder()
        .timeout(Duration::from_secs(5))
        .retries(3)
        .endpoint("https://api.example.com") // <- moves state to Provided
        .build();

    println!("{client:?}");
}
