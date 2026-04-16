// Classical GoF Visitor — trait Visitor + trait Element with accept().
// Each Element subtype's accept() double-dispatches to the matching
// visit_<Element> method on the Visitor.
//
// This is the RIGHT choice only when downstream crates need to add
// new Visitor operations over element types they don't own. For the
// common case (you own the element types), the enum+match form in
// code/idiomatic.rs is strictly better.

use std::collections::HashMap;

// ---- Element hierarchy -----------------------------------------------

pub trait Element {
    fn accept(&self, v: &mut dyn Visitor);
}

pub struct Num(pub i64);
pub struct Add(pub Box<dyn Element>, pub Box<dyn Element>);
pub struct Var(pub String);

impl Element for Num { fn accept(&self, v: &mut dyn Visitor) { v.visit_num(self); } }
impl Element for Add { fn accept(&self, v: &mut dyn Visitor) { v.visit_add(self); } }
impl Element for Var { fn accept(&self, v: &mut dyn Visitor) { v.visit_var(self); } }

// ---- Visitor trait --------------------------------------------------

pub trait Visitor {
    fn visit_num(&mut self, n: &Num);
    fn visit_add(&mut self, a: &Add);
    fn visit_var(&mut self, v: &Var);
}

// ---- Concrete Visitor: Evaluator ------------------------------------

pub struct Evaluator<'a> {
    env: &'a HashMap<String, i64>,
    stack: Vec<i64>,
}

impl<'a> Evaluator<'a> {
    pub fn new(env: &'a HashMap<String, i64>) -> Self { Self { env, stack: Vec::new() } }
    pub fn result(&self) -> i64 { *self.stack.last().unwrap_or(&0) }
}

impl Visitor for Evaluator<'_> {
    fn visit_num(&mut self, n: &Num) { self.stack.push(n.0); }
    fn visit_add(&mut self, a: &Add) {
        a.0.accept(self);
        a.1.accept(self);
        let r = self.stack.pop().unwrap_or(0);
        let l = self.stack.pop().unwrap_or(0);
        self.stack.push(l + r);
    }
    fn visit_var(&mut self, v: &Var) {
        let x = *self.env.get(&v.0).unwrap_or(&0);
        self.stack.push(x);
    }
}

// ---- Concrete Visitor: Printer --------------------------------------

#[derive(Default)]
pub struct Printer { pub text: String }

impl Visitor for Printer {
    fn visit_num(&mut self, n: &Num) { self.text.push_str(&n.0.to_string()); }
    fn visit_var(&mut self, v: &Var) { self.text.push_str(&v.0); }
    fn visit_add(&mut self, a: &Add) {
        self.text.push('(');
        a.0.accept(self);
        self.text.push_str(" + ");
        a.1.accept(self);
        self.text.push(')');
    }
}

fn main() {
    // (x + (1 + 2))
    let e: Box<dyn Element> = Box::new(Add(
        Box::new(Var("x".into())),
        Box::new(Add(Box::new(Num(1)), Box::new(Num(2)))),
    ));

    let mut env = HashMap::new();
    env.insert("x".to_string(), 10);

    let mut p = Printer::default();
    e.accept(&mut p);
    println!("print: {}", p.text);

    let mut ev = Evaluator::new(&env);
    e.accept(&mut ev);
    println!("eval:  {}", ev.result());
}
