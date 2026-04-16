// Singleton — four Rust forms, from weakest to strongest. Prefer the
// first form that fits; every extra mechanism buys you something and
// costs you something.
//
//   1. `const`  — compile-time-known, zero overhead. The best form.
//   2. `static` — runtime-initialized if it can be written as a
//                 literal expression (no allocation).
//   3. `OnceLock<T>` — lazy init exactly once, thread-safe. std, no deps.
//   4. `LazyLock<T>` — OnceLock + init closure, auto-deref. std, 1.80+.
//
// NOT shown: `static mut` — unsafe, race-condition-prone, no thread
// safety guarantees. If you think you need it, you almost certainly
// want OnceLock<Mutex<T>> instead. See [Interior Mutability] for the
// pattern when the singleton itself must be mutable.

use std::sync::{LazyLock, Mutex, OnceLock};

// ---- 1. const — pure compile-time constant ----------------------------

pub const MAX_RETRIES: u32 = 3;
pub const APP_NAME: &str = "rust-patterns";

// ---- 2. static — runtime-initialized, literal expressions only --------

pub static BUILD_CHANNEL: &str = "stable";

// ---- 3. OnceLock — lazy init exactly once -----------------------------

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub port: u16,
}

fn config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| {
        // In a real app, read env vars / config file. This runs AT
        // MOST once, regardless of how many threads call config()
        // concurrently.
        println!("[init] loading config");
        Config {
            database_url: std::env::var("DATABASE_URL")
                .unwrap_or_else(|_| "postgres://localhost/dev".to_string()),
            port: 8080,
        }
    })
}

// ---- 4. LazyLock — syntactic sugar over OnceLock + closure -----------

// Ergonomic for call sites that want `&*MESSAGES` rather than function-
// call syntax. `LazyLock<T>` derefs to `T` on demand.
pub static MESSAGES: LazyLock<Vec<&'static str>> = LazyLock::new(|| {
    println!("[init] building messages");
    vec!["hello", "world", "patterns"]
});

// ---- Mutable singleton — only when absolutely required ----------------

// If the singleton's state genuinely must be mutable and shared, wrap
// its value in a Mutex. The OnceLock guarantees the Mutex is
// constructed exactly once; the Mutex guarantees exclusive access.
//
// Question your choices first — a mutable process-wide singleton is
// global state, which is hard to test and reason about. Prefer
// threading the value explicitly through your call stack.
fn metrics() -> &'static Mutex<u64> {
    static COUNTER: OnceLock<Mutex<u64>> = OnceLock::new();
    COUNTER.get_or_init(|| Mutex::new(0))
}

fn main() {
    println!("app = {APP_NAME}, channel = {BUILD_CHANNEL}, max_retries = {MAX_RETRIES}");

    // Lazy-loaded config
    let cfg = config();
    println!("config: {:?}", cfg);
    let cfg2 = config();
    println!("same pointer? {}", std::ptr::eq(cfg, cfg2)); // true

    // LazyLock
    for m in MESSAGES.iter() {
        println!("msg: {m}");
    }

    // Mutable singleton
    {
        let mut c = metrics().lock().expect("metrics mutex poisoned");
        *c += 1;
    }
    {
        let c = metrics().lock().expect("metrics mutex poisoned");
        println!("counter = {}", *c);
    }
    // NOTE: `expect` on a poisoned mutex is acceptable here because a
    // poisoned counter is a program bug, not a recoverable condition.
    // For user-facing paths, handle `PoisonError` explicitly.
}
