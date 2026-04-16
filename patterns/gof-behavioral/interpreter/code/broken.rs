// Broken example — three Interpreter-in-Rust traps.
// This file is expected to FAIL to compile.
//
//   1. Recursive enum without Box — infinite size (E0072).
//   2. Non-exhaustive match on the AST — E0004.
//   3. Stack-overflow-prone deep recursion for pathological inputs.
//      Not a compile error; the teaching point is "tree-walking
//      interpreters need a plan for deep trees."

// Mistake #1 — Expr contains Expr directly, not Box<Expr>.
pub enum Expr {
    Num(i64),
    Add(Expr, Expr),
    //   ^^^^ error[E0072]: recursive type `Expr` has infinite size
    //   help: insert indirection (e.g., Box)
}

// Mistake #2 — add a new variant (Sub) but forget to update eval().
pub enum Expr2 {
    Num(i64),
    Add(Box<Expr2>, Box<Expr2>),
    Sub(Box<Expr2>, Box<Expr2>),
}

pub fn eval(e: &Expr2) -> i64 {
    match e {
        Expr2::Num(n) => *n,
        Expr2::Add(a, b) => eval(a) + eval(b),
        // Forgot Sub:
        // error[E0004]: non-exhaustive patterns: `Sub(_, _)` not covered
        //
        // This is the exhaustiveness check doing its job — it forces
        // you to touch every `eval`-like function when you add a
        // grammar rule. That's the whole point of the enum form.
    }
}

// Mistake #3 — deep recursion without a plan. For a tree with
// 1,000,000 nested Adds, this blows the stack on typical platforms.
pub fn deep_recursive(depth: u32) -> u64 {
    if depth == 0 { 0 } else { 1 + deep_recursive(depth - 1) }
    // Compiles; at depth ~60k on default-stack Linux threads, panics
    // with "thread main has overflowed its stack". Real interpreters
    // either use an explicit work stack or a dedicated large-stack
    // thread (std::thread::Builder::stack_size).
}

fn main() {}
