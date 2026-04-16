// Broken example — naive port of the GoF pattern that tries to mutate
// the Context's state through `&mut self` without the `Option::take()`
// trick. The borrow checker refuses to let us move out of a `Box`
// that's behind a mutable reference.
//
// This file is expected to FAIL to compile with E0507.

trait State {
    fn submit(self: Box<Self>) -> Box<dyn State>;
    fn name(&self) -> &'static str;
}

struct Draft;
struct Pending;

impl State for Draft {
    fn submit(self: Box<Self>) -> Box<dyn State> { Box::new(Pending) }
    fn name(&self) -> &'static str { "Draft" }
}
impl State for Pending {
    fn submit(self: Box<Self>) -> Box<dyn State> { self }
    fn name(&self) -> &'static str { "Pending" }
}

struct Post {
    body: String,
    state: Box<dyn State>,
}

impl Post {
    fn new(body: &str) -> Self {
        Self { body: body.to_owned(), state: Box::new(Draft) }
    }

    fn submit(&mut self) {
        // The naive translation: call `.submit()` on the current state
        // and assign the result back. This fails to compile:
        //
        //   error[E0507]: cannot move out of `self.state` which is behind
        //                 a mutable reference
        //
        // `self.state` is a `Box<dyn State>` inside a `&mut Post`. The
        // method `submit` on `State` takes `self: Box<Self>`, which
        // means it wants to *own* the Box. Moving it out through a
        // mutable reference is forbidden because it would leave
        // `self.state` uninitialized.
        self.state = self.state.submit();
        //                      ^^^^^^ cannot move out of `self.state`
    }
}

fn main() {
    let mut post = Post::new("hi");
    post.submit();
    println!("{}", post.state.name());
}
