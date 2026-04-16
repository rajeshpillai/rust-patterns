// Broken example — calling .build() before supplying the required endpoint.
//
// In the typestate builder, `build()` is defined only on
// HttpClientBuilder<Provided>. Calling it on HttpClientBuilder<Missing>
// doesn't "fail gracefully" — it simply does not compile.
//
// This file is expected to FAIL to compile with E0599.

use std::marker::PhantomData;
use std::time::Duration;

pub struct Missing;
pub struct Provided;

#[derive(Debug)]
pub struct HttpClient {
    pub endpoint: String,
}

pub struct HttpClientBuilder<Endpoint> {
    endpoint: Option<String>,
    _marker: PhantomData<Endpoint>,
}

impl HttpClientBuilder<Missing> {
    pub fn new() -> Self {
        Self { endpoint: None, _marker: PhantomData }
    }
    pub fn endpoint(self, e: impl Into<String>) -> HttpClientBuilder<Provided> {
        HttpClientBuilder { endpoint: Some(e.into()), _marker: PhantomData }
    }
}

impl HttpClientBuilder<Provided> {
    pub fn build(self) -> HttpClient {
        HttpClient { endpoint: self.endpoint.unwrap() }
    }
}

fn main() {
    // We never called .endpoint(...), so the builder is still
    // HttpClientBuilder<Missing>. `.build()` is defined only on the
    // Provided impl, so this line is a compile error:
    //
    //   error[E0599]: no method named `build` found for struct
    //                 `HttpClientBuilder<Missing>` in the current scope
    //
    // The compiler is protecting us from ever constructing an
    // HttpClient without an endpoint.
    let client = HttpClientBuilder::<Missing>::new()
        .build();
    //   ^^^^^ error[E0599]: no method named `build` found

    let _ = Duration::from_secs(1); // (kept so Duration import is used)
    println!("{client:?}");
}
