// Broken example — two footguns when mechanically translating Command.
// This file is expected to FAIL to compile.
//
// 1. Storing a `Vec<Box<dyn FnOnce()>>` and trying to call the
//    commands via a shared reference. FnOnce consumes the closure,
//    so calling it requires OWNERSHIP — `&self` is not enough.
//
// 2. Building a `dyn Command` trait with a method that returns Self,
//    which breaks object-safety. Box<dyn Command> won't compile.

pub struct QueueBad {
    work: Vec<Box<dyn FnOnce()>>,
}

impl QueueBad {
    pub fn run(&self) {
        for cmd in &self.work {
            cmd();
            //^^ error[E0507]: cannot move out of `*cmd` which is behind
            //   a shared reference
            //   move occurs because the function's self parameter is
            //   `FnOnce`, which consumes the value
        }
    }
}

// Object-safe trait: returns Self — not allowed behind dyn.
pub trait Command {
    fn execute(&mut self);
    fn clone_command(&self) -> Self;
    //                         ^^^^ error: the trait `Command` cannot be
    //                              made into an object because method
    //                              `clone_command` references `Self`
}

pub fn run_boxed(_c: Box<dyn Command>) { /* won't compile because Command isn't object-safe */ }

fn main() {}
