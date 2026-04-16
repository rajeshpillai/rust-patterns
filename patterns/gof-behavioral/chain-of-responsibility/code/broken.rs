// Broken example — two common Chain of Responsibility traps.
// This file is expected to FAIL to compile.
//
//   1. Store handlers as `Vec<dyn Handler>` instead of
//      `Vec<Box<dyn Handler>>`. Trait objects are unsized; you need
//      indirection.
//
//   2. Build a self-referential "next" link without handling
//      ownership — each Handler holding `next: Box<dyn Handler>`
//      gives you a linked list, but appending requires reaching
//      down to the tail, which is awkward without a separate
//      builder.

pub trait Handler {
    fn call(&self);
}

// Mistake #1 — inline dyn in a collection.
pub struct Chain {
    pub handlers: Vec<dyn Handler>,
    //            ^^^^^^^^^^^^^^^^ error[E0277]: the size for values
    //                              of type `(dyn Handler + 'static)`
    //                              cannot be known at compilation time
}

// Mistake #2 — every Handler stores its own `next`. Adding a layer
// requires modifying the LAST handler to point at the new one, which
// needs recursion + ownership shuffling. The pattern compiles, but
// writing "append a handler" is a mess. The `Vec<Box<dyn Handler>>`
// form (code/idiomatic.rs) sidesteps this entirely.
pub struct LinkedHandler {
    pub next: Option<Box<LinkedHandler>>,
    pub name: String,
}

impl LinkedHandler {
    pub fn append(self, extra: LinkedHandler) -> LinkedHandler {
        // If self has no `next`, attach extra as self.next. Otherwise
        // recurse into self.next. This compiles, but now try to write
        // it WITHOUT taking `self` by value — `&mut self` + the
        // recursive `self.next.as_mut()?.append(extra)` fights the
        // borrow checker because the append borrow spans the
        // recursion.
        match self.next {
            None => LinkedHandler { next: Some(Box::new(extra)), ..self },
            Some(tail) => {
                // E0499 if you try this with &mut self — cannot borrow
                // `*tail` as mutable more than once at a time.
                LinkedHandler {
                    next: Some(Box::new(tail.append(extra))),
                    ..self
                }
            }
        }
    }
}

fn main() {}
