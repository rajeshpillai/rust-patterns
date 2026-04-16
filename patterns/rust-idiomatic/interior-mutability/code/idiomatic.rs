// Interior mutability — mutate through a shared reference when you
// MUST. The default should always be `&mut self` + ownership. Reach
// for these tools only when the access pattern genuinely demands
// shared state + mutation.
//
// This file covers the single-threaded tools (Cell, RefCell) and the
// multi-threaded ones (Mutex, RwLock, OnceLock, atomics). Each shows
// a tiny example with the idiomatic access pattern.

use std::cell::{Cell, RefCell};
use std::sync::{Mutex, OnceLock, RwLock};
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

// ---- Cell<T> — single-threaded, T: Copy --------------------------------

// A per-struct counter that external code can increment even though
// we hand out only `&Counter`. Cell works because u64 is Copy, so
// .get() returns a *value*, not a borrow.
pub struct Counter {
    hits: Cell<u64>,
}

impl Counter {
    pub fn new() -> Self { Self { hits: Cell::new(0) } }
    pub fn bump(&self) { self.hits.set(self.hits.get() + 1); }
    pub fn value(&self) -> u64 { self.hits.get() }
}

// ---- RefCell<T> — single-threaded, dynamic borrow-check ---------------

// The classic "graph with shared children" shape. We use RefCell so
// we can mutate the children through a shared reference when adding
// edges. .borrow() / .borrow_mut() panic if the aliasing rule is
// broken — which is a runtime check, not a compile one. The
// preferred alternative is almost always to restructure the data to
// avoid sharing + mutation.
pub struct Graph {
    nodes: RefCell<Vec<String>>,
}

impl Graph {
    pub fn new() -> Self { Self { nodes: RefCell::new(Vec::new()) } }
    pub fn add(&self, name: impl Into<String>) {
        self.nodes.borrow_mut().push(name.into());
    }
    pub fn count(&self) -> usize { self.nodes.borrow().len() }
}

// ---- Mutex<T> — multi-threaded, blocking -------------------------------

// Threaded request counter. Every worker locks, increments, unlocks.
// The counter's value is only accurate *between* locks. For pure
// integer counters, AtomicU64 is faster (see below).
pub struct RequestCounter {
    inner: Mutex<u64>,
}

impl RequestCounter {
    pub fn new() -> Self { Self { inner: Mutex::new(0) } }
    pub fn bump(&self) {
        // expect() on a poisoned lock is acceptable here — a poisoned
        // counter is a program bug, not a recoverable condition.
        let mut n = self.inner.lock().expect("counter mutex poisoned");
        *n += 1;
    }
    pub fn value(&self) -> u64 {
        *self.inner.lock().expect("counter mutex poisoned")
    }
}

// ---- RwLock<T> — multi-threaded, many readers XOR one writer ----------

// Configuration that is mostly read, occasionally swapped.
pub struct ConfigStore {
    inner: RwLock<String>,
}

impl ConfigStore {
    pub fn new(initial: &str) -> Self { Self { inner: RwLock::new(initial.into()) } }
    pub fn get(&self) -> String {
        self.inner.read().expect("config rwlock poisoned").clone()
    }
    pub fn set(&self, new: String) {
        *self.inner.write().expect("config rwlock poisoned") = new;
    }
}

// ---- Atomic* — multi-threaded, lock-free, single-word types ----------

// For a plain counter, atomics beat Mutex<u64>. Ordering::Relaxed is
// correct for counters because we don't need cross-thread
// happens-before for the counter itself; if we did, AcqRel / SeqCst.
pub struct AtomicCounter {
    v: AtomicU64,
}

impl AtomicCounter {
    pub fn new() -> Self { Self { v: AtomicU64::new(0) } }
    pub fn bump(&self) { self.v.fetch_add(1, Ordering::Relaxed); }
    pub fn value(&self) -> u64 { self.v.load(Ordering::Relaxed) }
}

// ---- OnceLock<T> — init exactly once ---------------------------------

// Lazy, thread-safe initialization. See also the Singleton pattern.
fn config() -> &'static String {
    static CONFIG: OnceLock<String> = OnceLock::new();
    CONFIG.get_or_init(|| "loaded-once".to_string())
}

fn main() {
    // Cell
    let c = Counter::new();
    c.bump(); c.bump();
    println!("counter = {}", c.value());

    // RefCell
    let g = Graph::new();
    g.add("a"); g.add("b");
    println!("graph has {} nodes", g.count());

    // Mutex + threads
    let shared = std::sync::Arc::new(RequestCounter::new());
    let handles: Vec<_> = (0..4).map(|_| {
        let s = shared.clone();
        thread::spawn(move || for _ in 0..100 { s.bump(); })
    }).collect();
    for h in handles { h.join().unwrap(); }
    println!("mutex counter = {}", shared.value());

    // RwLock
    let cfg = std::sync::Arc::new(ConfigStore::new("v1"));
    cfg.set("v2".into());
    println!("config = {}", cfg.get());

    // AtomicU64 is the right choice for this exact pattern
    let atomic = std::sync::Arc::new(AtomicCounter::new());
    let handles: Vec<_> = (0..4).map(|_| {
        let a = atomic.clone();
        thread::spawn(move || for _ in 0..100 { a.bump(); })
    }).collect();
    for h in handles { h.join().unwrap(); }
    println!("atomic counter = {}", atomic.value());

    // OnceLock
    println!("config = {}", config());
}
