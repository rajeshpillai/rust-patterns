// Idiomatic Rust — callback list of closures. No trait object, no
// Arc, no Mutex; each registered closure is a plain `Fn(&Event)`.
// `on()` returns a `SubscriptionId` the caller can use to unsubscribe.
//
// Tradeoffs (pick the right form for your concurrency model):
//
//   * This file: synchronous, single-threaded, no async runtime needed.
//     Ideal for in-process event hooks, UI event buses, test spies.
//
//   * Multi-threaded: swap `Vec` for `RwLock<Vec<...>>` and add `Send
//     + Sync` bounds on the closures. See the GoF-style file for the
//     shape, though channel-based alternatives are usually better.
//
//   * Async / multi-subscriber: use `tokio::sync::broadcast` or an
//     async stream. Each subscriber owns its Receiver / Stream so
//     registration, unregistration, and backpressure are explicit.

use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Event {
    pub kind: &'static str,
    pub payload: String,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SubscriptionId(u64);

pub struct EventBus {
    next_id: u64,
    // Key is the SubscriptionId so unsubscribe is O(log n) and order-
    // stable. The closure is boxed once; no per-emit allocation.
    callbacks: HashMap<SubscriptionId, Box<dyn Fn(&Event)>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self { next_id: 0, callbacks: HashMap::new() }
    }

    #[must_use = "drop the SubscriptionId and you lose your ability to unsubscribe"]
    pub fn on<F: Fn(&Event) + 'static>(&mut self, f: F) -> SubscriptionId {
        self.next_id += 1;
        let id = SubscriptionId(self.next_id);
        self.callbacks.insert(id, Box::new(f));
        id
    }

    pub fn off(&mut self, id: SubscriptionId) -> bool {
        self.callbacks.remove(&id).is_some()
    }

    pub fn emit(&self, event: &Event) {
        // Snapshotting the iterator is unnecessary here because `emit`
        // is `&self` and callbacks cannot mutate the bus through it.
        // If you changed `on`/`off` to take `&self` + interior mut,
        // clone the callback Vec before iterating to avoid reentrancy
        // hazards.
        for cb in self.callbacks.values() {
            cb(event);
        }
    }
}

impl Default for EventBus {
    fn default() -> Self { Self::new() }
}

fn main() {
    let mut bus = EventBus::default();

    let _log = bus.on(|e| println!("[log] {} — {}", e.kind, e.payload));
    let email = bus.on(|e| {
        if e.kind == "user.signup" {
            println!("[email] welcoming {}", e.payload);
        }
    });

    bus.emit(&Event { kind: "user.signup",  payload: "rajesh@example.com".into() });
    bus.emit(&Event { kind: "user.deleted", payload: "someone@example.com".into() });

    // Unsubscribe. Next emit produces only the log line.
    assert!(bus.off(email));
    bus.emit(&Event { kind: "user.signup",  payload: "second@example.com".into() });
}
