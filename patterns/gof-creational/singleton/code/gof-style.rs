// Direct GoF translation — private constructor + static getInstance.
// Rust's modules replace "private constructor" with module-private
// visibility, and `OnceLock` replaces the double-checked-locking dance
// that Java/C++ examples of this pattern are famous for.
//
// The shape matches the GoF description exactly; see code/idiomatic.rs
// for the full menu of Rust-specific forms.

use std::sync::OnceLock;

pub struct Logger {
    // pretend this is a real sink
    prefix: &'static str,
}

impl Logger {
    // Module-private constructor — callers outside this module cannot
    // construct a Logger directly.
    fn new() -> Self {
        Self { prefix: "[LOG]" }
    }

    /// The GoF `getInstance()`. Thread-safe init via std's OnceLock —
    /// no `lazy_static` or `once_cell` crate required since Rust 1.70.
    pub fn get_instance() -> &'static Logger {
        static INSTANCE: OnceLock<Logger> = OnceLock::new();
        INSTANCE.get_or_init(Logger::new)
    }

    pub fn log(&self, msg: &str) {
        println!("{} {}", self.prefix, msg);
    }
}

fn main() {
    Logger::get_instance().log("first call");
    Logger::get_instance().log("second call");
    // Same Logger both times.
    let a: *const Logger = Logger::get_instance();
    let b: *const Logger = Logger::get_instance();
    assert!(std::ptr::eq(a, b));
}
