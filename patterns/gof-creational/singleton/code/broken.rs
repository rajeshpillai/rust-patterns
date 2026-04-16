// Broken example — `static mut` without unsafe, and with unsafe in a
// way that races on any real multi-threaded access.
//
// Since Rust 2024 edition, reading/writing `static mut` is unsafe.
// The first function below won't compile at all. The second shows why
// people reach for `OnceLock` instead of muddling through with
// `unsafe` blocks and a prayer.
//
// This file is expected to FAIL to compile.

static mut COUNTER: u64 = 0;

// Mistake #1 — touching `static mut` without unsafe.
// The compiler rejects this: `static mut` is global mutable state
// and the language forces you to write `unsafe` to acknowledge the
// hazard.
pub fn bump_no_unsafe() {
    COUNTER += 1;
    //^^^^^^^ error[E0133]: use of mutable static is unsafe and
    //        requires unsafe function or block
}

// Mistake #2 — unsafe + `static mut` is a data race waiting to
// happen. Two threads can each read COUNTER, compute COUNTER+1, then
// both write back the *same* value. Rust will not stop this at
// compile time here because the unsafe block says "trust me", but
// the runtime behavior is undefined under concurrent access.
pub fn bump_unsafe() {
    unsafe { COUNTER += 1; }
}

// The correct form uses OnceLock<Mutex<u64>>. See code/idiomatic.rs
// for the full treatment.

fn main() {
    bump_no_unsafe();
    bump_unsafe();
}
