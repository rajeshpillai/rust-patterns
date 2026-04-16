// Broken example — the two real Prototype-in-Rust mistakes.
// This file is expected to FAIL to compile.
//
//   1. Try to `.clone()` a type that doesn't implement `Clone`.
//   2. Put `dyn Clone` behind a `Box` and expect it to work.
//      `Clone` is not object-safe because `clone(&self) -> Self`
//      references Self.

pub struct Secret {
    pub bytes: Vec<u8>,
}
// Intentionally no `#[derive(Clone)]` — secrets shouldn't be
// casually duplicated.

pub fn duplicate_secret(s: &Secret) -> Secret {
    s.clone()
    //^^^^^^ error[E0599]: no method named `clone` found for reference
    //       `&Secret` in the current scope
    //       note: the following trait defines an item named `clone`,
    //             perhaps you need to implement it: `Clone`
}

// Mistake #2: trying to box a `dyn Clone`.
pub trait Shape: Clone { fn area(&self) -> f64; }

pub fn pick_shape() -> Box<dyn Shape> {
    //                  ^^^^^^^^^^^^^ error[E0038]: the trait `Shape`
    //                                cannot be made into an object
    //                                because it requires `Self: Sized`
    //                                (via the Clone supertrait).
    unimplemented!()
}

fn main() {}
