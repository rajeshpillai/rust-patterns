// Broken example — two Template Method footguns.
// This file is expected to FAIL to compile.
//
//   1. Adding a new required method to a trait that already has
//      concrete impls breaks every downstream impl. Without a
//      default body, the impl blocks fail to compile.
//
//   2. Calling a generic method from a default trait method that's
//      supposed to be object-safe — any generic fn in a trait
//      breaks object-safety.

pub trait DataPipeline {
    fn run(&self) -> String {
        let bytes = self.load();
        self.emit(&bytes)
    }
    fn load(&self) -> Vec<u8>;
    fn emit(&self, bytes: &[u8]) -> String;
    fn checksum(&self, bytes: &[u8]) -> u32;
    //^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    //                     NEW REQUIRED METHOD — no default body.
}

// This impl existed before `checksum` was added. It now fails to
// compile with E0046 until `checksum` is implemented.
pub struct CsvPipeline;
impl DataPipeline for CsvPipeline {
    fn load(&self) -> Vec<u8> { Vec::new() }
    fn emit(&self, _bytes: &[u8]) -> String { String::new() }
    //^ error[E0046]: not all trait items implemented, missing: `checksum`
}

// Mistake #2: generic method inside an object-safe trait.
pub trait Sink {
    fn emit<T: ToString>(&self, item: T);
    //      ^^^^^^^^^^^ object-safety violation if we then want
    //                   `Box<dyn Sink>`.
}
pub fn make() -> Box<dyn Sink> {
    //           ^^^^^^^^^^^^^ error[E0038]: the trait `Sink` cannot
    //                         be made into an object because method
    //                         `emit` has generic type parameters
    unimplemented!()
}

fn main() {}
