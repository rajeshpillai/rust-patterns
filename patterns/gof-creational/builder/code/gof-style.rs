// Direct GoF translation — Director + abstract Builder + ConcreteBuilder.
//
// This compiles, but it is NOT how you should write a builder in Rust.
// It inherits the mutable-shared-state design of the 1994 pattern, which
// forces Rc<RefCell<_>> and dyn dispatch that Rust gives you no reason
// to accept. Keep this file around only to show the contrast with
// code/idiomatic.rs.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

#[derive(Debug)]
struct HttpClient {
    endpoint: String,
    timeout: Duration,
    retries: u8,
}

trait HttpClientBuilder {
    fn set_endpoint(&self, endpoint: &str);
    fn set_timeout(&self, timeout: Duration);
    fn set_retries(&self, retries: u8);
    fn get_result(&self) -> HttpClient;
}

struct ConcreteHttpClientBuilder {
    endpoint: RefCell<Option<String>>,
    timeout: RefCell<Option<Duration>>,
    retries: RefCell<Option<u8>>,
}

impl ConcreteHttpClientBuilder {
    fn new() -> Rc<Self> {
        Rc::new(Self {
            endpoint: RefCell::new(None),
            timeout: RefCell::new(None),
            retries: RefCell::new(None),
        })
    }
}

impl HttpClientBuilder for ConcreteHttpClientBuilder {
    fn set_endpoint(&self, endpoint: &str) {
        *self.endpoint.borrow_mut() = Some(endpoint.to_owned());
    }
    fn set_timeout(&self, timeout: Duration) {
        *self.timeout.borrow_mut() = Some(timeout);
    }
    fn set_retries(&self, retries: u8) {
        *self.retries.borrow_mut() = Some(retries);
    }
    fn get_result(&self) -> HttpClient {
        // GoF assumes the caller has called every setter. In Rust this is
        // a runtime trap — missing fields panic instead of failing to compile.
        // See code/idiomatic.rs for the Result-returning alternative.
        HttpClient {
            endpoint: self
                .endpoint
                .borrow()
                .clone()
                .expect("endpoint was never set"),
            timeout: self.timeout.borrow().expect("timeout was never set"),
            retries: self.retries.borrow().expect("retries was never set"),
        }
    }
}

struct Director {
    builder: Rc<dyn HttpClientBuilder>,
}

impl Director {
    fn construct(&self) -> HttpClient {
        self.builder.set_endpoint("https://api.example.com");
        self.builder.set_timeout(Duration::from_secs(5));
        self.builder.set_retries(3);
        self.builder.get_result()
    }
}

fn main() {
    let builder = ConcreteHttpClientBuilder::new();
    let director = Director {
        builder: builder.clone(),
    };
    let client = director.construct();
    println!("{client:?}");
}
