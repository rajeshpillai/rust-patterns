// Broken example — three common Bridge mistakes.
// This file is expected to FAIL to compile.
//
//   1. Storing a raw `dyn Renderer` inline in the abstraction.
//      Trait objects are unsized; need Box<dyn Renderer> or a
//      generic parameter.
//
//   2. Accidentally mixing the "abstraction × implementation"
//      quadratic by hard-coding an impl. Compiles, but the pattern
//      is gone.
//
//   3. Trying to store multiple different renderer impls in a Vec
//      without trait-object boxing.

pub trait Renderer {
    fn heading(&self, t: &str) -> String;
}

// Mistake #1 — unsized dyn in a struct field.
pub struct Message {
    pub title: String,
    pub renderer: dyn Renderer,
    //            ^^^^^^^^^^^^ error[E0277]: the size for values of
    //                          type `(dyn Renderer + 'static)` cannot
    //                          be known at compilation time
}

// Mistake #2 — hard-coded Renderer type removes the Bridge.
pub struct Plain;
impl Renderer for Plain { fn heading(&self, t: &str) -> String { t.to_string() } }

pub struct HardCodedMessage {
    pub title: String,
    pub renderer: Plain,
    //            ^^^^^ compiles, but you've killed the pattern.
    //            A downstream caller can't swap the renderer.
}

// Mistake #3 — Vec of mixed concrete renderers.
pub struct Html;
impl Renderer for Html { fn heading(&self, t: &str) -> String { format!("<h1>{t}</h1>") } }

pub fn all_renderers() -> Vec<_> {
    vec![Plain, Html]
    //   ^^^^^ error[E0308]: mismatched types — expected struct `Plain`,
    //                       found struct `Html`
    //   Fix: vec![Box::new(Plain) as Box<dyn Renderer>, Box::new(Html)]
}

fn main() {}
