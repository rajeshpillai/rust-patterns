// Broken example — try to store a raw trait object without Box or &dyn,
// and try to reassign a Context's *generic* strategy at runtime. Both
// are honest mistakes when translating GoF mechanically.
//
// This file is expected to FAIL to compile.

pub trait Compress {
    fn compress(&self, input: &[u8]) -> Vec<u8>;
}

// ---- Mistake #1: store `dyn Compress` directly (not behind a pointer).
// Trait objects are unsized, so they cannot live inline in a struct.
pub struct UploaderBad1 {
    strategy: dyn Compress,
    //        ^^^^^^^^^^^^ error[E0277]: the size for values of type
    //                     `(dyn Compress + 'static)` cannot be known
    //                     at compilation time
}

// ---- Mistake #2: swap a *generic* strategy at runtime.
// The generic parameter is fixed at instantiation; `set_strategy`
// with a different type is impossible by design.
pub struct UploaderGeneric<S: Compress> {
    strategy: S,
}

impl<S: Compress> UploaderGeneric<S> {
    pub fn set_strategy<T: Compress>(&mut self, new: T) {
        self.strategy = new;
        //              ^^^ error[E0308]: mismatched types
        //              expected `S`, found `T`
    }
}

fn main() {}
