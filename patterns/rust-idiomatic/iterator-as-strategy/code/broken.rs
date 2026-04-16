// Broken example — two common Iterator-as-Strategy mistakes.
// This file is expected to FAIL to compile.
//
//   1. `sort_by` wants a `FnMut(&T, &T) -> Ordering`, but this
//      closure returns `bool`. Picking the wrong closure signature
//      for an adapter is the #1 Iterator footgun.
//
//   2. Trying to collect two differently-typed closures into one
//      Vec. Every closure has a distinct anonymous type — even
//      with the same signature — so they don't unify.

use std::cmp::Ordering;

fn main() {
    // Mistake #1: sort_by with a `bool`-returning closure (the Python
    // / Ruby sort-key shape). Rust's sort_by wants an Ordering.
    let mut v = vec![3, 1, 4, 1, 5];
    v.sort_by(|a, b| a < b);
    //        ^^^^^^^^^^^^^ error[E0308]: mismatched types
    //                      expected `Ordering`, found `bool`
    //
    // The fix is either:
    //   v.sort_by(|a, b| a.cmp(b));          // explicit Ordering
    //   v.sort_by_key(|&x| x);               // simpler for sort-key uses
    println!("{v:?}");

    // Mistake #2: put two distinctly-typed closures in one Vec.
    let square = |x: i32| x * x;
    let double = |x: i32| x * 2;
    let strategies = vec![square, double];
    //               ^^^^^^^^^^^^^^^^^^^^^ error[E0308]: mismatched types
    //                                     — each closure has a unique type
    //
    // The fix is a trait object: Vec<Box<dyn Fn(i32) -> i32>>
    for f in &strategies { println!("{}", f(5)); }
}
