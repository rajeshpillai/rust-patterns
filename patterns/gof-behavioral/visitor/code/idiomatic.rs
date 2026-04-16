// Idiomatic Rust "Visitor" — it's just `match`. The enum IS the
// element hierarchy, and each operation is a function with a
// recursive match. Adding a new operation = add a new function.
// Adding a new variant = every match gets a compile error, which
// is exactly the refactor assistant you want.
//
// This is strictly better than the GoF trait Visitor when:
//   * You own the element types.
//   * The variants are a closed set.
//   * Operations come and go, variants don't.

use std::collections::HashMap;

#[derive(Debug)]
pub enum Expr {
    Num(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
}

// ---- Operations: each is just a recursive function. -----------------

pub fn eval(e: &Expr, env: &HashMap<String, i64>) -> i64 {
    match e {
        Expr::Num(n) => *n,
        Expr::Var(name) => *env.get(name).unwrap_or(&0),
        Expr::Add(l, r) => eval(l, env) + eval(r, env),
    }
}

pub fn print(e: &Expr) -> String {
    match e {
        Expr::Num(n) => n.to_string(),
        Expr::Var(name) => name.clone(),
        Expr::Add(l, r) => format!("({} + {})", print(l), print(r)),
    }
}

// Optimization pass — constant folding. Returns a new Expr tree.
pub fn fold_constants(e: &Expr) -> Expr {
    match e {
        Expr::Num(n) => Expr::Num(*n),
        Expr::Var(n) => Expr::Var(n.clone()),
        Expr::Add(l, r) => {
            let l = fold_constants(l);
            let r = fold_constants(r);
            match (&l, &r) {
                (Expr::Num(a), Expr::Num(b)) => Expr::Num(a + b),
                _ => Expr::Add(Box::new(l), Box::new(r)),
            }
        }
    }
}

// Generic visitor via a mutable callback — folds over the tree
// without building a new one.
pub fn walk(e: &Expr, f: &mut impl FnMut(&Expr)) {
    f(e);
    if let Expr::Add(l, r) = e {
        walk(l, f);
        walk(r, f);
    }
}

fn main() {
    // (x + (1 + 2))
    let e = Expr::Add(
        Box::new(Expr::Var("x".into())),
        Box::new(Expr::Add(Box::new(Expr::Num(1)), Box::new(Expr::Num(2)))),
    );

    let mut env = HashMap::new();
    env.insert("x".into(), 10);

    println!("print:       {}", print(&e));
    println!("eval:        {}", eval(&e, &env));
    println!("constfold:   {}", print(&fold_constants(&e)));

    let mut count = 0;
    walk(&e, &mut |_| count += 1);
    println!("node count:  {count}");
}
