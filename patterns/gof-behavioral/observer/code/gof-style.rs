// Direct GoF translation — Subject holds a list of boxed Observer
// trait objects, calls update() on each notify(). Thread-safety is
// bolted on with Arc<Mutex<_>> because the GoF pattern says nothing
// about ownership and Rust demands you answer that question.
//
// This file compiles and is thread-safe, but it is strictly worse
// than the channel or callback form in code/idiomatic.rs:
//
//   * Four layers of indirection: Arc<Mutex<Vec<Box<dyn ...>>>>.
//   * Every notify() locks the Mutex, even though observers rarely
//     need exclusive access to one another.
//   * Observers cannot easily remove themselves — you end up
//     maintaining hand-rolled integer handles, weak refs, or copying
//     the whole Vec on every mutation.
//   * Running observer code under the Mutex invites deadlocks if an
//     observer re-enters the Subject.
//
// Kept as the contrast.

use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub struct Event {
    pub kind: &'static str,
    pub payload: String,
}

pub trait Observer: Send + Sync {
    fn update(&self, event: &Event);
}

#[derive(Default)]
pub struct Subject {
    observers: Mutex<Vec<Arc<dyn Observer>>>,
}

impl Subject {
    pub fn attach(&self, o: Arc<dyn Observer>) {
        self.observers.lock().unwrap().push(o);
    }

    pub fn notify(&self, event: &Event) {
        // Classic footgun: running observer code inside the Mutex.
        // If an observer calls back into Subject, it deadlocks.
        // The safer variant clones the Vec, drops the lock, then
        // iterates — at the cost of an extra allocation per notify.
        for o in self.observers.lock().unwrap().iter() {
            o.update(event);
        }
    }
}

// ---- concrete observers ------------------------------------------------

struct Logger;
struct Emailer;

impl Observer for Logger {
    fn update(&self, e: &Event) {
        println!("[log] {} — {}", e.kind, e.payload);
    }
}
impl Observer for Emailer {
    fn update(&self, e: &Event) {
        if e.kind == "user.signup" {
            println!("[email] welcoming {}", e.payload);
        }
    }
}

fn main() {
    let subject = Subject::default();
    subject.attach(Arc::new(Logger));
    subject.attach(Arc::new(Emailer));

    subject.notify(&Event { kind: "user.signup",  payload: "rajesh@example.com".into() });
    subject.notify(&Event { kind: "user.deleted", payload: "someone@example.com".into() });
}
