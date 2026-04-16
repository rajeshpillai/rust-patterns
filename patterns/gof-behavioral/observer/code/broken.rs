// Broken example — a tempting attempt to hold observers as raw
// `&dyn Observer` references directly on the Subject.
//
// The borrow checker rejects this: the references need a lifetime,
// and that lifetime must outlive the Subject itself. Callers end up
// fighting lifetimes across every `.attach(...)` call and the pattern
// becomes unusable. This is why every classical Rust port of the GoF
// Observer reaches for `Arc<dyn Observer>` or `Box<dyn Observer>`.
//
// This file is expected to FAIL to compile with E0106 (missing
// lifetime) and friends.

pub trait Observer {
    fn update(&self, payload: &str);
}

pub struct Subject {
    // This field has no lifetime. The compiler needs one — E0106.
    // Adding `<'a>` to Subject and `&'a dyn Observer` only pushes the
    // problem outward: every caller now has to thread `'a` through
    // their storage. That's the smell.
    observers: Vec<&dyn Observer>,
}

impl Subject {
    pub fn new() -> Self { Self { observers: Vec::new() } }
    pub fn attach(&mut self, o: &dyn Observer) {
        self.observers.push(o);
    }
    pub fn notify(&self, payload: &str) {
        for o in &self.observers { o.update(payload); }
    }
}

struct Logger;
impl Observer for Logger {
    fn update(&self, p: &str) { println!("[log] {p}"); }
}

fn main() {
    let mut s = Subject::new();
    let l = Logger;
    s.attach(&l);
    s.notify("hi");
}
