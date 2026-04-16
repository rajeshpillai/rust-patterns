// Broken example — two Proxy footguns in Rust.
// This file is expected to FAIL to compile.
//
//   1. Forget that Proxy must share the exact trait of the real type.
//      If the Proxy signature differs, it's not a Proxy — callers
//      can't use it transparently.
//
//   2. Mutate the cache through `&self` without interior mutability.
//      Caching proxies need Cell/RefCell/Mutex; otherwise the cache
//      field becomes read-only.

use std::collections::HashMap;

pub trait Fetcher {
    fn fetch(&self, key: &str) -> Option<Vec<u8>>;
}

pub struct RealFetcher {
    data: HashMap<String, Vec<u8>>,
}
impl Fetcher for RealFetcher {
    fn fetch(&self, key: &str) -> Option<Vec<u8>> { self.data.get(key).cloned() }
}

// Mistake #1 — Proxy's fetch has a different signature (extra arg),
// so it's not a Fetcher impl. Any call site that expects `&dyn Fetcher`
// breaks, and `impl Fetcher for Proxy` can't be written.
pub struct WrongProxy<F: Fetcher> { pub inner: F }
impl<F: Fetcher> WrongProxy<F> {
    pub fn fetch(&self, key: &str, tracing_id: u64) -> Option<Vec<u8>> {
        self.inner.fetch(key)
    }
}

// Good API shape would reuse the Fetcher trait so WrongProxy would
// `impl Fetcher`. Since fetch signatures differ, we get:
//
//   let p: &dyn Fetcher = &wrong_proxy;
//   //                    ^^^^^^^^^^^^ error[E0277]: the trait `Fetcher`
//   //                                 is not implemented for `WrongProxy<...>`

// Mistake #2 — mutate cache through &self without interior mutability.
pub struct CachingProxy<F: Fetcher> {
    inner: F,
    cache: HashMap<String, Vec<u8>>,  // plain HashMap, no RefCell
}
impl<F: Fetcher> Fetcher for CachingProxy<F> {
    fn fetch(&self, key: &str) -> Option<Vec<u8>> {
        if let Some(v) = self.cache.get(key) { return Some(v.clone()); }
        let v = self.inner.fetch(key)?;
        self.cache.insert(key.to_string(), v.clone());
        //          ^^^^^^ error[E0596]: cannot borrow `self.cache` as
        //                 mutable, as it is behind a `&` reference.
        //                 The fix is `cache: RefCell<HashMap<...>>`.
        Some(v)
    }
}

fn main() {}
