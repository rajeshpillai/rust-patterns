// Broken example — asking for `Fn` when the closure is really `FnMut`,
// and two closures of the same shape having incompatible types.
//
// This file is expected to FAIL to compile.

// Mistake #1: the function bound is Fn (can be called via &self), but
// the closure captures `count` by mutable reference (it's FnMut). A
// closure that mutates its environment cannot implement Fn.
pub fn run_twice<F: Fn()>(f: F) {
    f();
    f();
}

// Mistake #2: two closures with identical signatures still have
// distinct, anonymous types. They cannot be pushed into the same
// Vec<_>. You need a unifying trait object — `Vec<Box<dyn Fn()>>`.
pub fn build_list() {
    let greet = || println!("hello");
    let bye   = || println!("bye");
    let _hooks = vec![greet, bye];
    //         ^^^^^ error[E0308]: expected closure, found a different closure
}

fn main() {
    let mut count = 0;

    // Tries to use a closure that mutates `count` where a `Fn` is
    // required. Compiler refuses:
    //
    //   error[E0525]: expected a closure that implements the `Fn`
    //                 trait, but this closure only implements `FnMut`
    run_twice(|| count += 1);

    build_list();
}
