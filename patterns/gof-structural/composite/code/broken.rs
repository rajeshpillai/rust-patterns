// Broken example — two shapes the compiler rejects.
// This file is expected to FAIL to compile.
//
//   1. Recursive enum without indirection. Rust needs to know the
//      size of a type at compile time. A variant that holds `Self`
//      directly would make the type infinitely sized (E0072).
//
//   2. Composite via `Vec<dyn Node>` inline — same unsized problem,
//      this time for trait objects. You need `Box<dyn Node>`.

// Mistake #1 — recursive type without Box/Vec.
pub enum Node {
    File { name: String, bytes: u64 },
    // No Box, no Vec, no indirection — Rust can't size this.
    Folder { name: String, child: Node },
    //                            ^^^^ error[E0072]: recursive type
    //                                 `Node` has infinite size
    //                                 help: insert some indirection
    //                                 (e.g., `Box`, `Rc`, or `&`)
}

// Mistake #2 — trait-object Composite with inline dyn.
pub trait NodeT {
    fn size(&self) -> u64;
}

pub struct Folder {
    pub children: Vec<dyn NodeT>,
    //            ^^^^^^^^^^^^^^ error[E0277]: the size for values of
    //                           type `(dyn NodeT + 'static)` cannot
    //                           be known at compilation time
    //                           — you need Vec<Box<dyn NodeT>>
}

fn main() {}
