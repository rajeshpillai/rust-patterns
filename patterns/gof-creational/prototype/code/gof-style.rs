// Classical GoF Prototype — a `Prototype` trait with a `clone_box`
// method that returns a trait object. This is what you'd write if
// you ignored Rust's built-in `Clone` trait.
//
// Every line of this is unnecessary: `#[derive(Clone)]` does the
// whole job. See code/idiomatic.rs.

pub trait Prototype {
    fn clone_box(&self) -> Box<dyn Prototype>;
    fn name(&self) -> &str;
}

#[derive(Clone)]
pub struct Document {
    pub title: String,
    pub tags: Vec<String>,
}

impl Prototype for Document {
    // `dyn Clone` is not object-safe (clone returns Self), so GoF-style
    // Prototype in Rust needs this wrapper that returns `Box<dyn Prototype>`
    // explicitly.
    fn clone_box(&self) -> Box<dyn Prototype> {
        Box::new(self.clone())
    }
    fn name(&self) -> &str { &self.title }
}

fn main() {
    let template: Box<dyn Prototype> = Box::new(Document {
        title: "Project Proposal".into(),
        tags: vec!["work".into(), "draft".into()],
    });

    // Duplicate via the Prototype interface.
    let copy = template.clone_box();
    println!("template: {}", template.name());
    println!("copy:     {}", copy.name());
    // And... that's it. We wrote 20 lines to do what .clone() does.
}
