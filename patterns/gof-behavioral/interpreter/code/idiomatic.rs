// Idiomatic Rust Interpreter — one enum for the AST, one function
// per "interpretation". Rust's match replaces the Visitor double-
// dispatch the classical Interpreter pattern requires in most
// languages.

use std::collections::HashMap;

pub type Env = HashMap<String, i64>;

#[derive(Debug, Clone)]
pub enum Expr {
    Num(i64),
    Var(String),
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

// Ergonomic constructors — let `(a + b) * c` read naturally in code.
impl Expr {
    pub fn num(n: i64)                  -> Self { Expr::Num(n) }
    pub fn var(s: impl Into<String>)    -> Self { Expr::Var(s.into()) }
    pub fn add(a: Expr, b: Expr)        -> Self { Expr::Add(Box::new(a), Box::new(b)) }
    pub fn sub(a: Expr, b: Expr)        -> Self { Expr::Sub(Box::new(a), Box::new(b)) }
    pub fn mul(a: Expr, b: Expr)        -> Self { Expr::Mul(Box::new(a), Box::new(b)) }
}

// ---- Interpretation #1: evaluator --------------------------------------

#[derive(Debug)]
#[non_exhaustive]
pub enum EvalError { UnknownVar(String) }

pub fn eval(e: &Expr, env: &Env) -> Result<i64, EvalError> {
    match e {
        Expr::Num(n) => Ok(*n),
        Expr::Var(name) => env.get(name)
            .copied()
            .ok_or_else(|| EvalError::UnknownVar(name.clone())),
        Expr::Add(a, b) => Ok(eval(a, env)? + eval(b, env)?),
        Expr::Sub(a, b) => Ok(eval(a, env)? - eval(b, env)?),
        Expr::Mul(a, b) => Ok(eval(a, env)? * eval(b, env)?),
    }
}

// ---- Interpretation #2: pretty-printer --------------------------------

pub fn pretty(e: &Expr) -> String {
    match e {
        Expr::Num(n) => n.to_string(),
        Expr::Var(name) => name.clone(),
        Expr::Add(a, b) => format!("({} + {})", pretty(a), pretty(b)),
        Expr::Sub(a, b) => format!("({} - {})", pretty(a), pretty(b)),
        Expr::Mul(a, b) => format!("({} * {})", pretty(a), pretty(b)),
    }
}

// ---- Interpretation #3: constant folding ------------------------------

pub fn fold(e: &Expr) -> Expr {
    match e {
        Expr::Num(_) | Expr::Var(_) => e.clone(),
        Expr::Add(a, b) => bin(fold(a), fold(b), |x, y| x + y, Expr::Add),
        Expr::Sub(a, b) => bin(fold(a), fold(b), |x, y| x - y, Expr::Sub),
        Expr::Mul(a, b) => bin(fold(a), fold(b), |x, y| x * y, Expr::Mul),
    }
}

fn bin<F, C>(l: Expr, r: Expr, op: F, ctor: C) -> Expr
where
    F: Fn(i64, i64) -> i64,
    C: Fn(Box<Expr>, Box<Expr>) -> Expr,
{
    match (&l, &r) {
        (Expr::Num(a), Expr::Num(b)) => Expr::Num(op(*a, *b)),
        _ => ctor(Box::new(l), Box::new(r)),
    }
}

// ---- Interpretation #4: free variables list ----------------------------

pub fn free_vars(e: &Expr, out: &mut std::collections::BTreeSet<String>) {
    match e {
        Expr::Num(_) => {},
        Expr::Var(name) => { out.insert(name.clone()); }
        Expr::Add(a, b) | Expr::Sub(a, b) | Expr::Mul(a, b) => {
            free_vars(a, out);
            free_vars(b, out);
        }
    }
}

fn main() -> Result<(), EvalError> {
    // (x + 2) * (5 - y)
    let expr = Expr::mul(
        Expr::add(Expr::var("x"), Expr::num(2)),
        Expr::sub(Expr::num(5), Expr::var("y")),
    );

    let mut env = Env::new();
    env.insert("x".into(), 3);
    env.insert("y".into(), 1);

    println!("source:     {}", pretty(&expr));
    println!("eval:       {}", eval(&expr, &env)?);

    // Constant folding on (3 + 2) is a no-op here because x and y
    // are Vars, but (2 + 3) or (5 - 0) inside would fold to a Num.
    let e2 = Expr::mul(Expr::add(Expr::num(2), Expr::num(3)), Expr::num(5));
    println!("source:     {}", pretty(&e2));
    println!("folded:     {}", pretty(&fold(&e2)));

    let mut vs = std::collections::BTreeSet::new();
    free_vars(&expr, &mut vs);
    println!("free vars:  {:?}", vs);

    // Unknown variable — typed error, not panic.
    let err = eval(&Expr::var("unknown"), &Env::new()).unwrap_err();
    println!("expected:   {err:?}");
    Ok(())
}
