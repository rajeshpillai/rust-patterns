// Prototype in Rust — `#[derive(Clone)]`. That's the pattern.
//
// The "interesting" part of Prototype in Rust is not *how* to clone
// (the compiler derives the method) but *when* to clone vs. move,
// and how to wrap configuration templates so downstream callers
// can't mutate your template accidentally.

#[derive(Clone, Debug)]
pub struct Document {
    pub title: String,
    pub tags: Vec<String>,
    pub body: String,
}

// A Template holds an immutable prototype and spawns fresh Documents.
// Wrapping makes the template's internal state private and ensures
// callers never modify the template — they modify copies.
pub struct Template {
    base: Document,
}

impl Template {
    pub fn new(base: Document) -> Self { Self { base } }

    /// Produce a fresh Document by cloning the template. Callers are
    /// free to mutate the returned value without touching the
    /// template.
    pub fn instance(&self) -> Document {
        self.base.clone()
    }

    /// Sometimes you want to customize at construction — a small
    /// builder-y touch on top of Prototype.
    pub fn instance_with_title(&self, title: impl Into<String>) -> Document {
        let mut d = self.base.clone();
        d.title = title.into();
        d
    }
}

fn main() {
    let template = Template::new(Document {
        title: "Untitled".into(),
        tags: vec!["draft".into()],
        body: "Hello, world".into(),
    });

    let mut d1 = template.instance();
    d1.title = "Project Alpha".into();
    d1.tags.push("work".into());

    let d2 = template.instance_with_title("Meeting Notes");

    println!("{:?}", d1);
    println!("{:?}", d2);
    // The template is untouched — clone gave each call its own copy.
    println!("template still: {:?}", template.instance());
}
