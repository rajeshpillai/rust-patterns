// Broken example — the classic misuse of a consuming-self builder.
//
// `.build()` moves `self` out of the builder to construct the HttpClient.
// Trying to call `.build()` again on the same binding is a use-after-move,
// which the borrow checker rejects with E0382.
//
// This file is expected to FAIL to compile. That is the teaching moment.

use std::time::Duration;

#[must_use = "HttpClientBuilder does nothing until .build() is called"]
#[derive(Default)]
pub struct HttpClientBuilder {
    endpoint: Option<String>,
    timeout: Option<Duration>,
}

#[derive(Debug)]
pub struct HttpClient {
    pub endpoint: String,
    pub timeout: Duration,
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
    pub fn build(self) -> HttpClient {
        HttpClient {
            endpoint: self.endpoint.unwrap_or_default(),
            timeout: self.timeout.unwrap_or(Duration::from_secs(30)),
        }
    }
}

fn main() {
    let builder = HttpClientBuilder::default()
        .endpoint("https://api.example.com")
        .timeout(Duration::from_secs(5));

    let client_a = builder.build();
    //                ^^^^^^^^^^ `builder` is moved here

    let client_b = builder.build();
    //             ^^^^^^^ error[E0382]: use of moved value: `builder`

    println!("{client_a:?} {client_b:?}");
}
