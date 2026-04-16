// Direct GoF translation — Context (Post) holds a Box<dyn State>, and
// each ConcreteState implements the full State trait.
//
// The classical pattern expects `state.handle(self)` to mutate the
// Context, which in Rust runs straight into "cannot move out of a
// borrowed Box". The usual workarounds:
//
//   * `Option<Box<dyn State>>` + `.take()` — the one used below
//   * `Rc<RefCell<Box<dyn State>>>` — heavier, interior mutability
//   * rebuilding the Context on every transition — verbose
//
// Kept here for contrast. See code/idiomatic.rs for the enum form and
// the Typestate pattern (Track B) for the compile-time upgrade.

trait State {
    fn submit(self: Box<Self>)  -> Box<dyn State> { self }
    fn approve(self: Box<Self>) -> Box<dyn State> { self }
    fn reject(self: Box<Self>)  -> Box<dyn State> { self }
    fn name(&self) -> &'static str;
}

struct Draft;
struct Pending;
struct Published;

impl State for Draft {
    fn submit(self: Box<Self>) -> Box<dyn State> { Box::new(Pending) }
    fn name(&self) -> &'static str { "Draft" }
}

impl State for Pending {
    fn approve(self: Box<Self>) -> Box<dyn State> { Box::new(Published) }
    fn reject(self: Box<Self>)  -> Box<dyn State> { Box::new(Draft) }
    fn name(&self) -> &'static str { "Pending" }
}

impl State for Published {
    // .submit() / .approve() / .reject() on Published silently do
    // nothing — the default trait methods return `self` unchanged.
    // This is the runtime trap the Typestate pattern eliminates.
    fn name(&self) -> &'static str { "Published" }
}

struct Post {
    body: String,
    // Option + `.take()` is the trick that lets us move the state out
    // through `&mut self`. Classic GoF translation pain.
    state: Option<Box<dyn State>>,
}

impl Post {
    fn new(body: impl Into<String>) -> Self {
        Self { body: body.into(), state: Some(Box::new(Draft)) }
    }
    fn submit(&mut self) {
        if let Some(s) = self.state.take() { self.state = Some(s.submit()); }
    }
    fn approve(&mut self) {
        if let Some(s) = self.state.take() { self.state = Some(s.approve()); }
    }
    fn reject(&mut self) {
        if let Some(s) = self.state.take() { self.state = Some(s.reject()); }
    }
    fn status(&self) -> &'static str {
        self.state.as_ref().map(|s| s.name()).unwrap_or("UNKNOWN")
    }
}

fn main() {
    let mut post = Post::new("Hello, world");
    println!("{} — {}", post.status(), post.body);   // Draft
    post.approve();                                   // no-op (Draft cannot approve)
    println!("{} — {}", post.status(), post.body);   // still Draft — silent failure
    post.submit();
    post.approve();
    println!("{} — {}", post.status(), post.body);   // Published
}
