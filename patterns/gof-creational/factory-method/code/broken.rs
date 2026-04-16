// Broken example — two factory-method footguns.
// This file is expected to FAIL to compile.
//
//   1. Returning `impl Trait` from a function whose body produces
//      two different concrete types in different branches. The
//      `impl` return-type is a single opaque *compile-time-known*
//      type; a match that produces Json on one arm and Yaml on
//      another is an error.
//
//   2. Trying to use `Box<dyn Trait>` where the trait is NOT
//      object-safe (e.g., has an associated type or a generic
//      method). Object-safety rules apply.

pub trait Formatter {
    fn extension(&self) -> &'static str;
}
pub struct Json; impl Formatter for Json { fn extension(&self) -> &'static str { "json" } }
pub struct Yaml; impl Formatter for Yaml { fn extension(&self) -> &'static str { "yaml" } }

pub enum Kind { Json, Yaml }

// Mistake #1 — `impl Formatter` forces one concrete type for all branches.
pub fn bad_factory(kind: Kind) -> impl Formatter {
    match kind {
        Kind::Json => Json,
        //            ^^^^ expected Json
        Kind::Yaml => Yaml,
        //            ^^^^ error[E0308]: mismatched types —
        //                 expected struct `Json`, found struct `Yaml`
    }
}

// Mistake #2 — a trait with an associated type is NOT object-safe,
// so you can't return `Box<dyn Creator>`.
pub trait Creator {
    type Product;
    fn create(&self) -> Self::Product;
}

pub fn bad_box() -> Box<dyn Creator> {
    //              ^^^^^^^^^^^^^^^^ error[E0038]: the trait `Creator`
    //                               cannot be made into an object
    //                               because its type parameter / associated
    //                               type cannot be known at vtable call time
    unimplemented!()
}

fn main() {}
