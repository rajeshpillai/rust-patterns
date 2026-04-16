// Broken example — two typical facade mistakes.
// This file is expected to FAIL to compile.
//
//   1. Return a private-module type from a public facade method.
//      Visibility bubbles up: if a function is `pub` but returns
//      `parser::Record` which is private, rustc rejects it as
//      "private type in public interface" (E0446).
//
//   2. Expose a "facade" method that panics on a recoverable error.
//      Compiles fine, but it's a library-design smell: a facade
//      that hides a failure from its public signature is lying
//      about its contract.

mod inner {
    pub(super) struct Record {
        pub id: u64,
    }
    pub(super) fn load() -> Record { Record { id: 1 } }
}

pub struct Facade;

impl Facade {
    // Mistake #1: public method, private return type.
    pub fn fetch(&self) -> inner::Record {
        //                ^^^^^^^^^^^^^ error[E0446]:
        //                private type `inner::Record` in public interface
        inner::load()
    }

    // Mistake #2 (logical): public method that SWALLOWS a recoverable
    // error by calling .unwrap(), turning a bad path into a panic
    // instead of returning a typed error to the caller.
    pub fn load_maybe(&self, path: &str) -> String {
        std::fs::read_to_string(path).unwrap()
        //                           ^^^^^^^^ not an error; a design bug.
        //                           Facades should RETURN Result, not
        //                           panic — see code/idiomatic.rs.
    }
}

fn main() {
    let _ = Facade.fetch();
}
