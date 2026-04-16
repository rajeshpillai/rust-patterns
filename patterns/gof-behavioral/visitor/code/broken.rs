// Broken example — two common Visitor footguns in Rust.
// This file is expected to FAIL to compile.
//
//   1. Forgetting that a `match` must be exhaustive. Adding a new
//      enum variant makes every non-exhaustive match a compile error
//      (E0004) — which is the useful behavior. A match that uses `_`
//      to catch "everything else" HIDES that signal.
//
//   2. Trying to call `accept` through `&dyn Element` when the
//      Visitor method takes `&mut self` and the Element method takes
//      `&self`. Borrow checker refuses reuse.

pub enum Expr {
    Num(i64),
    Var(String),
}

// Mistake #1: incomplete match without a wildcard.
// Imagine a developer added Expr::Add later and forgot to update eval.
pub fn eval(e: &Expr) -> i64 {
    match e {
        Expr::Num(n) => *n,
        // No Var arm, no wildcard. Compile error:
        //   error[E0004]: non-exhaustive patterns: `Var(_)` not covered
    }
}

// Mistake #2: the GoF form with borrow-checker tension.
pub trait Element {
    fn accept(&self, v: &mut dyn Visitor);
}
pub trait Visitor { fn visit_num(&mut self, n: i64); }

pub struct Number(i64);
impl Element for Number {
    fn accept(&self, v: &mut dyn Visitor) { v.visit_num(self.0); }
}

pub fn run_both(el: &dyn Element, v: &mut dyn Visitor) {
    el.accept(v);
    // Imagine we later add: el.accept(v) again for a side-effect.
    // That second call is fine here, but the moment the Visitor
    // stores *references* to Elements across calls, lifetimes bite:
    //   error[E0499]: cannot borrow `*v` as mutable more than once
}

fn main() {
    let _ = eval(&Expr::Num(1));
}
