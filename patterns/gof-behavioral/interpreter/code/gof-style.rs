// Classical GoF Interpreter — a trait per grammar rule? No: GoF
// actually says ONE trait (AbstractExpression) with an interpret
// method, and one CLASS per grammar rule. Rust port: a trait +
// one impl per rule. Works, but Box<dyn Expression> for every child
// is a heap allocation per node, and you lose the exhaustive `match`.
//
// See code/idiomatic.rs for the enum + match form that Rust prefers.

use std::collections::HashMap;

pub trait Expression {
    fn interpret(&self, env: &HashMap<String, i64>) -> i64;
}

pub struct Num(pub i64);
pub struct Var(pub String);
pub struct Add(pub Box<dyn Expression>, pub Box<dyn Expression>);
pub struct Mul(pub Box<dyn Expression>, pub Box<dyn Expression>);

impl Expression for Num {
    fn interpret(&self, _: &HashMap<String, i64>) -> i64 { self.0 }
}
impl Expression for Var {
    fn interpret(&self, env: &HashMap<String, i64>) -> i64 {
        *env.get(&self.0).unwrap_or(&0)
    }
}
impl Expression for Add {
    fn interpret(&self, env: &HashMap<String, i64>) -> i64 {
        self.0.interpret(env) + self.1.interpret(env)
    }
}
impl Expression for Mul {
    fn interpret(&self, env: &HashMap<String, i64>) -> i64 {
        self.0.interpret(env) * self.1.interpret(env)
    }
}

fn main() {
    // (x + 2) * 5 with x = 3
    let expr: Box<dyn Expression> = Box::new(Mul(
        Box::new(Add(
            Box::new(Var("x".into())),
            Box::new(Num(2)),
        )),
        Box::new(Num(5)),
    ));

    let mut env = HashMap::new();
    env.insert("x".into(), 3);

    println!("result = {}", expr.interpret(&env));
    // Every node is in a Box<dyn Expression>. Four heap allocations,
    // four vtable lookups per eval, no exhaustive check when you
    // add a new rule. See idiomatic.rs for the enum alternative.
}
