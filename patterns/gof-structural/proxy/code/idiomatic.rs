// Proxy — three flavors behind one trait.
//
//   A) AuthProxy — protection proxy. Refuses requests without a
//      valid token.
//   B) CachingProxy — caching proxy. Only hits the real Fetcher on
//      cache miss.
//   C) LazyFetcher — virtual proxy. Defers construction of the
//      real thing until the first call.
//
// All three share the same `Fetcher` trait as the real object, so
// callers can swap them transparently.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum FetchError {
    Unauthorized,
    NotFound { key: String },
    Backend { reason: String },
}

impl fmt::Display for FetchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unauthorized         => f.write_str("unauthorized"),
            Self::NotFound { key }     => write!(f, "not found: {key}"),
            Self::Backend { reason }   => write!(f, "backend: {reason}"),
        }
    }
}
impl std::error::Error for FetchError {}

pub trait Fetcher {
    fn fetch(&self, key: &str) -> Result<Vec<u8>, FetchError>;
}

// ---- Real implementation --------------------------------------------

pub struct RealFetcher {
    data: HashMap<String, Vec<u8>>,
}
impl RealFetcher {
    pub fn with_data<I>(entries: I) -> Self
    where I: IntoIterator<Item = (String, Vec<u8>)>
    {
        Self { data: entries.into_iter().collect() }
    }
}
impl Fetcher for RealFetcher {
    fn fetch(&self, key: &str) -> Result<Vec<u8>, FetchError> {
        println!("[real] fetching {key}");
        self.data.get(key).cloned().ok_or_else(|| FetchError::NotFound { key: key.into() })
    }
}

// ---- A) Protection proxy --------------------------------------------

pub struct AuthProxy<F: Fetcher> {
    pub inner: F,
    pub token: String,
}

impl<F: Fetcher> Fetcher for AuthProxy<F> {
    fn fetch(&self, key: &str) -> Result<Vec<u8>, FetchError> {
        if self.token != "secret" {
            return Err(FetchError::Unauthorized);
        }
        self.inner.fetch(key)
    }
}

// ---- B) Caching proxy -----------------------------------------------

pub struct CachingProxy<F: Fetcher> {
    inner: F,
    cache: RefCell<HashMap<String, Vec<u8>>>,
}
impl<F: Fetcher> CachingProxy<F> {
    pub fn new(inner: F) -> Self {
        Self { inner, cache: RefCell::new(HashMap::new()) }
    }
}
impl<F: Fetcher> Fetcher for CachingProxy<F> {
    fn fetch(&self, key: &str) -> Result<Vec<u8>, FetchError> {
        if let Some(hit) = self.cache.borrow().get(key).cloned() {
            println!("[cache] hit {key}");
            return Ok(hit);
        }
        let v = self.inner.fetch(key)?;
        self.cache.borrow_mut().insert(key.to_string(), v.clone());
        Ok(v)
    }
}

// ---- C) Virtual proxy (lazy) ----------------------------------------

pub struct LazyFetcher<F, B>
where
    F: Fetcher,
    B: Fn() -> F,
{
    builder: B,
    cache: RefCell<Option<F>>,
}

impl<F, B> LazyFetcher<F, B>
where
    F: Fetcher,
    B: Fn() -> F,
{
    pub fn new(builder: B) -> Self {
        Self { builder, cache: RefCell::new(None) }
    }
}

impl<F, B> Fetcher for LazyFetcher<F, B>
where
    F: Fetcher,
    B: Fn() -> F,
{
    fn fetch(&self, key: &str) -> Result<Vec<u8>, FetchError> {
        if self.cache.borrow().is_none() {
            println!("[lazy] constructing real Fetcher");
            *self.cache.borrow_mut() = Some((self.builder)());
        }
        self.cache.borrow().as_ref().unwrap().fetch(key)
    }
}

fn main() -> Result<(), FetchError> {
    let real = RealFetcher::with_data(vec![
        ("alice".to_string(), b"alice-data".to_vec()),
        ("bob".to_string(),   b"bob-data".to_vec()),
    ]);

    // A) Protection
    let authed = AuthProxy { inner: real, token: "secret".into() };

    // B) Caching, on top of the protection proxy
    let cached = CachingProxy::new(authed);

    println!("--- cold ---");
    let _ = cached.fetch("alice")?;
    println!("--- warm ---");
    let _ = cached.fetch("alice")?;        // served from cache
    let _ = cached.fetch("bob")?;

    // C) Virtual (lazy) — construction is deferred until first fetch
    let lazy = LazyFetcher::new(|| RealFetcher::with_data([(
        "lazy".into(), b"bytes".to_vec(),
    )]));
    println!("--- lazy ---");
    let _ = lazy.fetch("lazy")?;
    let _ = lazy.fetch("lazy")?;           // inner Fetcher already built

    Ok(())
}
